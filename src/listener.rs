use winapi::shared::minwindef::{LPVOID, WPARAM, LPARAM, LRESULT, DWORD};
use winapi::{
    shared::windef::*,
    um::winuser::*,
};
use winapi::um::libloaderapi::GetModuleHandleW;
use std::ptr;
use std::sync::Mutex;
use crate::{Event, Button};
use tokio::sync::mpsc::UnboundedSender;
use once_cell::sync::Lazy;

static RESULT_SENDER: Lazy<Mutex<Option<UnboundedSender<Event>>>> = Lazy::new(|| Mutex::new(None));

#[macro_export]
/// Convert regular expression to a native string, to be passable as an argument in WinAPI
macro_rules! native_str {
    ($str: expr) => {
        format!("{}\0", $str).as_ptr() as *const u16
    };
}

/// Makes a fake window to be used for our listener
fn create_window() -> HWND {
    unsafe {
        let h_instance = GetModuleHandleW(ptr::null_mut());
        let class_name = native_str!("winky::shadow");
        let win = WNDCLASSW {
            hInstance: h_instance,
            lpfnWndProc: Some(wnd_proc),
            lpszClassName: class_name,
            style: 0,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
        };

        assert!(RegisterClassW(&win) != 0);

        let hwnd = CreateWindowExW(
            WS_EX_CLIENTEDGE,
            class_name,
            class_name,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut());

        assert!(hwnd != ptr::null_mut());

        return hwnd;
    }
}

fn attach(hwnd: HWND) {
    let mouse = RAWINPUTDEVICE {
	    usUsagePage: 1,
	    usUsage: 2,	// Mice
	    dwFlags: RIDEV_INPUTSINK,
	    hwndTarget: hwnd,
    };

    let keyboard = RAWINPUTDEVICE {
	    usUsagePage: 1,
	    usUsage: 6,	// Keyboards
	    dwFlags: RIDEV_INPUTSINK,
	    hwndTarget: hwnd,
    };

    unsafe { 
        RegisterRawInputDevices(vec![mouse, keyboard].as_ptr(), 2, std::mem::size_of::<RAWINPUTDEVICE>() as u32);
    }
}

#[allow(unused)]
fn get_device_name(device: RAWINPUTDEVICELIST) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    unsafe {
        let mut name: [u16; 1024] = [0; 1024];
        let mut name_size: u32 = 1024;

        let bytes = GetRawInputDeviceInfoW(device.hDevice, RIDI_DEVICENAME, name.as_mut_ptr() as LPVOID, &mut name_size);

        let name_slice = &name[0..bytes as usize];
        let full_name = match OsString::from_wide(name_slice).into_string(){
            Ok(something) => something,
            Err(_) => panic!("String Conversion Failed"),
        };

        String::from(full_name)
    }    
}

#[allow(unused)] 
fn get_devices() {
    unsafe {
        let mut buffer: [RAWINPUTDEVICELIST; 1000] = std::mem::zeroed();
        let mut num_devices: u32 = 0;
        let device_list_size = std::mem::size_of::<RAWINPUTDEVICELIST>();

        // Need to call this twice - the first time to fill out num_devices so we can
        // send it back in with the second call.
        GetRawInputDeviceList(ptr::null_mut(), &mut num_devices, device_list_size as u32);
        GetRawInputDeviceList(buffer.as_mut_ptr() as *mut RAWINPUTDEVICELIST, &mut num_devices, device_list_size as u32);

        for pos in 0..num_devices as usize {
            let device_ptr = (&mut buffer[pos..pos+1]).as_mut_ptr() as *const RAWINPUTDEVICELIST;
            let device = *device_ptr;
            println!("{}", get_device_name(device));
        }
    }
}

