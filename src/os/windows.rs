use std::process;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Input::Ime::ImmGetDefaultIMEWnd;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL};
use crate::args::Signal;

const TARGET_ENV: &str = "Windows";

// these values will be used as LPARAM/WPARAM of WM_IME_CONTROL
const IMC_GETOPENSTATUS: usize = 5;
const IMC_SETOPENSTATUS: usize = 6;

const IME_CLOSE: isize = 0;
const IME_OPEN: isize = 1;

pub fn run(sig: &Signal) {
    if let Signal::ShowHelp = sig {
        crate::utils::show_help(TARGET_ENV);
    }

    // get a handler of the foreground window.
    let hwnd = unsafe { GetForegroundWindow() };

    // https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HWND.html
    // if we failed to get the foreground window, or if it's NULL then:
    if hwnd.is_invalid() {
        eprintln!("Error: Failed to get a handler of the foreground window.");
        process::exit(1);
    }

    // get a window handler of an IME that is associated with the window.
    let ime_hwnd = unsafe { ImmGetDefaultIMEWnd(hwnd) };

    if ime_hwnd.is_invalid() {
        eprintln!("Error: Failed to get a window handler of an IME associated with the foreground window.");
        process::exit(1);
    }

    let stat = match sig {
        Signal::GetCurrentIMEState => get_current_ime_state(ime_hwnd),
        Signal::CloseIME => {
            set_ime_state(ime_hwnd, IME_CLOSE);
            IME_CLOSE
        },
        Signal::OpenIME => {
            set_ime_state(ime_hwnd, IME_OPEN);
            IME_OPEN
        },
        Signal::ToggleIME => {
            let current_state = get_current_ime_state(ime_hwnd);
            match current_state {
                0 => {
                    set_ime_state(ime_hwnd, IME_OPEN);
                    IME_OPEN
                }
                1 => {
                    set_ime_state(ime_hwnd, IME_CLOSE);
                    IME_CLOSE
                }
                _ => {
                    set_ime_state(ime_hwnd, IME_CLOSE);
                    IME_CLOSE
                }
            }
        }
    };

    println!("{}", stat); 
}

/// This function obtains the current state of the IME.
///
/// If the return type is:
///
/// - 0: it means your IME is now OFF (CLOSE).
/// - 1: it means your IME is now ON (OPEN).
fn get_current_ime_state(ime_hwnd: HWND) -> isize {
    // https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.SendMessageW.html
    let result = unsafe {
        SendMessageW(
            ime_hwnd,
            WM_IME_CONTROL,
            Some(WPARAM(IMC_GETOPENSTATUS)),
            None, // <- 0 || NULL
        )
    };

    // https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.LRESULT.html
    result.0
}

/// This function will set the IME to a specified state.
///
/// If the state is:
///
/// - LPARAM(0): your IME will be OFF (CLOSE)
/// - LPARAM(1): your IME will be ON (ON)
fn set_ime_state(ime_hwnd: HWND, state: isize) {
    // https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/WindowsAndMessaging/fn.SendMessageW.html
    unsafe {
        SendMessageW(
            ime_hwnd,
            WM_IME_CONTROL,
            Some(WPARAM(IMC_SETOPENSTATUS)),
            Some(LPARAM(state)), // provide the specified state as LPARAM
        )
    };
}
