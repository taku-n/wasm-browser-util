use wasm_bindgen::prelude::*;
use js_sys::Promise;
use web_sys::console;
use futures::channel::oneshot;
use futures::prelude::*;

use std::time::Duration;

use fluvio_wasm_timer::Delay;

// run_local():
//  Runs an async block concurrently and immediately without await.
//
// Usage:
//  let future = run_local::<return-type-of-async-block, _>(async [move] {});
//
// Example:
//  let future = run_local::<i32, _>(async {2});
//  future.await.unwrap();  // 2 of i32
//
// I'm not sure but I guess the static lifetime is needed
// for the time the spawned thread remains longer than the caller.
pub fn run_local<T, Fut>(future: Fut) -> impl Future<Output = Result<T, oneshot::Canceled>>
        where
                T: 'static,
                Fut: Future<Output = T> + 'static,
{
    let (tx, rx) = oneshot::channel();

    let future_with_tx = async move {
        let result = future.await;

        // unwrap() method cannot be called on `Result<(), T>` due to unsatisfied trait bounds
        // T: Debug
        tx.send(result).unwrap_or(());  // Better way?
    };
    wasm_bindgen_futures::spawn_local(future_with_tx);

    rx
}

async fn test_run_local() {
    console::log_1(&"test_run_local() begins.".into());
    let future = run_local::<i32, _>(async {
        2
    });
    let two = future.await.unwrap();
    console::log_1(&two.into());  //=> 2
    console::log_1(&"test_run_local() ends.".into());
}

pub async fn sleep(s: i32) {
    // It panics when the i32 value is outside of the range of u64.
    let s_u64 = u64::try_from(s).unwrap();

    let d = Duration::from_secs(s_u64);
    Delay::new(d).await.unwrap();
}

#[wasm_bindgen]
pub async fn js_sleep(s: i32) -> Promise {
    sleep(s).await;

    Promise::resolve(&JsValue::NULL)
}

#[wasm_bindgen]
#[allow(unreachable_code)]
pub async fn js_panic() -> Promise {
    panic!();

    Promise::resolve(&JsValue::NULL)
}

#[wasm_bindgen(start)]
pub async fn main() {
    console::log_1(&"main() begins.".into());
    test_run_local().await;
    console::log_1(&"main() ends.".into());
}