fn handle_key(raw_input: &RAWINPUT) {
    let tx = RESULT_SENDER.lock().unwrap();
    if let Some(tx) = tx.as_ref() {
        let raw_keyboard_input = unsafe { raw_input.data.keyboard() };
        match raw_keyboard_input.Flags as u32 {
            RI_KEY_MAKE | RI_KEY_BREAK => {
                let down = raw_keyboard_input.Flags == RI_KEY_MAKE as u16;
                match num::FromPrimitive::from_u32(raw_keyboard_input.MakeCode as u32) {
                    Some(code) => tx.send(Event::Keyboard(code, down)).unwrap(),
                    None => { println!("Unknown code: {}", raw_keyboard_input.MakeCode); }
                }
            }
            _ => {}
        }
    }
}

fn handle_mouse(raw_input: &RAWINPUT) {
    let tx = RESULT_SENDER.lock().unwrap();
    if let Some(tx) = tx.as_ref() {
        let raw_mouse_input = unsafe { raw_input.data.mouse() };
        match raw_mouse_input.usButtonFlags as u16 {
            RI_MOUSE_LEFT_BUTTON_DOWN => tx.send(Event::MouseButton(Button::Left, true)).unwrap(),
            RI_MOUSE_LEFT_BUTTON_UP => tx.send(Event::MouseButton(Button::Left, false)).unwrap(),
            RI_MOUSE_RIGHT_BUTTON_DOWN => tx.send(Event::MouseButton(Button::Right, true)).unwrap(),
            RI_MOUSE_RIGHT_BUTTON_UP => tx.send(Event::MouseButton(Button::Right, false)).unwrap(),
            RI_MOUSE_MIDDLE_BUTTON_DOWN => tx.send(Event::MouseButton(Button::Middle, true)).unwrap(),
            RI_MOUSE_MIDDLE_BUTTON_UP => tx.send(Event::MouseButton(Button::Middle, false)).unwrap(),
            RI_MOUSE_BUTTON_4_DOWN => tx.send(Event::MouseButton(Button::X1, true)).unwrap(),
            RI_MOUSE_BUTTON_4_UP => tx.send(Event::MouseButton(Button::X1, false)).unwrap(),
            RI_MOUSE_BUTTON_5_DOWN => tx.send(Event::MouseButton(Button::X2, true)).unwrap(),
            RI_MOUSE_BUTTON_5_UP => tx.send(Event::MouseButton(Button::X2, false)).unwrap(),
            RI_MOUSE_WHEEL => {
                if (raw_mouse_input.usButtonData as i16) > 0 {
                    tx.send(Event::MouseButton(Button::WheelUp, true)).unwrap();
                } else {
                    tx.send(Event::MouseButton(Button::WheelDown, true)).unwrap();
                }
            }
            _ => {  }
        }
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_INPUT => {
            let mut dwsize: u32 = std::mem::size_of::<RAWINPUT>() as u32;
            let mut raw_input: RAWINPUT = std::mem::zeroed();

            GetRawInputData(
                l_param as *mut _,
                RID_INPUT,
                &mut raw_input as *mut _ as *mut winapi::ctypes::c_void,
                &mut dwsize as *mut _,
                std::mem::size_of::<RAWINPUTHEADER>() as u32
            );

            match raw_input.header.dwType {
                RIM_TYPEKEYBOARD => handle_key(&raw_input),
                RIM_TYPEMOUSE => handle_mouse(&raw_input),
                _ => {}
            }
            0
        },
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

fn message_loop(hwnd: HWND) {
    let mut msg = MSG {
        hwnd : hwnd,
        message : 0 as u32,
        wParam : 0 as WPARAM,
        lParam : 0 as LPARAM,
        time : 0 as DWORD,
        pt : POINT { x: 0, y: 0, },
    };
    unsafe {
        while GetMessageW(&mut msg, hwnd as HWND, WM_INPUT, WM_INPUT) == 1 {
            DispatchMessageW(&msg);
        }
        CloseWindow(hwnd);
    }
}

pub fn run_hook(tx: UnboundedSender<Event>) {
    *RESULT_SENDER.lock().unwrap() = Some(tx);

    std::thread::spawn(|| {
        let hwnd = create_window();
        attach(hwnd);
        message_loop(hwnd);   
    });
}