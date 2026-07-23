mod native_membrane;

mod legacy {
    include!("main.rs");

    pub(super) fn run() -> anyhow::Result<()> {
        main()
    }
}

fn main() -> anyhow::Result<()> {
    if native_membrane::run_env().map_err(anyhow::Error::msg)? {
        Ok(())
    } else {
        legacy::run()
    }
}
