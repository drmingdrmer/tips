//! A simplified example of a map API without lifetimes.

use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Value type to be stored in a map
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Val(pub u64);

pub trait MapKey: Send + Sync + Clone + Ord + 'static {}

impl MapKey for String {}

#[async_trait::async_trait]
pub trait MapApiRO<K>: Send + Sync {
    type V: Default + Clone + PartialEq + Send + Sync + Unpin + 'static;

    async fn get<Q>(&self, key: &Q) -> Self::V
    where
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized;
}

#[async_trait::async_trait]
pub trait MapApi<K>: MapApiRO<K> + Send + Sync {
    async fn set(&mut self, key: K, value: Self::V) -> (Self::V, Self::V);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct Level {
    pub kv: BTreeMap<String, Val>,
}

#[async_trait::async_trait]
impl MapApiRO<String> for Level {
    type V = Val;

    async fn get<Q>(&self, key: &Q) -> Self::V
    where
        String: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        self.kv.get(key).cloned().unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl MapApi<String> for Level {
    async fn set(&mut self, key: String, value: Self::V) -> (Self::V, Self::V) {
        let prev = self.kv.insert(key.clone(), value.clone());
        (prev.unwrap_or_default(), value)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Writable is a wrapper around a mutable reference to a Level.
#[derive(Debug)]
pub struct Writable<'d> {
    pub writable: &'d mut Level,
}

#[async_trait::async_trait]
impl<'d, K> MapApiRO<K> for Writable<'d>
where
    Level: MapApiRO<K>,
{
    type V = <Level as MapApiRO<K>>::V;

    async fn get<Q>(&self, key: &Q) -> Self::V
    where
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        self.writable.get(key).await
    }
}

#[async_trait::async_trait]
impl<'d, K> MapApi<K> for Writable<'d>
where
    Level: MapApi<K>,
    K: MapKey,
{
    async fn set(&mut self, key: K, value: Self::V) -> (Self::V, Self::V) {
        self.writable.set(key, value).await
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Static(readonly) multi levels of data, which is composed of several [`Level`].
///
/// It only implements [`MapApiRO`].
#[derive(Debug, Default, Clone)]
pub struct StaticLevels {
    pub levels: Vec<Arc<Level>>,
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

#[async_trait::async_trait]
impl<K> MapApiRO<K> for StaticLevels
where
    Level: MapApiRO<K>,
    K: MapKey,
{
    type V = <Level as MapApiRO<K>>::V;

    async fn get<Q>(&self, key: &Q) -> Self::V
    where
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        for level in self.iter_levels() {
            let got = level.get(key).await;
            if got != Self::V::default() {
                return got;
            }
        }
        Self::V::default()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct RefMut<'d> {
    pub writable: &'d mut Level,
    pub frozen: &'d StaticLevels,
}

impl<'d> RefMut<'d> {
    pub fn new(w: &'d mut Level, frozen: &'d StaticLevels) -> Self {
        Self {
            writable: w,
            frozen,
        }
    }

    pub fn iter_levels(&self) -> impl Iterator<Item = &Level> {
        [&*self.writable]
            .into_iter()
            .chain(self.frozen.iter_levels())
    }
}

#[async_trait::async_trait]
impl<'d, K> MapApiRO<K> for RefMut<'d>
where
    Level: MapApiRO<K>,
    K: MapKey,
{
    type V = <Level as MapApiRO<K>>::V;

    async fn get<Q>(&self, key: &Q) -> Self::V
    where
        K: Borrow<Q>,
        Q: Ord + Send + Sync + ?Sized,
    {
        for ld in self.iter_levels() {
            let got = ld.get(key).await;
            if got != Self::V::default() {
                return got;
            }
        }
        Self::V::default()
    }
}

#[async_trait::async_trait]
impl<'d, K> MapApi<K> for RefMut<'d>
where
    Level: MapApi<K>,
    K: MapKey,
{
    async fn set(&mut self, key: K, value: Self::V) -> (Self::V, Self::V) {
        let prev = self.get(&key).await;
        let (_prev, res) = self.writable.set(key, value).await;
        (prev, res)
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
            let prev = mu.set(k(), Val(3)).await;
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

        StaticLevels::new([Arc::new(d1), Arc::new(d2)])
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

        let res = rm.set(k(), Val(5)).await;
        println!("LeveledRefMut::set() res: {:?}", res);

        let got = rm.get(&k()).await;
        println!("LeveledRefMut: {:?}", got);
    }
}
