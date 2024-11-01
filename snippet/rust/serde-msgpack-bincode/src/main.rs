// Run it with:
// cargo test

use std::io;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
enum EnumOld {
    A(String),
    B(u32),
}

// enum that extended with extra variant
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
enum EnumNewAppendField {
    A(String),
    B(u32),
    C(String),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
enum EnumNewInsertFieldInTheMiddle {
    A(String),
    C(String),
    B(u32),
}

#[test]
fn test_bincode_backward_compat_enum() {
    let old_format = EnumOld::B(100);
    let bytes = bincode::serialize(&old_format).unwrap();
    let _: EnumOld = bincode::deserialize(&bytes).unwrap();

    // enum extended with new field is ok
    let new: EnumNewAppendField = bincode::deserialize(&bytes).unwrap();
    assert_eq!(new, EnumNewAppendField::B(100));

    // enum, insert with new field in the middle, is NOT ok
    let new: Result<EnumNewInsertFieldInTheMiddle, _> = bincode::deserialize(&bytes);
    assert!(new.is_err())
}

#[test]
fn test_msgpack_backward_compat_struct() {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
    struct StructOld {
        a: u32,
        b: u32,
    }

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
    struct StructNew {
        a: u32,
        b: u32,
        new_string: Option<String>,
        #[serde(default = "default_new_int")]
        new_int: u32,
    }

    let old_format = StructOld { a: 1, b: 2 };

    // msgpack without schema, NOT backward compatible
    let bytes = rmp_serde::to_vec(&old_format).unwrap();
    let old: Result<StructOld, _> = rmp_serde::from_slice(&bytes);
    assert!(old.is_ok());

    let new: Result<StructNew, _> = rmp_serde::from_slice(&bytes);
    assert!(new.is_err());

    // named messagepack is ok
    let bytes = rmp_serde::to_vec_named(&old_format).unwrap();
    let new: StructNew = rmp_serde::from_slice(&bytes).unwrap();
    assert_eq!(new.a, 1);
    assert_eq!(new.b, 2);
    assert_eq!(new.new_string, None);
    assert_eq!(new.new_int, 100);
}

#[test]
fn test_msgpack_backward_compat_enum() {
    // msgpack with schema
    let old_format = EnumOld::B(100);
    let bytes = rmp_serde::to_vec_named(&old_format).unwrap();
    let _old: EnumOld = rmp_serde::from_slice(&bytes).unwrap();

    // enum extended with new field is ok
    let new: EnumNewAppendField = rmp_serde::from_slice(&bytes).unwrap();
    assert_eq!(new, EnumNewAppendField::B(100));

    // enum, insert with new field in the middle, is ok
    let new: EnumNewInsertFieldInTheMiddle = rmp_serde::from_slice(&bytes).unwrap();
    assert_eq!(new, EnumNewInsertFieldInTheMiddle::B(100));

    // nested struct, backward compat

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
    struct StructOld {
        enum_field: EnumOld,
    }

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
    struct StructNew {
        enum_field: EnumNewInsertFieldInTheMiddle,
        new_string: Option<String>,
        #[serde(default = "default_new_int")]
        new_int: u32,
    }

    // enum backward compat test case: out of order enum evolution

    let olds = StructOld {
        enum_field: EnumOld::B(100),
    };

    let bytes = rmp_serde::to_vec_named(&olds).unwrap();
    let _old: StructOld = rmp_serde::from_slice(&bytes).unwrap();

    let new: StructNew = rmp_serde::from_slice(&bytes).unwrap();

    assert_eq!(new.enum_field, EnumNewInsertFieldInTheMiddle::B(100));
    assert_eq!(new.new_string, None);
    assert_eq!(new.new_int, default_new_int());
}

pub(crate) struct OffsetReader<R> {
    inner: R,
    offset: usize,
}

impl<R: io::Read> OffsetReader<R> {
    pub(crate) fn new(inner: R) -> Self {
        Self { inner, offset: 0 }
    }

    pub(crate) fn offset(&self) -> usize {
        self.offset
    }
}

impl<R: io::Read> io::Read for OffsetReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.offset += n;
        Ok(n)
    }
}


#[test]
fn test_msgpack_writer_reader() {
    // msgpack with schema
    let old_format = EnumOld::B(1);
    let mut buf = Vec::new();
    rmp_serde::encode::write_named(&mut buf, &old_format).unwrap();
    println!("{:?}", buf);

    let old_format = EnumOld::B(2);
    rmp_serde::encode::write_named(&mut buf, &old_format).unwrap();
    println!("{:?}", buf);

    let mut r = OffsetReader::new(&buf[..]);

    // NOTE that unlike serde_json,
    // rmp_serde is able to end when it reads the first valid msgpack object

    let t :EnumOld = rmp_serde::decode::from_read(&mut r).unwrap();
    println!("{:?}, {}", t, r.offset());
    assert_eq!(t, EnumOld::B(1));

    let t :EnumOld = rmp_serde::decode::from_read(&mut r).unwrap();
    println!("{:?}, {}", t, r.offset());
    assert_eq!(t, EnumOld::B(2));

}

fn main() {}

fn default_new_int() -> u32 {
    100
}
