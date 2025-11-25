pub enum Signal {
    GetCurrentIMEState,
    OpenIME,
    CloseIME,
}

pub fn parse_args() -> Signal {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Signal::GetCurrentIMEState;
    } else {
        match args[1].parse::<isize>() {
            Ok(i) => {
                match i {
                    0 => return Signal::CloseIME,
                    1 => return Signal::OpenIME,
                    x => {
                        eprintln!("Error: Argument: {x} is unsupported.");
                        std::process::exit(1);
                    }
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
    }
}
