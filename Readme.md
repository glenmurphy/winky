# winky
Rust module for using the keyboard on Windows:
- Emits keyboard events using scan codes rather than VK codes (required for some games)
- Emits mouse events
- Channel+thread based listener for keyboard events, allowing for hotkeys

# usage
Cargo.toml:
```
[dependencies]
winky = { git = "https://github.com/glenmurphy/winky/" }
```

main.rs:
```
use winky::{self, Key};

#[tokio::main]
async fn main() {
  winky::press(Key::A);
  winky::release(Key::A);

  let mut key_rx = winky::listen();
  loop {
    let (code, down) = key_rx.recv().await.unwrap();
    println!("{:?}", code);
    if code == Key::Q as u32 && down {
      return;
    }
  }
}
```