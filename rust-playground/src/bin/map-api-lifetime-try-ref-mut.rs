//! A complex example of a map API with lifetimes.

// TODO: define MapKey { type V: MapValue }

#![feature(type_alias_impl_trait)]

use futures_util::stream::BoxStream;
use futures_util::StreamExt;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::future::Future;
use std::ops::RangeBounds;
use std::sync::Arc;
use stream_more::KMerge;

pub fn by_key_seq<K, V>((k1, _v1): &(K, V), (k2, _v2): &(K, V)) -> bool
where
    K: MapKey,
{
    k1 <= k2
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Val(u64);

#[derive(Debug, Default)]
pub struct Level {
    kv: BTreeMap<String, Val>,
}

#[derive(Debug, Default, Clone)]
pub struct StaticLevels {
    levels: Vec<Arc<Level>>,
}

#[derive(Debug)]
pub struct Ref<'d> {
    writable: &'d Level,
    frozen: &'d StaticLevels,
}

#[derive(Debug)]
pub struct RefMut<'d> {
    writable: &'d mut Level,
    frozen: &'d StaticLevels,
}

#[derive(Debug)]
pub struct LevelMap {
    writable: Level,
    frozen: StaticLevels,
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
    pub fn to_ref(&self) -> Ref {
        Ref::new(&*self.writable, &self.frozen)
    }
    pub fn into_ref(self) -> Ref<'d> {
        Ref::new(self.writable, self.frozen)
    }
}

pub trait MapKey: Clone + Ord + Send + Sync + Unpin + 'static {
    type V: Default + Clone + PartialEq + Send + Sync + Unpin + 'static;
}

impl MapKey for String {
    type V = Val;
}

pub trait MapApiRO<'d, K>: Send + Sync
where
    K: MapKey,
{
    type GetFut<'f, Q>: Future<Output = K::V>
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

    type RangeFut<'f, Q, R>: Future<Output = BoxStream<'f, (K, K::V)>>
    where
        // Self: 'f,
        'd: 'f,
        K: Borrow<Q>,
        R: RangeBounds<Q> + Send + Sync + Clone,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;

    fn range<'f, Q, R>(self, range: R) -> Self::RangeFut<'f, Q, R>
    where
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        R: RangeBounds<Q> + Send + Sync + Clone;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait MapApi<'me, 'd, K>: MapApiRO<'d, K>
where
    K: MapKey,
{
    type SetFut<'f>: Future<Output = (K::V, K::V)>
    where
        Self: 'f,
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
    type GetFut<'f, Q> = <&'ro_me T as MapApiRO<'ro_d, K>>::GetFut<'f, Q>
    where
        Self: 'f,
        'ro_me: 'f,
        'ro_d: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        'ro_me: 'f,
        'ro_d: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        (&*self).get(key)
    }

    type RangeFut<'f, Q, R> = <&'ro_me T as MapApiRO<'ro_d, K>>::RangeFut<'f, Q,R>
    where
        // Self: 'f,
        // 'ro_me: 'f,
        'ro_d: 'f,
        K: Borrow<Q>,
        R: RangeBounds<Q> + Send + Sync + Clone,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;

    fn range<'f, Q, R>(self, range: R) -> Self::RangeFut<'f, Q, R>
    where
        // 'ro_me: 'f,
        'ro_d: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        R: RangeBounds<Q> + Clone + Send + Sync,
    {
        (&*self).range(range)
    }
}

impl<'d> MapApiRO<'d, String> for &'d Level {
    type GetFut<'f, Q> = impl Future<Output =<String as MapKey>::V> + 'f
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
        async move { self.kv.get(key).cloned().unwrap_or_default() }
    }

    type RangeFut<'f, Q, R> = impl Future<Output = BoxStream<'f, (String, <String as MapKey>::V)>>
        where
            Self: 'f,
            'd: 'f,
            String: Borrow<Q>,
            R: RangeBounds<Q> + Send + Sync + Clone,
            Q: Ord + Send + Sync + ?Sized,
            Q: 'f;

    fn range<'f, Q, R>(self, range: R) -> Self::RangeFut<'f, Q, R>
    where
        'd: 'f,
        String: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        R: RangeBounds<Q> + Clone + Send + Sync,
    {
        async move {
            let it = self.kv.range(range).map(|(k, v)| (k.clone(), v.clone()));
            futures::stream::iter(it).boxed()
        }
    }
}

// This will fail, with lifetime `'d`
// impl<'me, 'd> MapApi<'me, 'd, String> for &'me mut Level {
//     type SetFut<'f> = impl Future<Output = (<String as MapKey>::V, <String as MapKey>::V)> + 'f
//         where
//             Self: 'f,
//             'me : 'f
//     ;
//
//     fn set<'f>(self, key: String, value: Option<<String as MapKey>::V>) -> Self::SetFut<'f>
//         where
//             'me: 'f,
//     {
//         async move {
//             let prev = self.kv.insert(key.clone(), value.unwrap());
//             (
//                 prev.unwrap_or_default(),
//                 self.kv.get(&key).cloned().unwrap_or_default(),
//             )
//         }
//     }
// }
impl<'me> MapApi<'me, 'me, String> for &'me mut Level {
    type SetFut<'f> = impl Future<Output = (<String as MapKey>::V, <String as MapKey>::V)> + 'f
    where
        Self: 'f,
        'me : 'f
    ;

