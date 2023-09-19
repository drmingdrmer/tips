#![feature(type_alias_impl_trait)]

use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt;
use std::future::Future;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Val(u64);

trait MapApiRO<'d, K>: Send + Sync {
    // type V: Clone + Send + Sync + Unpin + 'static;

    type GetFut<'f, Q>: Future<Output = Val>
    where
        Self: 'f,
        'd: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

trait MapApi<'me, K>: MapApiRO<'me, K> + Send + Sync
where
    K: Ord + Send + Sync + 'static,
{
    type SetFut<'f>: Future<Output = (Val, Val)>
    where
        Self: 'f,
        'me: 'f;

    /// Set an entry and returns the old value and the new value.
    fn set<'f>(self, key: K, value: Option<Val>) -> Self::SetFut<'f>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

impl<'ro_me, 'd, K, T> MapApiRO<'d, K> for &'ro_me mut T
where
    &'ro_me T: MapApiRO<'d, K>,
    K: Ord + Send + Sync + 'static,
    T: Send + Sync,
{
    type GetFut<'f, Q> = <&'ro_me T as MapApiRO<'d, K>>::GetFut<'f, Q>
    where
        Self: 'f,
        'ro_me: 'f,
        'd: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        'ro_me: 'f,
        'd: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        (&*self).get(key)
    }
}

#[derive(Debug, Default)]
pub struct Level {
    kv: BTreeMap<String, Val>,
}

impl<'d> MapApiRO<'d, String> for &'d Level {
    type GetFut<'f, Q> = impl Future<Output =Val> + 'f
    where
        Self: 'f,
        'd: 'f,
        String: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        'd: 'f,
        String: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f,
    {
        async move { self.kv.get(key).unwrap().clone() }
    }
}

impl<'me> MapApi<'me, String> for &'me mut Level {
    type SetFut<'f> = impl Future<Output = (Val, Val)> + 'f
    where
        Self: 'f,
        'me : 'f
    ;

    fn set<'f>(mut self, key: String, value: Option<Val>) -> Self::SetFut<'f>
    where
        'me: 'f,
    {
        async move {
            let prev = self.kv.insert(key.clone(), value.unwrap());
            (prev.unwrap(), self.kv.get(&key).unwrap().clone())
        }
    }
}

#[derive(Debug)]
pub struct LeveledRef<'d> {
    pub writable: &'d Level,
}

impl<'d, K> MapApiRO<'d, K> for LeveledRef<'d>
where
    &'d Level: MapApiRO<'d, K> + Send + Sync,
{
    type GetFut<'f, Q> = impl Future<Output =Val> + 'f
    where
        Self: 'f,
        'd: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        'd: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f,
    {
        async move { self.writable.get(key).await }
    }
}

#[derive(Debug, Default, Clone)]
struct StaticLevels {
    levels: Vec<Arc<Level>>,
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

impl<'d, K> MapApiRO<'d, K> for &'d StaticLevels
where
    K: Ord + Clone + fmt::Debug + Send + Sync + Unpin + 'static,
    for<'e> &'e Level: MapApiRO<'e, K>,
{
    type GetFut<'f,Q> = impl Future<Output=Val> + 'f
    where
        Self: 'f,
        'd: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        'd: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f,
    {
        async move {
            for level_data in self.iter_levels() {
                // let got = <&LevelData as MapApiRO<'_, '_, K>>::get(level_data, key).await;
                let got = level_data.get(key).await;
                if got != Val(0) {
                    return got;
                }
            }
            Val(0)
        }
    }
}

#[derive(Debug)]
pub struct RefMut<'d> {
    pub writable: &'d mut Level,
}

// To reborrow &'d mut writable as &writable, we need &self to outlive 'd.
impl<'me, 'd, K> MapApiRO<'d, K> for &'me RefMut<'d>
where
    for<'e> &'e Level: MapApiRO<'e, K>,
{
    type GetFut<'f, Q> = impl Future<Output =Val> + 'f
    where
        Self: 'f,
        'd: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        'd: 'f,
        'me: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f,
    {
        async move {
            let level_data = &*self.writable;
            let got = level_data.get(key).await;
            got
        }
    }
}

