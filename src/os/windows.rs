use std::{env, process};
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Input::Ime::ImmGetDefaultIMEWnd;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL};

// these values will be used as LPARAM/WPARAM of WM_IME_CONTROL
const IMC_GETOPENSTATUS: usize = 5;
const IMC_SETOPENSTATUS: usize = 6;

pub fn run() {
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

    let args: Vec<String> = env::args().collect();

    let stat: isize = if args.len() < 2 {
        get_current_ime_state(ime_hwnd)
    } else {
        let specified_state = match args[1].parse::<isize>() {
            Ok(s) => s,
            Err(_) => {
                // failed to convert types.
                eprintln!("Error: Invalid status argument provided.");
                process::exit(1);
            }
        };

        set_ime_state(ime_hwnd, specified_state);

        specified_state
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
