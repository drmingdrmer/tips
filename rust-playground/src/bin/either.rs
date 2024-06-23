use std::any::{Any, TypeId};
use std::mem::ManuallyDrop;
use std::ptr::read_unaligned;

unsafe fn transmute_unchecked<T, U>(t: T) -> U {
    let uninit = ManuallyDrop::new(t);
    read_unaligned(&*uninit as *const _ as *const U)
}

enum Impossible {}

enum OneOf2<T1, T2> {
    V1(T1),
    V2(T2),
}

impl<T1, T2> OneOf2<T1, T2>
where
    T1: Any + 'static,
    T2: Any + 'static,
{
    fn new<T>(t: T) -> Self
    where
        T: Any + 'static,
    {
        let tid = TypeId::of::<T>();

        if tid == TypeId::of::<T1>() {
            Self::V1(unsafe { transmute_unchecked(t) })
        } else if tid == TypeId::of::<T2>() {
            Self::V2(unsafe { transmute_unchecked(t) })
        } else {
            unreachable!()
        }
    }

    fn get<T>(&self) -> Option<&T>
    where
        T: 'static,
    {
        let d = match self {
            Self::V1(l) => l as &dyn Any,
            Self::V2(r) => r as &dyn Any,
        };
        d.downcast_ref::<T>()
    }
}

enum OneOf3<T1, T2, T3> {
    V1(T1),
    V2(T2),
    V3(T3),
}

impl<T1, T2, T3> OneOf3<T1, T2, T3>
where
    T1: Any + 'static,
    T2: Any + 'static,
    T3: Any + 'static,
{
    fn new<T>(t: T) -> Self
    where
        T: Any + 'static,
    {
        let tid = TypeId::of::<T>();

        if tid == TypeId::of::<T1>() {
            Self::V1(unsafe { transmute_unchecked(t) })
        } else if tid == TypeId::of::<T2>() {
            Self::V2(unsafe { transmute_unchecked(t) })
        } else if tid == TypeId::of::<T3>() {
            Self::V3(unsafe { transmute_unchecked(t) })
        } else {
            unreachable!()
        }
    }

    fn get<T>(&self) -> Option<&T>
    where
        T: 'static,
    {
        let d = match self {
            Self::V1(l) => l as &dyn Any,
            Self::V2(r) => r as &dyn Any,
            Self::V3(c) => c as &dyn Any,
        };
        d.downcast_ref::<T>()
    }
}

impl<T1, T2, U3> From<OneOf2<T1, T2>> for OneOf3<T1, T2, U3> {
    fn from(value: OneOf2<T1, T2>) -> Self {
        match value {
            OneOf2::V1(v) => OneOf3::V1(v),
            OneOf2::V2(v) => OneOf3::V2(v),
        }
    }
}

fn main() {
    let e: OneOf3<u8, u16, u32> = OneOf3::new(1u8);
    println!("a: {:?}", e.get::<u8>());
    println!("b: {:?}", e.get::<u16>());
    println!("c: {:?}", e.get::<u32>());

    let e: OneOf3<u8, u16, u32> = OneOf3::new(2u16);
    println!("a: {:?}", e.get::<u8>());
    println!("b: {:?}", e.get::<u16>());
    println!("c: {:?}", e.get::<u32>());

    let e: OneOf3<u8, u16, u32> = OneOf3::new(3u32);
    println!("a: {:?}", e.get::<u8>());
    println!("b: {:?}", e.get::<u16>());
    println!("c: {:?}", e.get::<u32>());

    let o2: OneOf2<u8, u16> = OneOf2::new(1u8);
    let o3 = OneOf3::<u8, u16, u32>::from(o2);
    println!("a: {:?}", o3.get::<u8>());
}

type APIError<E> = OneOf3<RPCError, Fatal, E>;

fn install_snapshot() -> Result<InstallReply, APIError<InstallSnapshotError>> {}
