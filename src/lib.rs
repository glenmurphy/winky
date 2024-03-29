// We have to make our own keyboard system because the existing systems use
// SendInput using the vk codes, whereas games need SendInput with keyboard
// scancodes
use std::{io::Error};
use winapi::um::winuser::*;
use num_derive::FromPrimitive;    
mod listener;

// keyboard scan codes from http://www.quadibloc.com/comp/scan.htm
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, FromPrimitive)]
#[allow(unused)]
pub enum Key {
    Esc = 0x01,
    // Function Keys
    F1 = 0x3B,
    F2 = 0x3C,
    F3 = 0x3D,
    F4 = 0x3E,
    F5 = 0x3F,
    F6 = 0x40,
    F7 = 0x41,
    F8 = 0x42,
    F9 = 0x43,
    F10 = 0x44,
    F11 = 0x57,
    F12 = 0x58,
    F13 = 0x5B,
    F14 = 0x5C,
    F15 = 0x5D,
    F16 = 0x63,
    F17 = 0x64,
    F18 = 0x65,
    F19 = 0x66,
    F20 = 0x67,
    F21 = 0x68,
    F22 = 0x69,
    F23 = 0x6A,
    F24 = 0x6B,
    // Top stuff
    Backquote = 0x29,
    Num1 = 0x02,
    Num2 = 0x03,
    Num3 = 0x04,
    Num4 = 0x05,
    Num5 = 0x06,
    Num6 = 0x07,
    Num7 = 0x08,
    Num8 = 0x09,
    Num9 = 0x0A,
    Num0 = 0x0B,
    Minus = 0x0C,
    Equals = 0x0D,
    Backspace = 0x0E,
    Tab = 0x0F,
    CapsLock = 0x3A,
    // Main Letters
    A = 0x1E,
    B = 0x30,
    C = 0x2E,
    D = 0x20,
    E = 0x12,
    F = 0x21,
    G = 0x22,
    H = 0x23,
    I = 0x17,
    J = 0x24,
    K = 0x25,
    L = 0x26,
    M = 0x32,
    N = 0x31,
    O = 0x18,
    P = 0x19,
    Q = 0x10,
    R = 0x13,
    S = 0x1F,
    T = 0x14,
    U = 0x16,
    V = 0x2F,
    W = 0x11,
    X = 0x2D,
    Y = 0x15,
    Z = 0x2C,
    // Main Symbols
    BracketLeft = 0x1A,
    BracketRight = 0x1B,
    BackSlash = 0x2B,
    SemiColon = 0x27,
    Quote = 0x28,
    Enter = 0x1C,
    Comma = 0x33,
    Period = 0x34,
    Slash = 0x35,
    Space = 0x39,
    // Cursor Keys
    Up = 0xE048,
    Left = 0xE04B,
    Right = 0xE04D,
    Down = 0xE050,
    // Edit keys
    PrintScreen = 0x0E37,
    SysRq = 0x54,
    ScrollLock = 0x46,
    Pause = 0x0E45,
    Insert = 0x0E52,
    Delete = 0x0E53,
    Home = 0x0E47,
    End = 0x0E4F,
    PageUp = 0x0E49,
    PageDown = 0x0E51,
    // Numpad
    NumLock = 0x45,
    NumpadDivide = 0x0E35,
    NumpadMultiply = 0x37,
    NumpadMinus = 0x4A,
    NumpadEquals = 0x0E0D,
    NumpadPlus = 0x4E,
    NumpadEnter = 0x0E1C,
    NumpadDot = 0x53,
    Numpad1 = 0x4F,
    Numpad2 = 0x50,
    Numpad3 = 0x51,
    Numpad4 = 0x4B,
    Numpad5 = 0x4C,
    Numpad6 = 0x4D,
    Numpad7 = 0x47,
    Numpad8 = 0x48,
    Numpad9 = 0x49,
    Numpad0 = 0x52,
    NumpadEnd = 0xEE4F,
    NumpadDown = 0xEE50,
    NumpadPageDown = 0xEE51,
    NumpadLeft = 0xEE4B,
    NumpadClear = 0xEE4C,
    NumpadRight = 0xEE4D,
    NumpadHome = 0xEE47,
    NumpadUp = 0xEE48,
    NumpadPageUp = 0xEE49,
    NumpadInsert = 0xEE52,
    NumpadDelete = 0xEE53,
    // Modifier Keys
    Shift = 0x2A,
    ShiftRight = 0x36,
    Control = 0x1D,
    ControlRight = 0x0E1D,
    Alt = 0x38, // AKA Option key
    AltGr = 0x0E38,
    Meta = 0x0E5B, // AKA Windows Key,
    MetaRight = 0x0E5C,
    Menu = 0x0E5D,
    // Media Keys
    Power = 0xE05E,
    Sleep = 0xE05F,
    Wake = 0xE063,
    MediaPlay = 0xE022,
    MediaStop = 0xE024,
    MediaPrev = 0xE010,
    MediaNext = 0xE019,
    MediaSelect = 0xE06D,
    MediaEject = 0xE02C,
    VolMute = 0xE020,
    VolUp = 0xE030,
    VolDown = 0xE02E,
    AppMail = 0xE06C,
    AppCalc = 0xE021,
    AppMusic = 0xE03C,
    AppPhotos = 0xE064,
    BrowserSearch = 0xE065,
    BrowserHome = 0xE032,
    BrowserBack = 0xE06A,
    BrowserForward = 0xE069,
    BrowserStop = 0xE068,
    BrowserRefresh = 0xE067,
    BrowserBookmarks = 0xE066,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[allow(unused)]
pub enum Button {
    None,
    Left,
    Right,
    Middle,
    X1,
    X2,
    WheelUp,
    WheelDown,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[allow(unused)]
pub enum Event {
    MouseButton(Button, bool),
    Keyboard(Key, bool),
}

fn send_key(key: Key, down: bool) -> Result<(), Error> {
    let mut input_u: INPUT_u = unsafe { std::mem::zeroed() };

    let mut flags = KEYEVENTF_SCANCODE;
    if !down { 
        flags |= KEYEVENTF_KEYUP
    }
    if key as u32 > 0xFF {
        flags |= KEYEVENTF_EXTENDEDKEY;
    }

    unsafe {
        *input_u.ki_mut() = KEYBDINPUT {
            wVk: 0,
            dwExtraInfo: 0,
            wScan: key as u16,
            time: 0,
            dwFlags: flags,
        }
    }

    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: input_u,
    };
    let ipsize = std::mem::size_of::<INPUT>() as i32;
    unsafe {
        SendInput(1, &mut input, ipsize);
    };
    Ok(())
}

fn send_mouse(x: i32, y: i32, button: Button, down: bool) -> Result<(), Error> {
    let mut input_u: INPUT_u = unsafe { std::mem::zeroed() };

    let mut mouse_data: i16 = 0;
    let mut flags = match button {
        Button::Left => if down { MOUSEEVENTF_LEFTDOWN } else { MOUSEEVENTF_LEFTUP },
        Button::Right => if down { MOUSEEVENTF_RIGHTDOWN } else { MOUSEEVENTF_RIGHTUP },
        Button::Middle => if down { MOUSEEVENTF_MIDDLEDOWN } else { MOUSEEVENTF_MIDDLEUP },
        Button::WheelUp => { 
            mouse_data = WHEEL_DELTA;
            MOUSEEVENTF_WHEEL
        },
        Button::WheelDown => { 
            mouse_data = -WHEEL_DELTA;
            MOUSEEVENTF_WHEEL
        },
        _ => 0
    };
    if x != 0 || y != 0 {
        flags |= MOUSEEVENTF_MOVE;
    }

    unsafe {
        *input_u.mi_mut() = MOUSEINPUT {
            dx: x as winapi::shared::ntdef::LONG,
            dy: y as winapi::shared::ntdef::LONG,
            mouseData: mouse_data as u32,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0,
        }
    }

    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: input_u,
    };
    let ipsize = std::mem::size_of::<INPUT>() as i32;
    unsafe {
        SendInput(1, &mut input, ipsize);
    };
    Ok(())
}

pub fn press(key: Key) {
    send_key(key, true).unwrap();
}

pub fn release(key: Key) {
    send_key(key, false).unwrap();
}

pub fn mouse_move(x: i32, y: i32) {
    send_mouse(x, y, Button::None, false).unwrap();
}

pub fn button_press(button: Button) {
    send_mouse(0, 0, button, true).unwrap();
}

pub fn button_release(button: Button) {
    send_mouse(0, 0, button, false).unwrap();
}

pub fn listen() -> tokio::sync::mpsc::UnboundedReceiver<crate::Event> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    listener::run_hook(tx);
    rx
}