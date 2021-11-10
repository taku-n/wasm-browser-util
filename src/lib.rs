use wasm_bindgen::prelude::*;
use web_sys::console;

use std::cell::Cell;
use std::rc::Rc;
use std::sync::mpsc;

use futures::prelude::*;

pub async fn spawn() -> i32 {
    let (mut tx, mut rx) = futures::channel::mpsc::unbounded();
    let output = Rc::new(Cell::new(0));
    let output_move = output.clone();

    let async_block = async move {
        output_move.set(1);
        console::log_1(&"inside".into());
        console::log_1(&output_move.get().into());
        tx.send(2).await;
        ()
    };
    wasm_bindgen_futures::spawn_local(async_block);

    console::log_1(&"outside".into());
    console::log_1(&output.get().into());
    output.get();
    rx.next().await.unwrap()
}

pub async fn test_spawn() {
    console::log_1(&"test_spawn() begins.".into());
    let output = spawn().await;
    console::log_1(&output.into());
    console::log_1(&"test_spawn() ends.".into());
}

#[wasm_bindgen(start)]
pub async fn main() {
    console::log_1(&"main() begins.".into());
    test_spawn().await;
    console::log_1(&"main() ends.".into());
}
