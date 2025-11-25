mod os;
mod args;

fn main() {
    let sig = crate::args::parse_args();

    #[cfg(target_os = "windows")]
    {
        crate::os::windows::run(&sig);
        std::process::exit(0);
    }

    #[cfg(target_os = "linux")]
    {
        crate::os::linux::run(&sig);
        std::process::exit(0);
    }

    #[cfg(target_os = "macos")]
    {
        crate::os::macos::run(&sig);
        std::process::exit(0);
    }

    eprintln!("Error: Your platform is not supported.");

    std::process::exit(1);
}

