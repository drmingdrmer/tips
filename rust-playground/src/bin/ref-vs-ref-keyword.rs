#![feature(custom_inner_attributes)]
#![rustfmt::skip]
struct Foo {
    writer: Option<u64>,
}

impl Foo {
    fn ref_keyword_is_ok(&mut self) -> &mut u64 {
        if let Some(ref mut w) = self.writer {   //
            w                                    // Scope of `&mut self.writer` ends here
        } else {                                 //
            self.writer.insert(5)                //
        }                                        //
    }

    fn as_mut_does_not_compile(&mut self) -> &mut u64 {
        if let Some(w) = self.writer.as_mut() {  //
            w                                    //
        } else {                                 //
            self.writer.insert(5)                //
        }                                        // Scope of `&mut self.writer` lasts as long as the if-let statement,
                                                 // and ends here
    }

    fn no_return_value_ok(&mut self) {
        if let Some(w) = self.writer.as_mut() {
            w
        } else {
            self.writer.insert(5)
        };
    }
}

fn main() {}