    fn set<'f>(self, key: String, value: Option<<String as MapKey>::V>) -> Self::SetFut<'f>
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

////////////////////////////// MapApiRO for Writable

///////////////////////// MapApi for Writable

//////////////////////////////

/////////////////////////

// TODO: use LeveledRef
impl<'ro_d, K> MapApiRO<'ro_d, K> for &'ro_d StaticLevels
where
    K: MapKey,
    for<'e> &'e Level: MapApiRO<'e, K>,
{
    type GetFut<'f,Q> = impl Future<Output=K::V> + 'f
        where
            Self: 'f,
            'ro_d: 'f,
            K: Borrow<Q>,
            Q: Ord + Send + Sync + ?Sized,
            Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        'ro_d: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f,
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

    type RangeFut<'f, Q, R> = impl Future<Output = BoxStream<'f, (K, K::V)>>
        where
            Self: 'f,
            'ro_d: 'f,
            K: Borrow<Q>,
            R: RangeBounds<Q> + Send + Sync + Clone,
            Q: Ord + Send + Sync + ?Sized,
            Q: 'f;

    fn range<'f, Q, R>(self, range: R) -> Self::RangeFut<'f, Q, R>
    where
        'ro_d: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        R: RangeBounds<Q> + Clone + Send + Sync,
    {
        async move {
            let mut km = KMerge::by(by_key_seq);

            for api in self.iter_levels() {
                let a = api.range(range.clone()).await;
                km = km.merge(a);
            }

            let x: BoxStream<'_, (K, K::V)> = Box::pin(km);
            x
        }
    }
}

impl<'ro_me, 'ro_d, K> MapApiRO<'ro_d, K> for &'ro_me RefMut<'ro_d>
where
    K: MapKey,
    for<'him> &'him Level: MapApiRO<'him, K>,
{
    type GetFut<'f, Q> = impl Future<Output =K::V> + 'f
    where Self: 'f,
          'ro_me: 'f,
          'ro_d: 'f,
          K: Borrow<Q>,
          Q: Ord + Send + Sync + ?Sized,
          Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        'ro_me: 'f,
        'ro_d: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        async move {
            let level_data = &*self.writable;
            let got = level_data.get(key).await;
            got
        }
    }

    type RangeFut<'f, Q, R> = impl Future<Output = BoxStream<'f, (K, K::V)>>
        where
            // Self: 'f,
            // 'ro_me: 'f,
            'ro_d: 'f,
            K: Borrow<Q>,
            R: RangeBounds<Q> + Send + Sync + Clone,
            Q: Ord + Send + Sync + ?Sized,
            Q: 'f;

    fn range<'f, Q, R>(self, range: R) -> Self::RangeFut<'f, Q, R>
    where
        // 'ro_me: 'f,
        'ro_d: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        R: RangeBounds<Q> + Clone + Send + Sync,
    {
        let levels = self.iter_levels();

        async move {
            let mut km = KMerge::by(by_key_seq);

            for api in levels {
                let a = api.range(range.clone()).await;
                km = km.merge(a);
            }

            let x: BoxStream<'_, (K, K::V)> = Box::pin(km);
            x
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

    fn set<'f>(self, key: K, value: Option<K::V>) -> Self::SetFut<'f>
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

impl<'ro_d, K> MapApiRO<'ro_d, K> for RefMut<'ro_d>
where
    K: MapKey,
    for<'him> &'him Level: MapApiRO<'him, K>,
{
    type GetFut<'f, Q> = impl Future<Output =K::V> + 'f
    where Self: 'f,
          'ro_d: 'f,
          K: Borrow<Q>,
          Q: Ord + Send + Sync + ?Sized,
          Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        'ro_d: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        self.into_ref().get(key)
    }

    type RangeFut<'f, Q, R> = impl Future<Output = BoxStream<'f, (K, K::V)>>
        where
            Self: 'f,
            'ro_d: 'f,
            K: Borrow<Q>,
            R: RangeBounds<Q> + Send + Sync + Clone,
            Q: Ord + Send + Sync + ?Sized,
            Q: 'f;

    fn range<'f, Q, R>(self, range: R) -> Self::RangeFut<'f, Q, R>
    where
        'ro_d: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        R: RangeBounds<Q> + Clone + Send + Sync,
    {
        self.into_ref().range(range)
    }
}

impl<'me, 'd, K> MapApi<'me, 'd, K> for RefMut<'d>
where
    K: MapKey,
    for<'e> &'e Level: MapApiRO<'e, K>,
    for<'him> &'him mut Level: MapApi<'him, 'him, K>,
{
    type SetFut<'f> = impl Future<Output = (K::V, K::V)> + 'f
        where
            Self: 'f,
            'me: 'f;

    fn set<'f>(self, key: K, value: Option<K::V>) -> Self::SetFut<'f>
    where
        'me: 'f,
        'd: 'f,
    {
        async move {
            let prev = (&self).get(&key).await;

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

    // MapApi for ref of Writable

    // &StaticLeveledMap: get
    let static_levels = {
        let mut d1 = Level {
            kv: Default::default(),
        };

        let mut d2 = Level {
            kv: Default::default(),
        };

        d1.kv.insert(k(), Val(3));
        d2.kv.insert(k(), Val(2));

        StaticLevels::new([Arc::new(d1), Arc::new(d2)])
    };
    {
        let got = static_levels.get(&k()).await;
        println!("StaticLeveledMap: {:?}", got);
    }

    // LeveledRefMut :: get()
    {
        let mut d = Level {
            kv: Default::default(),
        };
        let mut rm = RefMut::new(&mut d, &static_levels);
        let got = (&rm).get(&k()).await;
        println!("LeveledRefMut: {:?}", got);

        let res = (&mut rm).set(k(), Some(Val(5))).await;
        println!("LeveledRefMut::set() res: {:?}", res);

        let got = rm.get(&k()).await;
        println!("LeveledRefMut: {:?}", got);
    }
}
