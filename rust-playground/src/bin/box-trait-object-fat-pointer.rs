use std::any::Any;

trait MyTrait {
    fn echo(&self);
}

struct MyStruct {
    a: u64,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("dropped: {}", self.a)
    }
}

impl MyTrait for MyStruct {
    fn echo(&self) {
        println!("MyStruct: {}", self.a)
    }
}

/// A wrapper of a `ExternalRequestFn` trait object that erases type parameters and make it `Send`.
///
/// This wrapper erases type parameters so that the channel for sending it does not need to contain the type parameters.
/// Only the sending end and the receiving end need to agree on the type parameters.
///
/// Internally, it stores the `ExternalRequestFn` trait object's data pointer in a `Box<dyn Any>`,
/// so that the `Drop::drop()` will be called when the wrapper is dropped.
/// And it stores the vtable pointer in another `usize` to make sure it is `Send`.
struct ExternalRequest {
    /// The data pointer.
    ///
    /// Wrap it in a `Box` to make sure it is dropped when `Boo` is dropped.
    data: Box<dyn Any + Send>,

    /// The vtable pointer.
    ///
    /// Stored in `usize` to make sure it is `Send`.
    vtable: usize,
}

impl ExternalRequest {
    fn new<V: MyTrait + Send + 'static>(v: V) -> Self {
        let raw: *const dyn MyTrait = &v;
        let (_data, vtable): (*const (), *const ()) = unsafe { std::mem::transmute(raw) };
        Self {
            data: Box::new(v),
            vtable: vtable as usize,
        }
    }

    fn unpack(self) -> Box<dyn MyTrait> {
        let raw: *const dyn Any = Box::into_raw(self.data);
        let (data, mut vtable): (*const (), *const ()) = unsafe { std::mem::transmute(raw) };
        vtable = self.vtable as *const ();

        let raw: *mut dyn MyTrait = unsafe { std::mem::transmute((data, vtable)) };

        unsafe { Box::from_raw(raw) }
    }
}

fn assert_send<T: Send>(_v: &T) {}

fn main() {
    {
        let x = MyStruct { a: 5 };
        let b = ExternalRequest::new(x);

        assert_send(&b);

        let y = b.unpack();
        y.echo();
    }

    let x = MyStruct { a: 1 };

    {
        let raw: *const dyn MyTrait = &x;
        let (data_1, vtable_1): (*const (), *const ()) = unsafe { std::mem::transmute(raw) };
        println!("Data pointer: {:?}", data_1);
        println!("Vtable pointer: {:?}", vtable_1);
    }

    {
        let raw: *const dyn Any = &x;
        let (data_1, vtable_1): (*const (), *const ()) = unsafe { std::mem::transmute(raw) };
        println!("Data pointer: {:?}", data_1);
        println!("Vtable pointer: {:?}", vtable_1);
    }

    {
        let b1: Box<dyn Any> = Box::new(MyStruct { a: 4 });

        let raw1: *const dyn Any = Box::into_raw(b1);
        let (data_1, vtable_1): (*const (), *const ()) = unsafe { std::mem::transmute(raw1) };

        println!("Data pointer: {:?}", data_1);
        println!("Vtable pointer: {:?}", vtable_1);

        let _ = unsafe { Box::from_raw(raw1 as *mut dyn Any) };
    }

    let b1: Box<dyn MyTrait> = Box::new(MyStruct { a: 3 });

    let raw1: *const dyn MyTrait = Box::into_raw(b1);
    let (data_1, vtable_1): (*const (), *const ()) = unsafe { std::mem::transmute(raw1) };

    println!("Data pointer: {:?}", data_1);
    println!("Vtable pointer: {:?}", vtable_1);

    let _ = unsafe { Box::from_raw(raw1 as *mut dyn MyTrait) };
}
