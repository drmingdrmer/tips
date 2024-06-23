// #![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt;
use std::future::Future;
use std::sync::Arc;

#[derive(Debug, Clone, Default, PartialEq)]
struct Val(u64);

#[derive(Debug, Default)]
pub struct Level {
    kv: BTreeMap<String, Val>,
}

#[derive(Debug)]
struct Writable<'d> {
    writable: &'d mut Level,
}

#[derive(Debug, Default, Clone)]
struct StaticLevels {
    levels: Vec<Arc<Level>>,
}

#[derive(Debug)]
pub struct RefMut<'d> {
    writable: &'d mut Level,

    frozen: &'d StaticLevels,
}

impl StaticLevels {
    fn new(levels: impl IntoIterator<Item = Arc<Level>>) -> Self {
        Self {
            levels: levels.into_iter().collect(),
        }
    }

    fn iter_levels(&self) -> impl Iterator<Item = &Level> {
        self.levels.iter().map(|x| x.as_ref()).rev()
    }
}

impl<'d> RefMut<'d> {
    fn new(w: &'d mut Level, frozen: &'d StaticLevels) -> Self {
        Self {
            writable: w,
            frozen,
        }
    }

    fn iter_levels(&self) -> impl Iterator<Item = &Level> {
        [&*self.writable]
            .into_iter()
            .chain(self.frozen.iter_levels())
    }
}

trait MapKey: Clone + Ord + Send + Sync + 'static {
    type V: Default + Clone + PartialEq + Send + Sync + Unpin + 'static;
}

impl MapKey for String {
    type V = Val;
}

