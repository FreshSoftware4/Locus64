mod native_membrane;

mod legacy {
    include!("main.rs");

    pub(super) fn run() -> anyhow::Result<()> {
        main()
    }
}

fn main() -> anyhow::Result<()> {
    if native_membrane::is_legacy_child() {
        native_membrane::announce_explicit_legacy();
        return legacy::run();
    }
    if let Some(code) = native_membrane::run_explicit_legacy().map_err(anyhow::Error::msg)? {
        if code == 0 {
            return Ok(());
        }
        std::process::exit(code);
    }
    if native_membrane::run_env().map_err(anyhow::Error::msg)? {
        Ok(())
    } else {
        native_membrane::warn_ambient_legacy_contact();
        legacy::run()
    }
}
