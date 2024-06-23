#![feature(core_intrinsics)] // Needed for `type_id`

use std::any::Any;
macro_rules! get_type {
    ($trait_type:ty, $value:expr) => {{
        let trait_object_ref: &$trait_type = &$value;

        let type_id = std::intrinsics::type_id::<$trait_type>();

        let vtable = {
            let fat_ptr: *const $trait_type = trait_object_ref;
            let (_data, vtable): (*const (), *const ()) = unsafe { std::mem::transmute(fat_ptr) };
            vtable as usize
        };

        (type_id, vtable)
    }};
}

// Example usage
trait ExampleTrait {
    fn example_method(&self);
}

impl ExampleTrait for String {
    fn example_method(&self) {
        // Implementation
    }
}

fn main() {
    {
        let my_string = "Hello, World!".to_string();

        // let (type_id, vtable) = get_type!(dyn ExampleTrait, my_string);
        // println!("TypeId: {:?}", type_id);
        // println!("VTable: {:?}", vtable);

        let t: Box<dyn Any> = Box::new(my_string);
        let u = t.downcast::<dyn ExampleTrait>();
    }
    {
        let my_string = "Hello, World!".to_string();
        let boxed: Box<dyn ExampleTrait> = Box::new(my_string);
        let box2: Box<dyn Any> = Box::new(boxed);
        let _u: Box<Box<dyn ExampleTrait>> = box2.downcast::<Box<dyn ExampleTrait>>().unwrap();
    }
}
