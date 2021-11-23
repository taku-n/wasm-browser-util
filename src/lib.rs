use wasm_bindgen::prelude::*;
use web_sys::console;
use futures::channel::oneshot;
use futures::prelude::*;

// Usage:
//  let future = run_local::<return-type-of-async-block, _>(async [move] {});
// Example:
//  let future = run_local::<i32, _>(async {2});
//  future.await.unwrap();  // 2 of i32
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

pub async fn test_run_local() {
    console::log_1(&"test_spawn() begins.".into());
    let future = run_local::<i32, _>(async {
        2
    });
    let two = future.await.unwrap();
    console::log_1(&two.into());  //=> 2
    console::log_1(&"test_spawn() ends.".into());
}

#[wasm_bindgen(start)]
pub async fn main() {
    console::log_1(&"main() begins.".into());
    test_run_local().await;
    console::log_1(&"main() ends.".into());
}
