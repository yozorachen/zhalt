mod os;

fn main() {
    #[cfg(target_os = "windows")]
    {
        crate::os::windows::run();
        std::process::exit(0);
    }

    #[cfg(target_os = "linux")]
    {
        crate::os::linux::run();
        std::process::exit(0);
    }

    #[cfg(target_os = "macos")]
    {
        crate::os::macos::run();
        std::process::exit(0);
    }

    eprintln!("Error: Your platform is not supported.");

    std::process::exit(1);
}

