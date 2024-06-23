#[async_trait::async_trait]
pub trait MapApiRO<K>: Send + Sync {
    /// Get an entry by key.
    async fn get(&self, key: &K) -> ();
}

pub trait AsMap {
    // Will be stabilized in 1.75
    fn as_map<K>(&self) -> &impl MapApiRO<K>
    where
        Self: MapApiRO<K> + Sized,
    {
        self
    }
}

fn main() {}
