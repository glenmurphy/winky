# Winky
Rust module for using the keyboard on Windows:
- Emits keyboard events using scan codes rather than VK codes (required for some games)
- Emits mouse events
- Channel+thread based listener for keyboard events, allowing for hotkeys


# Usage
`Cargo.toml:`
```toml
[dependencies]
winky = { git = "https://github.com/glenmurphy/winky/" }
```

`main.rs:`
```rust
use winky::{self, Event, Key};

#[tokio::main]
async fn main() {
  winky::press(Key::A);
  winky::release(Key::A);

  let mut rx = winky::listen();
  loop {
    match rx.recv().await.unwrap() {
      Event::Keyboard(Key::F9, true) => { println!("F9 pressed"); },
      _ => {}
    }
  }
}
```

# examples

`cargo run --example listener`