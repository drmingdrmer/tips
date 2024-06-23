#![feature(type_alias_impl_trait)]

use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::future::Future;
use std::sync::Arc;

pub fn by_key_seq<K, V>((k1, _v1): &(K, V), (k2, _v2): &(K, V)) -> bool
where
    K: MapKey,
{
    k1 <= k2
}

pub fn assert_send<T: Send>(v: T) -> T {
    v
}

pub fn assert_sync<T: Sync>(v: T) -> T {
    v
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Val(u64);

#[derive(Debug, Default)]
pub struct Level {
    pub kv: BTreeMap<String, Val>,
}

#[derive(Debug, Default, Clone)]
pub struct StaticLevels {
    levels: Vec<Arc<Level>>,
}

impl StaticLevels {
    pub fn new(levels: impl IntoIterator<Item = Arc<Level>>) -> Self {
        Self {
            levels: levels.into_iter().collect(),
        }
    }

    pub fn iter_levels(&self) -> impl Iterator<Item = &Level> {
        self.levels.iter().map(|x| x.as_ref()).rev()
    }
}

pub trait MapKey: Clone + Ord + Send + Sync + Unpin + 'static {
    type V: Default + Clone + PartialEq + Send + Sync + Unpin + 'static;
}

impl MapKey for String {
    type V = Val;
}

pub trait MapApiRO<K>: Send + Sync
where
    K: MapKey,
{
    type GetFut<'f, Q>: Future<Output = K::V>
    where
        Self: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f;
}

impl<'d> MapApiRO<String> for &'d Level {
    type GetFut<'f, Q> = impl Future<Output =<String as MapKey>::V> + 'f
        where
            Self: 'f,
            String: Borrow<Q>,
            Q: Ord + Send + Sync + ?Sized,
            Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        Self: 'f,
        String: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f,
    {
        async move { self.kv.get(key).cloned().unwrap_or_default() }
    }
}

impl<'ro_d, K> MapApiRO<K> for &'ro_d StaticLevels
where
    K: MapKey,
    for<'e> &'e Level: MapApiRO<K>,
{
    type GetFut<'f,Q> = impl Future<Output=K::V> + 'f
        where
            Self: 'f,
            K: Borrow<Q>,
            Q: Ord + Send + Sync + ?Sized,
            Q: 'f;

    fn get<'f, Q>(self, key: &'f Q) -> Self::GetFut<'f, Q>
    where
        Self: 'f,
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
        Q: 'f,
    {
        async move {
            for level_data in self.iter_levels() {
                let got = level_data.get(key).await;
                if got != K::V::default() {
                    return got;
                }
            }
            K::V::default()
        }
    }
}

#[tokio::main]
async fn main() {
    let k = || "a".to_string();

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
        assert_eq!(got, Val(2));
    }

    {
        let x = k();
        let fu = static_levels.get(x.as_str());
        let fu = assert_sync(fu);
    }
}
