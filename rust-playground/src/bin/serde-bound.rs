pub trait NodeId: serde::Serialize + for<'a> serde::Deserialize<'a> {}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(bound = "")]
pub enum Fatal<NID>
where
    NID: NodeId,
{
    StorageError(NID),
}

#[derive(serde::Deserialize, serde::Serialize)]
// #[serde(bound = "E: serde::Serialize + for <'d> serde::Deserialize<'d>")]
#[serde(bound(serialize = "E: serde::Serialize"))]
#[serde(bound(deserialize = "E: for <'d> serde::Deserialize<'d>"))]
struct MyError<NID, E>
where
    NID: NodeId,
    E: std::error::Error,
{
    nid: NID,
    err: E,
}

fn main() {}
