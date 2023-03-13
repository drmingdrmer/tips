use std::mem::size_of;
use std::sync::Arc;

trait Trait {}

// size: 8
// Arc<Arc>
// size: 16
// Arc<dyn T>

// size: 8; without tag
enum Blank_Arc {
    Blank,
    Arc(Arc<i32>),
}

// size: 16; without tag
enum Blank_ArcDyn {
    Blank,
    ArcDyn(Arc<dyn Trait>),
}

// size: 16; with tag
enum Arc_Arc {
    Arc1(Arc<i32>),
    Arc2(Arc<i32>),
}

// size: 16; without tag
enum Arc_ArcDyn {
    Arc(Arc<i32>),
    ArcDyn(Arc<dyn Trait>),
}

// size: 24; with tag
enum ArcDyn_ArcDyn {
    ArcDyn1(Arc<dyn Trait>),
    ArcDyn2(Arc<dyn Trait>),
}

// size: 24; with tag
enum Blank_Arc_ArcDyn {
    Blank,
    Arc(Arc<i32>),
    ArcDyn(Arc<dyn Trait>),
}

fn main() {
    println!("Arc<dyn Trait>                       : {}", size_of::<Arc<dyn Trait>>());
    println!("Arc<T>                               : {}", size_of::<Arc<i32>>());

    println!("enum{{Blank, Arc<T>}}                  : {}", size_of::<Blank_Arc>());
    println!("enum{{Blank, Arc<dyn Trait>}}          : {}", size_of::<Blank_ArcDyn>());
    println!("enum{{Arc<T>, Arc<T>}}                 : {}", size_of::<Arc_Arc>());
    println!("enum{{Arc<T>, Arc<dyn Trait>}}         : {}", size_of::<Arc_ArcDyn>());
    println!("enum{{Arc<dyn Trait>, Arc<dyn Trait>}} : {}", size_of::<ArcDyn_ArcDyn>());
    println!("enum{{Blank, Arc<T>, Arc<dyn Trait>}}  : {}", size_of::<Blank_Arc_ArcDyn>());

}
