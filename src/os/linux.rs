use std::process::Command;
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

#[cfg(feature = "fcitx5")]
fn imf_specific_control(sig: &Signal) {
    match *sig {
        Signal::GetCurrentIMEState => {
            Command::new("fcitx5-remote")
                .spawn();
        },
        Signal::CloseIME => {
            Command::new("fcitx5-remote")
                .arg("-c")
                .spawn();
        },
        Signal::OpenIME => {
            Command::new("fcitx5-remote")
                .arg("-o")
                .spawn();
        }
        Signal::ToggleIME => {
            // get the current state.
            let output_result = Command::new("zhalt")
                .output();
            
            match output_result {
                Ok(output) => {
                    let stdout = match str::from_utf8(&output.stdout) {
                        Ok(s) => s.trim(),
                        Err(_) => "?"
                    };

                    // prepare the next state.
                    let arg = match stdout {
                        "0" => "1",
                        _ => "0",
                    };

                    Command::new("zhalt")
                        .arg(arg)
                        .spawn();
                }
                Err(_) => {
                    Command::new("zhalt")
                        .arg("0")
                        .spawn();
                }
            }
        }
    }
}

#[cfg(feature = "ibus")]
fn imf_specific_control(sig: &Signal) {
    unimplemented!("ibus is currently unsupported.");
}
