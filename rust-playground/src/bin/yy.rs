#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct KVMeta {
    /// expiration time in second since 1970
    pub(crate) expire_at: u64,
}

fn main() {
    let got: KVMeta = serde_json::from_str(r#"{"expire_at":1,"foo":2}"#).unwrap();
    println!("{:?}", got)
}
