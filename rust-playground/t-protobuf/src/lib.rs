#[allow(clippy::all)]
pub mod protobuf {
    tonic::include_proto!("meta");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("meta_descriptor");
}

#[cfg(test)]
mod tests {

    use crate::protobuf::{Foo, U64List};
    #[test]
    fn test_foo() {
        let x = Foo { foo: 67 };

        let mut buf = vec![];
        prost::Message::encode(&x, &mut buf).unwrap();
        println!("{:?}", buf);
    }
}
