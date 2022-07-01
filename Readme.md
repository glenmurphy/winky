# winky
Rust module for using the keyboard on Windows:
- Emits keyboard events using scan codes rather than VK codes (required for some games)
- Emits mouse events
- Channel+thread based listener for keyboard events, allowing for hotkeys

# usage
```
use winky::{self, Key};

fn main() {
  winky::press(Key::A);
  winky::release(Key::A);

  let mut key_rx = winky::listen();
  loop {
    let (code, down) = key_rx.recv().unwrap();
    println!("{:?}", code);
    if key == Key::Q as u32 && down {
      return;
    }
  }
}
```