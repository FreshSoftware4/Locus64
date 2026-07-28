use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, exit},
};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.first().is_some_and(|arg| arg == "authority-audit") {
        print_authority_audit();
        return;
    }
    let target = sibling_binary("l64-cli").unwrap_or_else(|error| fail(&error));
    let status = Command::new(target)
        .args(&args)
        .status()
        .unwrap_or_else(|error| fail(&format!("failed to run l64-cli: {error}")));
    match status.code() {
        Some(code) => exit(code),
        None => fail("l64-cli terminated without an exit code"),
    }
}

fn sibling_binary(name: &str) -> Result<PathBuf, String> {
    let exe = env::current_exe().map_err(|error| error.to_string())?;
    let dir = exe
        .parent()
        .ok_or("current executable has no parent directory")?;
    binary_candidates(dir, name)
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| format!("could not locate sibling binary `{name}`"))
}

fn binary_candidates(dir: &Path, name: &str) -> Vec<PathBuf> {
    binary_candidates_for_host(dir, name, cfg!(windows))
}

fn binary_candidates_for_host(dir: &Path, name: &str, windows: bool) -> Vec<PathBuf> {
    if windows {
        vec![dir.join(name), dir.join(format!("{name}.exe"))]
    } else {
        vec![dir.join(name)]
    }
}

fn print_authority_audit() {
    println!("Locus64 authority audit");
    println!("authority: exact canonical L64R1/L64D");
    println!("transport: ordered L64B members; no composite authority");
    println!("execution: direct non-persistent RNA/DNA structural evaluation");
    println!("derived: projection, certification, observation, change");
    println!("deleted: legacy theorem/campaign/research/registry/tower execution island");
    println!("workspace: dependency-free native carriers");
}

fn fail(message: &str) -> ! {
    eprintln!("{message}");
    exit(2)
}

#[cfg(test)]
mod tests {
    use super::{binary_candidates, binary_candidates_for_host};
    use std::path::Path;

    #[test]
    fn portability_binary_candidates_follow_host_executable_law() {
        let dir = Path::new("portable bin");
        assert_eq!(
            binary_candidates_for_host(dir, "l64-cli", false),
            vec![dir.join("l64-cli")]
        );
        assert_eq!(
            binary_candidates_for_host(dir, "l64-cli", true),
            vec![dir.join("l64-cli"), dir.join("l64-cli.exe")]
        );
        assert_eq!(
            binary_candidates(dir, "l64-cli"),
            binary_candidates_for_host(dir, "l64-cli", cfg!(windows))
        );
    }
}
