#![feature(type_name_of_val)]

use futures_util::FutureExt;
use std::future::Future;
use std::pin::Pin;

async fn no_pin<Fu: Future<Output = ()>>(fu: Fu) {
    fu.now_or_never();
}

async fn std_pin<Fu: Future<Output = ()>>(fu: Fu) {
    // Expand to:
    // let pinned = ::core::pin::Pin::<&mut _> {
    //     pointer: &mut { fu },
    // };
    let pinned: Pin<&mut Fu> = std::pin::pin!(fu);

    pinned.await;
}

async fn futures_pin_mut<Fu: Future<Output = ()>>(fu: Fu) {
    // Expand to
    // let mut fu = fu;
    // #[allow(unused_mut)]
    // let mut fu = unsafe { ::pin_utils::core_reexport::pin::Pin::new_unchecked(&mut fu) };
    futures::pin_mut!(fu);

    let pinned: Pin<&mut Fu> = fu;

    pinned.await;
}

async fn futures_pin_mut_loop_value_moved<Fu: Future<Output = ()>>(fu: Fu) {
    loop {
        // Compilation error: value moved: fu
        // futures::pin_mut!(fu);
        break;
    }
    let _ = fu;
}

async fn std_pin_loop_value_moved<Fu: Future<Output = ()>>(fu: Fu) {
    loop {
        // Compilation error: Value moved: fu
        // let pinned = std::pin::pin!(fu);
        break;
    }
    let _ = fu;
}

async fn std_pin_outside_loop<Fu: Future<Output = ()>>(fu: Fu) {
    let mut pinned = std::pin::pin!(fu);
    loop {
        let pinned = pinned.as_mut();
        pinned.await;
        break;
    }
}

fn main() {
    futures::executor::block_on(async move {
        let fu = async { () };
        futures_pin_mut(fu).await;

        let fu = async { () };
        std_pin_loop_value_moved(fu).await;

        let fu = async { () };
        futures_pin_mut_loop_value_moved(fu).await;

        let fu = async { () };
        std_pin_outside_loop(fu).await;
    });
}