// Error: MapApi<'me, K> must have two lifetime parameter to distinguish between self and the data.
//        Otherwise the lifetime will be extended
//
// error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'d` due to conflicting requirements
// --> src/bin/map-api-test.rs:232:18
// |
// 232 | impl<'me, 'd, K> MapApi<'me, K> for &'me mut LeveledRefMut<'d>
// |                  ^^^^^^^^^^^^^^
// |
// note: first, the lifetime cannot outlive the lifetime `'me` as defined here...
// --> src/bin/map-api-test.rs:232:6
// |
// 232 | impl<'me, 'd, K> MapApi<'me, K> for &'me mut LeveledRefMut<'d>
// |      ^^^
// note: ...so that the types are compatible
// --> src/bin/map-api-test.rs:232:18
// |
// 232 | impl<'me, 'd, K> MapApi<'me, K> for &'me mut LeveledRefMut<'d>
// |                  ^^^^^^^^^^^^^^
// = note: expected `MapApiRO<'me, K>`
// found `MapApiRO<'_, K>`
// note: but, the lifetime must be valid for the lifetime `'d` as defined here...
// --> src/bin/map-api-test.rs:232:11
// |
// 232 | impl<'me, 'd, K> MapApi<'me, K> for &'me mut LeveledRefMut<'d>
// |           ^^
// note: ...so that the types are compatible
// --> src/bin/map-api-test.rs:232:18
// |
// 232 | impl<'me, 'd, K> MapApi<'me, K> for &'me mut LeveledRefMut<'d>
// |                  ^^^^^^^^^^^^^^
// = note: expected `<&LeveledRefMut<'d> as MapApiRO<'_, K>>`
// found `<&LeveledRefMut<'_> as MapApiRO<'_, K>>`

impl<'me, 'd, K> MapApi<'me, K> for &'me mut RefMut<'d>
where
    K: Ord + Clone + Send + Sync + 'static,
    for<'e> &'e Level: MapApiRO<'e, K>,
    for<'him, 'f> &'f mut Level: MapApi<'him, K>,
{
    type SetFut<'f> = impl Future<Output = (Val, Val)> + 'f
    where
        Self: 'f,
        'me: 'f
    ;

    fn set<'f>(mut self, key: K, value: Option<Val>) -> Self::SetFut<'f> {
        async move {
            (Val(1), Val(2))
            // let prev = (&*self).get(&key).await;
            //
            // let (_prev, res) = self.writable.set(key.clone(), value).await;
            // (prev, res)
            // (prev, self.writable.get(&key).await.clone())
        }
    }
}

#[tokio::main]
async fn main() {
    let mut d = Level {
        kv: Default::default(),
    };

    let k = || "a".to_string();

    d.kv.insert(k(), Val(1));

    // &'d LevelData: MapApi::get
    let got = d.get(&k()).await;
    println!("{:?}", got);

    // &mut LevelData :: set()
    {
        let q = d.set(k(), Some(Val(2))).await;
        println!("{:?}", q);

        let q = d.set(k(), Some(Val(3))).await;
        println!("{:?}", q);
    }

    // &mut LevelData :: get()
    {
        let mut_ld = &mut d;
        let got = mut_ld.get(&k()).await;
        println!("{:?}", got);
    }

    // LeveledRef :: get()

    let mut lvl_ref = LeveledRef { writable: &d };

    let got = lvl_ref.get(&k()).await;
    println!("{:?}", got);

    // &StaticLeveledMap: get
    {
        let mut d1 = Level {
            kv: Default::default(),
        };

        let mut d2 = Level {
            kv: Default::default(),
        };

        d1.kv.insert(k(), Val(3));
        d2.kv.insert(k(), Val(2));

        let lvl_map = StaticLevels {
            levels: vec![Arc::new(d1), Arc::new(d2)],
        };

        let got = lvl_map.get(&k()).await;
        println!("StaticLeveledMap: {:?}", got);
    }

    {
        let mut m = RefMut { writable: &mut d };
        let got = m.get(&k()).await;
        println!("{:?}", got);

        // {
        //     let mu = &mut m;
        //     let prev = mu.set(k(), Some(Val(3))).await;
        //     println!("prev: {:?}", prev);
        // }

        {
            let mu = &mut m;
            let got = mu.get(&k()).await;
            println!("{:?}", got);
        }
    }
}
