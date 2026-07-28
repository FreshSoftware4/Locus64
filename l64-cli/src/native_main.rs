mod native_membrane;

fn main() {
    match native_membrane::run_env() {
        Ok(status) => {
            let code = status.code();
            if code != 0 {
                std::process::exit(code);
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}
