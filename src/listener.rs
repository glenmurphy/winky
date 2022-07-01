use winapi::shared::windef::HHOOK;
use winapi::shared::minwindef::{WPARAM, LPARAM, LRESULT, UINT, DWORD};
use winapi::um::winuser;
use winapi::ctypes::c_int;
use winapi::shared::{windef::{HWND, POINT}};

use std::convert::TryFrom;
use std::sync::Mutex;

static mut RESULT_SENDER: Option<Mutex<tokio::sync::mpsc::UnboundedSender<(u32, bool)>>> = None;

pub fn run_hook(tx: tokio::sync::mpsc::UnboundedSender<(u32, bool)>) {
    unsafe {
        RESULT_SENDER = Some(Mutex::new(tx));
    }

    std::thread::spawn(|| {
        let hook = setup_hook();
        message_loop();
        remove_hook(hook);
    });
}

fn setup_hook() -> HHOOK {
    unsafe {
        let hook = winuser::SetWindowsHookExA(winuser::WH_KEYBOARD_LL, Some(callback), std::ptr::null_mut(), 0);
        if hook.is_null() {
            panic!("Windows hook null return");
        }
        hook
    }
}

fn remove_hook(hook: HHOOK) {
    unsafe {
        let result = winuser::UnhookWindowsHookEx(hook);
        if result == 0 {
            panic!("Windows unhook non-zero return");
        }
    }
}

fn message_loop() {
    // This function handles the event loop, which is necessary for the hook to function
    let mut msg = winuser::MSG {
        hwnd : 0 as HWND,
        message : 0 as UINT,
        wParam : 0 as WPARAM,
        lParam : 0 as LPARAM,
        time : 0 as DWORD,
        pt : POINT { x: 0, y: 0, },
    };
    unsafe {
        while 0 == winuser::GetMessageA(&mut msg, std::ptr::null_mut(), 0, 0) {
            winuser::TranslateMessage(&msg);
            winuser::DispatchMessageA(&msg);
        }
    }
}

#[allow(dead_code)]
unsafe extern "system" fn callback(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == winuser::HC_ACTION {
        let tx = RESULT_SENDER.as_ref().unwrap().lock().unwrap();
        match UINT::try_from(w_param).unwrap() {
            winuser::WM_KEYDOWN | winuser::WM_SYSKEYDOWN => {
                let info: winuser::PKBDLLHOOKSTRUCT = std::mem::transmute(l_param);
                tx.send(((*info).scanCode, true)).unwrap();
            },
            winuser::WM_KEYUP | winuser::WM_SYSKEYUP => {
                let info: winuser::PKBDLLHOOKSTRUCT = std::mem::transmute(l_param);
                tx.send(((*info).scanCode, false)).unwrap();
            },
            _ => (),
        }
    }

    winuser::CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param)
}