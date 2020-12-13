use std::process;

pub fn exit_on_error(message: &str) -> ! {
        eprintln!("{}", message);
        process::exit(2);
}
