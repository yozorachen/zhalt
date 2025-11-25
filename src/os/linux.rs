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
            Command::new("fcitx5-remote")
                .arg("-t")
                .spawn();
        }
    }
}

#[cfg(feature = "ibus")]
fn imf_specific_control(sig: &Signal) {
    unimplemented!("ibus is currently unsupported.");
}
