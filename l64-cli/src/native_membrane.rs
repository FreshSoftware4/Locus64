use std::{
    ffi::OsString,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

pub(super) fn run_env() -> Result<bool, String> {
    let args = std::env::args_os().collect::<Vec<_>>();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    dispatch(&args, &mut stdout)
}

pub(crate) fn dispatch(args: &[OsString], stdout: &mut impl Write) -> Result<bool, String> {
    let Some(command) = args.get(1).and_then(|item| item.to_str()) else {
        return Ok(false);
    };
    let Some(file) = args.get(2).map(PathBuf::from) else {
        return Ok(false);
    };

    match command {
        "normalize-rna" => normalize(args, &file, stdout),
        "compile-rna" => compile(args, &file, stdout),
        "sequence-dna" => sequence(args, &file, stdout),
        "verify-roundtrip" => verify(args, &file, stdout),
        _ => Ok(false),
    }
}

fn normalize(args: &[OsString], file: &Path, stdout: &mut impl Write) -> Result<bool, String> {
    let source = read(file)?;
    if !is_native_rna(&source) {
        return Ok(false);
    }
    require_arity(args, 3, "normalize-rna <file>")?;
    let normalized = l64_native::normalize_rna(&source).map_err(native_error)?;
    stdout.write_all(&normalized).map_err(io_error)?;
    Ok(true)
}

fn compile(args: &[OsString], file: &Path, stdout: &mut impl Write) -> Result<bool, String> {
    let source = read(file)?;
    if !is_native_rna(&source) {
        return Ok(false);
    }
    let options = CompileOptions::parse(&args[3..])?;
    let dna = l64_native::rna_to_dna(&source).map_err(native_error)?;
    let out = options.out.unwrap_or_else(|| file.with_extension("dna"));
    fs::write(&out, dna)
        .map_err(|error| format!("failed to write `{}`: {error}", out.display()))?;
    writeln!(stdout, "{}", out.display()).map_err(io_error)?;
    Ok(true)
}

fn sequence(args: &[OsString], file: &Path, stdout: &mut impl Write) -> Result<bool, String> {
    let dna = read(file)?;
    if !dna.starts_with(b"L64D") {
        return Ok(false);
    }
    require_arity(args, 3, "sequence-dna <file>")?;
    let rna = l64_native::dna_to_rna(&dna).map_err(native_error)?;
    stdout.write_all(&rna).map_err(io_error)?;
    Ok(true)
}

fn verify(args: &[OsString], file: &Path, stdout: &mut impl Write) -> Result<bool, String> {
    let source = read(file)?;
    if !is_native_rna(&source) {
        return Ok(false);
    }
    VerifyOptions::parse(&args[3..])?;
    let first_dna = l64_native::rna_to_dna(&source).map_err(native_error)?;
    let canonical_rna = l64_native::dna_to_rna(&first_dna).map_err(native_error)?;
    let second_dna = l64_native::rna_to_dna(&canonical_rna).map_err(native_error)?;
    if first_dna != second_dna {
        return Err("native RNA/DNA fixed point failed".into());
    }
    stdout
        .write_all(b"native RNA/DNA fixed point verified\n")
        .map_err(io_error)?;
    Ok(true)
}

#[derive(Default)]
struct CompileOptions {
    out: Option<PathBuf>,
}

impl CompileOptions {
    fn parse(args: &[OsString]) -> Result<Self, String> {
        let mut options = Self::default();
        let mut index = 0;
        while index < args.len() {
            let argument = args[index].to_string_lossy();
            match argument.as_ref() {
                "--out" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| "--out requires a path".to_string())?;
                    options.out = Some(PathBuf::from(value));
                }
                value if value.starts_with("--out=") => {
                    options.out = Some(PathBuf::from(&value[6..]));
                }
                "--artifact-class" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .and_then(|item| item.to_str())
                        .ok_or_else(|| "--artifact-class requires a value".to_string())?;
                    require_gene(value)?;
                }
                value if value.starts_with("--artifact-class=") => {
                    require_gene(&value[17..])?;
                }
                "--persist-lineage" => {
                    return Err("--persist-lineage belongs to the legacy named-record path".into());
                }
                _ => {
                    return Err(format!(
                        "unsupported native compile-rna option `{argument}`"
                    ));
                }
            }
            index += 1;
        }
        Ok(options)
    }
}

struct VerifyOptions;

impl VerifyOptions {
    fn parse(args: &[OsString]) -> Result<Self, String> {
        let mut index = 0;
        while index < args.len() {
            let argument = args[index].to_string_lossy();
            match argument.as_ref() {
                "--artifact-class" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .and_then(|item| item.to_str())
                        .ok_or_else(|| "--artifact-class requires a value".to_string())?;
                    require_gene(value)?;
                }
                value if value.starts_with("--artifact-class=") => {
                    require_gene(&value[17..])?;
                }
                _ => {
                    return Err(format!(
                        "unsupported native verify-roundtrip option `{argument}`"
                    ));
                }
            }
            index += 1;
        }
        Ok(Self)
    }
}

fn require_gene(value: &str) -> Result<(), String> {
    if value == "gene" {
        Ok(())
    } else {
        Err("native RNA has structural routes, not a legacy artifact class".into())
    }
}

fn require_arity(args: &[OsString], expected: usize, usage: &str) -> Result<(), String> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(format!("usage: l64-cli {usage}"))
    }
}

fn is_native_rna(source: &[u8]) -> bool {
    source
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .is_some_and(|start| source[start..].starts_with(b"L64R1"))
}

fn read(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("failed to read `{}`: {error}", path.display()))
}

fn native_error(error: impl core::fmt::Debug) -> String {
    format!("native authority rejected input: {error:?}")
}

fn io_error(error: std::io::Error) -> String {
    error.to_string()
}
