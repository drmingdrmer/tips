use std::marker::PhantomData;

struct It<'a> {
    i: i32,
    p: PhantomData<&'a ()>,
}

impl Drop for It<'_> {
    fn drop(&mut self) {
        self.i = 2;
        println!("dropped");
    }
}

impl<'a> Iterator for It<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        let p = &self.i as *const i32;
        let x = unsafe { &*p };

        Some(x)
    }
}

fn main() {
    let u = {
        let mut it = It {
            i: 1,
            p: PhantomData,
        };
        it.next()
    };

    println!("{:?}", u);
}
