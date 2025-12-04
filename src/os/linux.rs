use std::process::Command;
use std::io::{Read, Write};
use crate::args::Signal;

pub fn run(sig: &Signal) {
    // this line prevents linux users from compiling this program without a proper feature flag.
    // see also Cargo.toml
    #[cfg(not(feature = "dummy_feature"))]
    {
        compile_error!("Error: Please specify a feature flag if you want to build this binary for Linux.")
    }

    imf_specific_control(&sig);
}

// TODO: Need to review the current design of the interfaces for Linux.

/// Results of `fcitx5-remote`, `fcitx5-remote -c`, `fcitx5-remote -o` are heavily dependent on the
/// order of the Input Methods registered in fcitx5 (You can check the list by using
/// `fcitx5-configtool`).
///
/// For now, we are implementing this on the premise that if a English Keyboard is at the beginning of
/// the list, that means the English Keyboard is the "default".
#[cfg(feature = "fcitx5")]
fn imf_specific_control(sig: &Signal) {

    match *sig {
        Signal::ShowHelp => {
            const TARGET_ENV: &str = "Linux(fcitx5)";
            crate::utils::show_help(TARGET_ENV);
        },
        Signal::GetCurrentIMEState => {
            // The result will be 1 or 2.
            // 1 => Currently using the first Input Method in the list.
            // 2 => Currently "NOT" using first Input Method in the list.
            Command::new("fcitx5-remote")
                .spawn();
        },
        Signal::CloseIME => {
            // Switch to 1.
            Command::new("fcitx5-remote")
                .arg("-c")
                .spawn();
        },
        Signal::OpenIME => {
            // Switch back to the "previous" Input Method.
            Command::new("fcitx5-remote")
                .arg("-o")
                .spawn();
        }
        Signal::ToggleIME => {
            Command::new("fcitx5-remote")
                .arg("-t")
                .spawn();
        }
    }
}

/// See also fcitx5 version's doc comments.
#[cfg(feature = "ibus")]
fn imf_specific_control(sig: &Signal) {
    const TEMP_FILE: &str= "ibus_engine_state_for_zhalt.txt";

    // TODO

    match *sig {
        Signal::ShowHelp => {
            const TARGET_ENV: &str = "Linux(ibus)";
            crate::utils::show_help(TARGET_ENV);
        },
        Signal::GetCurrentIMEState => {
            // Show the current engine.
            Command::new("ibus")
                .arg("engine")
                .spawn();
        },
        Signal::CloseIME => {
            // Because it seems ibus engine doesn't have an API to get the "previous" state,
            // we probably have to store the current state in a temp file before we update the
            // state.
            let output = Command::new("ibus")
                .arg("engine")
                .output()
                .unwrap();

            if !output.status.success() {
                eprintln!("Error: Could not execute ibus engine successfully.");
                return;
            }

            let engine_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

            let mut file = std::fs::File::create(&get_temp_path()).unwrap();

            write!(file, "{}", engine_name).unwrap();

            // Then, switch to English Keyboard.
            Command::new("ibus")
                .arg("engine")
                .arg("xkb:us:eng")
                .spawn();
        }
        Signal::OpenIME => {
            let previous_state = std::fs::read_to_string(&get_temp_path());

            let previous_state = match previous_state {
                Ok(ps) => ps,
                Err(_) => {
                    eprintln!("Error: No previous state of ibus engine was found.");
                    return;
                }
            };

            Command::new("ibus")
                .arg("engine")
                .arg(previous_state)
                .spawn();
        }
        Signal::ToggleIME => {
            unimplemented!("Error: Currently zhalt does not have an API to toggle IME in ibus.");
        }
    }
}

#[cfg(any(feature = "fcitx5", feature = "ibus"))]
fn get_temp_path() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();

    #[cfg(feature = "fcitx5")]
    {
        p.push("fcitx5_state_for_zhalt.txt");
        return p;
    }

    #[cfg(feature = "ibus")]
    {
        p.push("ibus_engine_state_for_zhalt.txt");
        return p
    }
}