trait MapApiRO<'d, K>: Send + Sync
where
    K: MapKey,
{
    type GetFut<Q>: Future<Output = K::V>
    where
        Self: 'd,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'd;

    fn get<Q>(self, key: &'d Q) -> Self::GetFut<Q>
    where
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'd;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

trait MapApi<'me, 'd, K>: MapApiRO<'d, K> + Send + Sync
where
    K: MapKey,
{
    type SetFut<'f>: Future<Output = (K::V, K::V)>
    where
        Self: 'f,
        'd: 'f,
        'me: 'f;

    /// Set an entry and returns the old value and the new value.
    fn set<'f>(self, key: K, value: Option<K::V>) -> Self::SetFut<'f>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

impl<'ro_me, 'ro_d, K, T> MapApiRO<'ro_d, K> for &'ro_me mut T
where
    K: MapKey,
    &'ro_me T: MapApiRO<'ro_d, K>,
    K: Ord + Send + Sync + 'static,
    T: Send + Sync,
{
    type GetFut<Q> = <&'ro_me T as MapApiRO<'ro_d, K>>::GetFut<Q>
        where
            Self: 'ro_d,
            'ro_me: 'ro_d,
            K: Borrow<Q>,
            Q: Ord + Send + Sync + ?Sized,
            Q: 'ro_d;

    fn get<Q>(self, key: &'ro_d Q) -> Self::GetFut<Q>
    where
        'ro_me: 'ro_d,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        (&*self).get(key)
    }
}

impl<'d> MapApiRO<'d, String> for &'d Level {
    type GetFut<Q> = impl Future<Output =<String as MapKey>::V> + 'd
        where
            Self: 'd,
            String: Borrow<Q>,
            Q: Ord + Send + Sync + ?Sized,
            Q: 'd;

    fn get<Q>(self, key: &'d Q) -> Self::GetFut<Q>
    where
        String: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'd,
    {
        async move { self.kv.get(key).cloned().unwrap_or_default() }
    }
}

impl<'me> MapApi<'me, 'me, String> for &'me mut Level {
    type SetFut<'f> = impl Future<Output = (<String as MapKey>::V, <String as MapKey>::V)> + 'f
        where
            Self: 'f,
            'me : 'f
    ;

    fn set<'f>(mut self, key: String, value: Option<<String as MapKey>::V>) -> Self::SetFut<'f>
    where
        'me: 'f,
    {
        async move {
            let prev = self.kv.insert(key.clone(), value.unwrap());
            (
                prev.unwrap_or_default(),
                self.kv.get(&key).cloned().unwrap_or_default(),
            )
        }
    }
}

//////////////////////////////

impl<'ro_me, 'ro_d, K> MapApiRO<'ro_d, K> for &'ro_me Writable<'ro_d>
where
    K: MapKey,
    Self: 'ro_d,
    for<'him> &'him Level: MapApiRO<'him, K>,
{
    type GetFut< Q> = impl Future<Output =K::V> + 'ro_d
        where Self: 'ro_d,
              'ro_me: 'ro_d,
              K: Borrow<Q>,
              Q: Ord + Send + Sync + ?Sized,
              Q: 'ro_d;

    fn get<Q>(self, key: &'ro_d Q) -> Self::GetFut<Q>
    where
        'ro_me: 'ro_d,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        async move {
            let level_data = &*self.writable;
            let got = level_data.get(key).await;
            got
        }
    }
}

/////////////////////////

impl<'me, 'd, K> MapApi<'me, 'd, K> for &'me mut Writable<'d>
where
    K: MapKey,
    for<'e> &'e Level: MapApiRO<'e, K>,
    for<'him> &'him mut Level: MapApi<'him, 'him, K>,
{
    type SetFut<'f> = impl Future<Output = (K::V, K::V)> + 'f
        where
            Self: 'f,
            'me: 'f;

    fn set<'f>(mut self, key: K, value: Option<K::V>) -> Self::SetFut<'f>
    where
        'me: 'f,
        'd: 'f,
    {
        async move {
            let prev = self.get(&key).await;

            let (_prev, res) = self.writable.set(key.clone(), value).await;
            (prev, res)
        }
    }
}

impl<'ro_d, K> MapApiRO<'ro_d, K> for &'ro_d StaticLevels
where
    K: MapKey,
    for<'e> &'e Level: MapApiRO<'e, K>,
{
    type GetFut<Q> = impl Future<Output=K::V> + 'ro_d
        where
            Self: 'ro_d,
            K: Borrow<Q>,
            Q: Ord + Send + Sync + ?Sized,
            Q: 'ro_d;

    fn get<Q>(self, key: &'ro_d Q) -> Self::GetFut<Q>
    where
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'ro_d,
    {
        async move {
            for level_data in self.iter_levels() {
                // let got = <&LevelData as MapApiRO<'_, '_, K>>::get(level_data, key).await;
                let got = level_data.get(key).await;
                if got != K::V::default() {
                    return got;
                }
            }
            K::V::default()
        }
    }
}

impl<'ro_me, 'ro_d, K> MapApiRO<'ro_d, K> for &'ro_me RefMut<'ro_d>
where
    K: MapKey,
    for<'him> &'him Level: MapApiRO<'him, K>,
{
    type GetFut<Q> = impl Future<Output =K::V> + 'ro_d
        where Self: 'ro_d,
              'ro_me: 'ro_d,
              K: Borrow<Q>,
              Q: Ord + Send + Sync + ?Sized,
              Q: 'ro_d;

    fn get<Q>(self, key: &'ro_d Q) -> Self::GetFut<Q>
    where
        'ro_me: 'ro_d,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        async move {
            for ld in self.iter_levels() {
                let got = ld.get(key).await;
                if got != K::V::default() {
                    return got;
                }
            }
            K::V::default()
        }
    }
}

impl<'me, 'd, K> MapApi<'me, 'd, K> for &'me mut RefMut<'d>
where
    K: MapKey,
    for<'e> &'e Level: MapApiRO<'e, K>,
    for<'him> &'him mut Level: MapApi<'him, 'him, K>,
{
    type SetFut<'f> = impl Future<Output = (K::V, K::V)> + 'f
        where
            Self: 'f,
            'me: 'f;

    fn set<'f>(mut self, key: K, value: Option<K::V>) -> Self::SetFut<'f>
    where
        'me: 'f,
        'd: 'f,
    {
        async move {
            let prev = self.get(&key).await;

            let (_prev, res) = self.writable.set(key.clone(), value).await;
            (prev, res)
        }
    }
}

#[tokio::main]
async fn main() {
    let k = || "a".to_string();

    let mut d = Level {
        kv: Default::default(),
    };

    d.kv.insert(k(), Val(1));

    {
        let mut m = Writable { writable: &mut d };
        let got = m.get(&k()).await;
        println!("{:?}", got);

        {
            let mu = &mut m;
            let prev = mu.set(k(), Some(Val(3))).await;
            println!("prev: {:?}", prev);
        }

        {
            let mu = &mut m;
            let got = mu.get(&k()).await;
            println!("{:?}", got);
        }
    }

    // &StaticLeveledMap: get
    let lvl_map = {
        let mut d1 = Level {
            kv: Default::default(),
        };

        let mut d2 = Level {
            kv: Default::default(),
        };

        d1.kv.insert(k(), Val(3));
        d2.kv.insert(k(), Val(2));

        StaticLevels {
            levels: vec![Arc::new(d1), Arc::new(d2)],
        }
    };
    {
        let got = lvl_map.get(&k()).await;
        println!("StaticLeveledMap: {:?}", got);
    }

    // LeveledRefMut :: get()
    {
        let mut d = Level {
            kv: Default::default(),
        };
        let mut rm = RefMut::new(&mut d, &lvl_map);
        let got = rm.get(&k()).await;
        println!("LeveledRefMut: {:?}", got);

        let res = rm.set(k(), Some(Val(5))).await;
        println!("LeveledRefMut::set() res: {:?}", res);

        let got = rm.get(&k()).await;
        println!("LeveledRefMut: {:?}", got);
    }
}
