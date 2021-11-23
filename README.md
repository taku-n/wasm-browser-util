# wasm-browser-util

```
# Add wasm-browser-util.
cargo add wasm-browser-util --git https://github.com/taku-n/wasm-browser-util

# Update wasm-browser-util.
cargo update && cargo upgrade
```

## Example

```
cargo new --lib your-crate-name
cd your-crate-name
```

```
[package]
name = "your-crate-name"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["lib", "cdylib"]

[dependencies]
```

```
cargo add wasm-bindgen
cargo add wasm-bindgen-futures
cargo add web-sys --features console
cargo add wasm-browser-util --git https://github.com/taku-n/wasm-browser-util
```

```
use wasm_bindgen::prelude::*;
use web_sys::console;

use wasm_browser_util::run_local;
use wasm_browser_util::sleep;

#[wasm_bindgen(start)]
pub async fn main() {
    console::log_1(&"main() begins.".into());

    let future = run_local::<(), _>(async {
        console::log_1(&"Sleep.".into());
        sleep(6).await;
        console::log_1(&"Awake!".into());
    });
    sleep(3).await;
    console::log_1(&"I'm processing some cool stuff (in my dream)...?".into());
    future.await.unwrap();

    console::log_1(&"main() ends.".into());
}
```

```
<!DOCTYPE html>

<html>

<head>
<meta charset="UTF-8">
</head>

<body>
<script type="module">
import init from './pkg/your_crate_name.js';

async function main() {
  await init();
}

await main();
</script>
</body>

</html>
```

```
wasm-pack build --target web
python -m http.server 8888
```
