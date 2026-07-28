use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{BufReader, ErrorKind, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

const RETIRED_LEGACY_COMMANDS: &[&str] = &[
    "legacy",
    "certify",
    "certify-derived",
    "run-theorem",
    "compile-atlas",
    "canonize",
    "exec-host",
    "dump-canonical",
    "select-route",
    "derive-frontier",
    "tower-step",
    "dispatch-coverage",
    "derive-distress",
    "research-import",
    "research-export",
    "research-list",
    "research-route",
    "research-derive-from-report",
    "research-govern-report",
    "research-seed-export-remediation",
    "research-remediation-summary",
    "research-status",
    "research-promotion-readiness",
    "replay-report",
    "cache-stats",
    "clear-cache",
    "explain-invalidation",
    "export-report-dna",
    "import-report-dna",
    "export-validation-dna-bundle",
    "import-bundle",
    "dump-bundle-graph",
    "dump-overlay-world",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum ProcessStatus {
    Success = 0,
    Open = 10,
    Incomplete = 11,
    Invalid = 12,
}

impl ProcessStatus {
    pub const fn code(self) -> i32 {
        self as i32
    }

    const fn from_verdict(verdict: l64_certification::CertificationVerdict) -> Self {
        match verdict {
            l64_certification::CertificationVerdict::Certified => Self::Success,
            l64_certification::CertificationVerdict::Open => Self::Open,
            l64_certification::CertificationVerdict::Incomplete => Self::Incomplete,
            l64_certification::CertificationVerdict::Invalid => Self::Invalid,
        }
    }

    const fn severity(self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Open => 1,
            Self::Incomplete => 2,
            Self::Invalid => 3,
        }
    }

    fn merge(self, other: Self) -> Self {
        if other.severity() > self.severity() {
            other
        } else {
            self
        }
    }
}

pub(super) fn run_env() -> Result<ProcessStatus, String> {
    let args = std::env::args_os().collect::<Vec<_>>();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    dispatch(&args, &mut stdout)
}

pub(crate) fn dispatch(
    args: &[OsString],
    stdout: &mut impl Write,
) -> Result<ProcessStatus, String> {
    let Some(command) = args.get(1).and_then(|item| item.to_str()) else {
        print_help(stdout)?;
        return Ok(ProcessStatus::Success);
    };
    if matches!(command, "-V" | "--version" | "version") {
        require_arity(args, 2, "--version")?;
        writeln!(stdout, "l64-cli {}", env!("CARGO_PKG_VERSION")).map_err(io_error)?;
        return Ok(ProcessStatus::Success);
    }
    if command == "help" {
        match args.get(2).and_then(|item| item.to_str()) {
            None => print_help(stdout)?,
            Some(topic) if args.len() == 3 => print_command_help(topic, stdout)?,
            Some(_) => return Err("usage: l64-cli help [command]".into()),
        }
        return Ok(ProcessStatus::Success);
    }
    if matches!(command, "-h" | "--help") {
        require_arity(args, 2, "--help")?;
        print_help(stdout)?;
        return Ok(ProcessStatus::Success);
    }
    if args.len() == 3
        && args[2]
            .to_str()
            .is_some_and(|value| matches!(value, "-h" | "--help"))
    {
        print_command_help(command, stdout)?;
        return Ok(ProcessStatus::Success);
    }
    if RETIRED_LEGACY_COMMANDS.contains(&command) {
        return Err(format!(
            "legacy authority command `{command}` was permanently deleted after historical export; use current RNA/DNA, release, transport, certification, observation, or change carriers"
        ));
    }

    match command {
        "compile-bundle" => compile_bundle(args, stdout).map(|()| ProcessStatus::Success),
        "run-bundle" => run_bundle(args, stdout),
        "certify-bundle" => certify_bundle(args, stdout),
        "observe-bundle" => observe_bundle(args, stdout),
        "compare-bundle" => compare_bundle(args, stdout).map(|()| ProcessStatus::Success),
        "normalize-rna" | "compile-rna" | "sequence-dna" | "inspect-dna" | "verify-roundtrip"
        | "run-rna" | "run-dna" | "certify-dna" | "observe-dna" | "compare-dna" => {
            let file = args
                .get(2)
                .map(PathBuf::from)
                .ok_or_else(|| format!("usage: l64-cli {command} <file>"))?;
            match command {
                "normalize-rna" => normalize(args, &file, stdout).map(|()| ProcessStatus::Success),
                "compile-rna" => compile(args, &file, stdout).map(|()| ProcessStatus::Success),
                "sequence-dna" => sequence(args, &file, stdout).map(|()| ProcessStatus::Success),
                "inspect-dna" => inspect(args, &file, stdout).map(|()| ProcessStatus::Success),
                "verify-roundtrip" => verify(args, &file, stdout).map(|()| ProcessStatus::Success),
                "run-rna" => run_rna(args, &file, stdout),
                "run-dna" => run_dna(args, &file, stdout),
                "certify-dna" => certify_dna(args, &file, stdout),
                "observe-dna" => observe_dna(args, &file, stdout),
                "compare-dna" => compare_dna(args, &file, stdout).map(|()| ProcessStatus::Success),
                _ => unreachable!(),
            }
        }
        "export-genome-release" => export_release(args, stdout).map(|()| ProcessStatus::Success),
        _ => Err(format!(
            "unknown native command `{command}`; run `l64-cli --help`"
        )),
    }
}

fn print_help(stdout: &mut impl Write) -> Result<(), String> {
    writeln!(
        stdout,
        "l64-cli {} — dependency-free native Locus64",
        env!("CARGO_PKG_VERSION")
    )
    .map_err(io_error)?;
    writeln!(stdout, "usage: l64-cli <command> [arguments]").map_err(io_error)?;
    writeln!(stdout).map_err(io_error)?;
    writeln!(stdout, "AUTHORING").map_err(io_error)?;
    writeln!(
        stdout,
        "  normalize-rna        write canonical L64R1 RNA to stdout"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  compile-rna          compile L64R1 RNA into a new L64D authority file"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  sequence-dna         reconstruct canonical L64R1 RNA from L64D"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  verify-roundtrip     verify the exact RNA/DNA fixed point"
    )
    .map_err(io_error)?;
    writeln!(stdout).map_err(io_error)?;
    writeln!(stdout, "EXECUTION AND DERIVATION").map_err(io_error)?;
    writeln!(
        stdout,
        "  run-rna              execute structural authority from RNA"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  run-dna              execute structural authority from DNA"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  inspect-dna          render the verified root projection"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  certify-dna          certify every native context"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  observe-dna          render certification, replay, and report views"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  compare-dna          compare two exact DNA authorities"
    )
    .map_err(io_error)?;
    writeln!(stdout).map_err(io_error)?;
    writeln!(stdout, "TRANSPORT AND RELEASE").map_err(io_error)?;
    writeln!(
        stdout,
        "  compile-bundle       create a new ordered L64B transport"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  run-bundle           execute each ordered member independently"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  certify-bundle       certify each ordered member independently"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  observe-bundle       observe each ordered member independently"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  compare-bundle       compare members by transport index"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  export-genome-release  atomically export one native release"
    )
    .map_err(io_error)?;
    writeln!(stdout).map_err(io_error)?;
    writeln!(stdout, "Run `l64-cli help <command>` for exact usage.").map_err(io_error)?;
    writeln!(stdout).map_err(io_error)?;
    writeln!(stdout, "PROCESS STATUS").map_err(io_error)?;
    writeln!(
        stdout,
        "  0   command succeeded; certification is CERTIFIED when applicable"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  10  command succeeded with OPEN certification burdens"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  11  command succeeded with INCOMPLETE certification evidence"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  12  command succeeded with INVALID certification burdens"
    )
    .map_err(io_error)?;
    writeln!(
        stdout,
        "  2   usage, input, filesystem, or processing failure"
    )
    .map_err(io_error)
}

fn print_command_help(command: &str, stdout: &mut impl Write) -> Result<(), String> {
    let (usage, summary) = match command {
        "normalize-rna" => (
            "normalize-rna <file.rna>",
            "write canonical L64R1 RNA to stdout",
        ),
        "compile-rna" => (
            "compile-rna <file.rna> [--out <file.dna>]",
            "compile into a new canonical L64D authority; existing outputs are never overwritten",
        ),
        "sequence-dna" => (
            "sequence-dna <file.dna>",
            "write canonical reconstructable L64R1 RNA to stdout",
        ),
        "inspect-dna" => (
            "inspect-dna <file.dna>",
            "derive and verify the root projection",
        ),
        "verify-roundtrip" => (
            "verify-roundtrip <file.rna>",
            "verify RNA -> DNA -> RNA -> DNA exact equality",
        ),
        "run-rna" => (
            "run-rna <file.rna>",
            "compile transiently and evaluate structural authority",
        ),
        "run-dna" => (
            "run-dna <file.dna>",
            "validate and evaluate canonical DNA authority",
        ),
        "export-genome-release" => (
            "export-genome-release --rna <file.rna> --out <directory>",
            "atomically export source, authority, projection, and release record",
        ),
        "compile-bundle" => (
            "compile-bundle <member.dna>... --out <bundle.l64b>",
            "create a new ordered transport; existing outputs are never overwritten",
        ),
        "run-bundle" => (
            "run-bundle --file <bundle.l64b>",
            "execute every member independently in transport order",
        ),
        "certify-dna" => ("certify-dna <file.dna>", "certify all native contexts"),
        "certify-bundle" => (
            "certify-bundle --file <bundle.l64b>",
            "certify every member independently",
        ),
        "observe-dna" => (
            "observe-dna <file.dna>",
            "render verified certification, replay, and report projections",
        ),
        "observe-bundle" => (
            "observe-bundle --file <bundle.l64b>",
            "observe every member independently",
        ),
        "compare-dna" => (
            "compare-dna <before.dna> <after.dna>",
            "compare exact authorities and derived structural movement",
        ),
        "compare-bundle" => (
            "compare-bundle <before.l64b> <after.l64b>",
            "compare ordered member contacts by transport index",
        ),
        "authority-audit" => {
            return Err(
                "`authority-audit` belongs to the `l64` wrapper: run `l64 authority-audit`".into(),
            );
        }
        _ => {
            return Err(format!(
                "unknown native command `{command}`; run `l64-cli --help`"
            ));
        }
    };
    writeln!(stdout, "usage: l64-cli {usage}").map_err(io_error)?;
    writeln!(stdout, "{summary}").map_err(io_error)
}

fn compile_bundle(args: &[OsString], stdout: &mut impl Write) -> Result<(), String> {
    let options = BundleCompileOptions::parse(&args[2..])?;
    write_new_with(&options.out, |file| {
        let mut encoder = l64_transport::BundleEncoder::new(file)
            .map_err(|error| format!("failed to begin native bundle output: {error}"))?;
        for path in &options.members {
            let dna = read_dna(path)?;
            require_magic(path, &dna, b"L64D", "current L64D authority")?;
            encoder.push_member(&dna).map_err(|error| {
                format!(
                    "native bundle member `{}` rejected: {error}",
                    path.display()
                )
            })?;
        }
        encoder
            .finish()
            .map_err(|error| format!("failed to finish native bundle output: {error}"))?;
        Ok(())
    })?;
    writeln!(stdout, "{}", options.out.display()).map_err(io_error)?;
    Ok(())
}

fn run_bundle(args: &[OsString], stdout: &mut impl Write) -> Result<ProcessStatus, String> {
    let options = BundleFileOptions::parse(&args[2..], "run-bundle")?;
    let mut decoder = open_bundle(&options.file)?;
    writeln!(
        stdout,
        "L64 NATIVE BUNDLE EXECUTION v{}",
        l64_transport::BUNDLE_VERSION
    )
    .map_err(io_error)?;
    writeln!(stdout, "members={}", decoder.member_count()).map_err(io_error)?;
    writeln!(stdout, "composite_authority=none").map_err(io_error)?;
    writeln!(stdout, "composite_execution=none").map_err(io_error)?;
    writeln!(stdout, "ordering=transport_sequence").map_err(io_error)?;
    let mut status = ProcessStatus::Success;
    while let Some(member) = decoder
        .next_member()
        .map_err(|error| format!("native bundle execution rejected input: {error}"))?
    {
        let execution = l64_execution::execute_canonical_graph(
            member.graph(),
            l64_execution::ExecutionSource::BundleMember,
        )
        .map_err(|error| {
            format!(
                "native bundle member {} execution failed: {error}",
                member.index()
            )
        })?;
        let mut rendered = String::new();
        execution.render_into(&mut rendered, &format!("member.{}.", member.index()));
        stdout.write_all(rendered.as_bytes()).map_err(io_error)?;
        status = status.merge(ProcessStatus::from_verdict(execution.certification.verdict));
    }
    Ok(status)
}

fn certify_bundle(args: &[OsString], stdout: &mut impl Write) -> Result<ProcessStatus, String> {
    let options = BundleFileOptions::parse(&args[2..], "certify-bundle")?;
    let mut decoder = open_bundle(&options.file)?;
    writeln!(stdout, "L64 NATIVE BUNDLE CERTIFICATION v1").map_err(io_error)?;
    writeln!(stdout, "transport=canonical_l64b").map_err(io_error)?;
    writeln!(stdout, "transport_verified=true").map_err(io_error)?;
    writeln!(stdout, "members={}", decoder.member_count()).map_err(io_error)?;
    writeln!(stdout, "composite_authority=none").map_err(io_error)?;
    writeln!(stdout, "composite_certificate=none").map_err(io_error)?;
    let mut status = ProcessStatus::Success;
    while let Some(member) = decoder
        .next_member()
        .map_err(|error| format!("native bundle certification rejected input: {error}"))?
    {
        let certification = l64_certification::certify_graph(member.graph()).map_err(|error| {
            format!(
                "native bundle member {} certification failed: {error}",
                member.index()
            )
        })?;
        let mut rendered = String::new();
        certification.render_into(&mut rendered, &format!("member.{}.", member.index()));
        stdout.write_all(rendered.as_bytes()).map_err(io_error)?;
        status = status.merge(ProcessStatus::from_verdict(certification.verdict));
    }
    Ok(status)
}

fn observe_bundle(args: &[OsString], stdout: &mut impl Write) -> Result<ProcessStatus, String> {
    let options = BundleFileOptions::parse(&args[2..], "observe-bundle")?;
    let mut decoder = open_bundle(&options.file)?;
    writeln!(
        stdout,
        "L64 NATIVE BUNDLE OBSERVATION v{}",
        l64_observation::OBSERVATION_VERSION
    )
    .map_err(io_error)?;
    writeln!(stdout, "transport=canonical_l64b").map_err(io_error)?;
    writeln!(stdout, "transport_verified=true").map_err(io_error)?;
    writeln!(stdout, "members={}", decoder.member_count()).map_err(io_error)?;
    writeln!(stdout, "composite_authority=none").map_err(io_error)?;
    writeln!(stdout, "composite_verdict=none").map_err(io_error)?;
    writeln!(stdout, "composite_observation=none").map_err(io_error)?;
    let mut status = ProcessStatus::Success;
    while let Some(member) = decoder
        .next_member()
        .map_err(|error| format!("native bundle observation rejected input: {error}"))?
    {
        let observation =
            l64_observation::observe_canonical_graph(member.graph()).map_err(|error| {
                format!(
                    "native bundle member {} observation failed: {error}",
                    member.index()
                )
            })?;
        let mut rendered = String::new();
        observation.render_into(&mut rendered, &format!("member.{}.", member.index()));
        stdout.write_all(rendered.as_bytes()).map_err(io_error)?;
        status = status.merge(ProcessStatus::from_verdict(
            observation.certification.verdict,
        ));
    }
    Ok(status)
}

fn compare_bundle(args: &[OsString], stdout: &mut impl Write) -> Result<(), String> {
    require_arity(args, 4, "compare-bundle <before.l64b> <after.l64b>")?;
    let before_path = PathBuf::from(&args[2]);
    let after_path = PathBuf::from(&args[3]);
    let mut before = open_bundle(&before_path)?;
    let mut after = open_bundle(&after_path)?;
    let before_members = before.member_count();
    let after_members = after.member_count();
    writeln!(
        stdout,
        "L64 NATIVE BUNDLE CHANGE v{}",
        l64_change::CHANGE_VERSION
    )
    .map_err(io_error)?;
    writeln!(stdout, "transport=canonical_l64b").map_err(io_error)?;
    writeln!(stdout, "ordering=transport_sequence").map_err(io_error)?;
    writeln!(stdout, "before_members={before_members}").map_err(io_error)?;
    writeln!(stdout, "after_members={after_members}").map_err(io_error)?;
    writeln!(stdout, "composite_authority=none").map_err(io_error)?;
    writeln!(stdout, "composite_change_verdict=none").map_err(io_error)?;

    for index in 0..before_members.max(after_members) {
        let before_member = if index < before_members {
            before
                .next_member()
                .map_err(|error| format!("before bundle rejected input: {error}"))?
        } else {
            None
        };
        let after_member = if index < after_members {
            after
                .next_member()
                .map_err(|error| format!("after bundle rejected input: {error}"))?
        } else {
            None
        };
        let prefix = format!("member.{index}.");
        match (before_member, after_member) {
            (Some(left), Some(right)) => {
                let change = l64_change::compare_dna(left.dna(), right.dna()).map_err(|error| {
                    format!("native bundle member {index} comparison failed: {error}")
                })?;
                writeln!(
                    stdout,
                    "{prefix}contact={}",
                    if change.exact_equal {
                        "UNCHANGED"
                    } else {
                        "CHANGED"
                    }
                )
                .map_err(io_error)?;
                let mut rendered = String::new();
                change.render_into(&mut rendered, &prefix);
                stdout.write_all(rendered.as_bytes()).map_err(io_error)?;
            }
            (Some(_), None) => writeln!(stdout, "{prefix}contact=REMOVED").map_err(io_error)?,
            (None, Some(_)) => writeln!(stdout, "{prefix}contact=ADDED").map_err(io_error)?,
            (None, None) => unreachable!(),
        }
    }
    before
        .finish()
        .map_err(|error| format!("before bundle rejected input: {error}"))?;
    after
        .finish()
        .map_err(|error| format!("after bundle rejected input: {error}"))?;
    Ok(())
}

fn compare_dna(
    args: &[OsString],
    before_path: &Path,
    stdout: &mut impl Write,
) -> Result<(), String> {
    require_arity(args, 4, "compare-dna <before.dna> <after.dna>")?;
    let before = read(before_path)?;
    require_magic(before_path, &before, b"L64D", "current L64D authority")?;
    let after_path = PathBuf::from(&args[3]);
    let after = read(&after_path)?;
    require_magic(&after_path, &after, b"L64D", "current L64D authority")?;
    let change = l64_change::compare_dna(&before, &after)
        .map_err(|error| format!("native DNA comparison rejected input: {error}"))?;
    stdout
        .write_all(change.render_text().as_bytes())
        .map_err(io_error)?;
    Ok(())
}

fn export_release(args: &[OsString], stdout: &mut impl Write) -> Result<(), String> {
    let options = ReleaseOptions::parse(&args[2..])?;
    let source = read(&options.rna)?;
    require_rna(&options.rna, &source)?;
    let release = l64_release::export_native_release(&source, &options.out).map_err(|error| {
        if let Some(rna) = error.rna_error() {
            rna_input_error(&options.rna, &source, "native release rejected RNA", rna)
        } else {
            format!("native release rejected input: {error}")
        }
    })?;
    writeln!(stdout, "{}", release.root.display()).map_err(io_error)?;
    Ok(())
}

fn run_rna(
    args: &[OsString],
    file: &Path,
    stdout: &mut impl Write,
) -> Result<ProcessStatus, String> {
    let source = read(file)?;
    require_rna(file, &source)?;
    require_arity(args, 3, "run-rna <file>")?;
    let execution = l64_execution::execute_rna(&source).map_err(|error| {
        if let Some(rna) = error.rna_error() {
            rna_input_error(file, &source, "native RNA execution rejected input", rna)
        } else {
            format!("native RNA execution rejected input: {error}")
        }
    })?;
    stdout
        .write_all(execution.render_text().as_bytes())
        .map_err(io_error)?;
    Ok(ProcessStatus::from_verdict(execution.certification.verdict))
}

fn run_dna(
    args: &[OsString],
    file: &Path,
    stdout: &mut impl Write,
) -> Result<ProcessStatus, String> {
    let dna = read(file)?;
    require_magic(file, &dna, b"L64D", "current L64D authority")?;
    require_arity(args, 3, "run-dna <file>")?;
    let execution = l64_execution::execute_dna(&dna)
        .map_err(|error| format!("native DNA execution rejected input: {error}"))?;
    stdout
        .write_all(execution.render_text().as_bytes())
        .map_err(io_error)?;
    Ok(ProcessStatus::from_verdict(execution.certification.verdict))
}

fn normalize(args: &[OsString], file: &Path, stdout: &mut impl Write) -> Result<(), String> {
    let source = read(file)?;
    require_rna(file, &source)?;
    require_arity(args, 3, "normalize-rna <file>")?;
    let normalized = l64_native::normalize_rna(&source).map_err(|error| {
        rna_input_error(
            file,
            &source,
            "native RNA normalization rejected input",
            error,
        )
    })?;
    stdout.write_all(&normalized).map_err(io_error)?;
    Ok(())
}

fn compile(args: &[OsString], file: &Path, stdout: &mut impl Write) -> Result<(), String> {
    let source = read(file)?;
    require_rna(file, &source)?;
    let options = CompileOptions::parse(&args[3..])?;
    let dna = l64_native::rna_to_dna(&source).map_err(|error| {
        rna_input_error(
            file,
            &source,
            "native RNA compilation rejected input",
            error,
        )
    })?;
    let out = options.out.unwrap_or_else(|| file.with_extension("dna"));
    write_new(&out, &dna)?;
    writeln!(stdout, "{}", out.display()).map_err(io_error)?;
    Ok(())
}

fn sequence(args: &[OsString], file: &Path, stdout: &mut impl Write) -> Result<(), String> {
    let dna = read(file)?;
    require_magic(file, &dna, b"L64D", "current L64D authority")?;
    require_arity(args, 3, "sequence-dna <file>")?;
    let rna = l64_native::dna_to_rna(&dna).map_err(native_error)?;
    stdout.write_all(&rna).map_err(io_error)?;
    Ok(())
}

fn inspect(args: &[OsString], file: &Path, stdout: &mut impl Write) -> Result<(), String> {
    let dna = read(file)?;
    require_magic(file, &dna, b"L64D", "current L64D authority")?;
    require_arity(args, 3, "inspect-dna <file>")?;
    let graph = l64_native::decode_dna(&dna).map_err(native_error)?;
    let projection = l64_projection::ProjectionSet::derive(&graph, l64_native::ROOT_CONTEXT, 16)
        .map_err(native_error)?;
    projection.verify(&graph).map_err(native_error)?;
    stdout
        .write_all(projection.render_text().as_bytes())
        .map_err(io_error)?;
    Ok(())
}

fn certify_dna(
    args: &[OsString],
    file: &Path,
    stdout: &mut impl Write,
) -> Result<ProcessStatus, String> {
    let dna = read(file)?;
    require_magic(file, &dna, b"L64D", "current L64D authority")?;
    require_arity(args, 3, "certify-dna <file>")?;
    let certification = l64_certification::certify_dna(&dna)
        .map_err(|error| format!("native DNA certification rejected input: {error}"))?;
    stdout
        .write_all(certification.render_text().as_bytes())
        .map_err(io_error)?;
    Ok(ProcessStatus::from_verdict(certification.verdict))
}

fn observe_dna(
    args: &[OsString],
    file: &Path,
    stdout: &mut impl Write,
) -> Result<ProcessStatus, String> {
    let dna = read(file)?;
    require_magic(file, &dna, b"L64D", "current L64D authority")?;
    require_arity(args, 3, "observe-dna <file>")?;
    let observation = l64_observation::observe_dna(&dna)
        .map_err(|error| format!("native DNA observation rejected input: {error}"))?;
    stdout
        .write_all(observation.render_text().as_bytes())
        .map_err(io_error)?;
    Ok(ProcessStatus::from_verdict(
        observation.certification.verdict,
    ))
}

fn verify(args: &[OsString], file: &Path, stdout: &mut impl Write) -> Result<(), String> {
    let source = read(file)?;
    require_rna(file, &source)?;
    require_arity(args, 3, "verify-roundtrip <file>")?;
    let first_dna = l64_native::rna_to_dna(&source)
        .map_err(|error| rna_input_error(file, &source, "native roundtrip rejected RNA", error))?;
    let canonical_rna = l64_native::dna_to_rna(&first_dna).map_err(native_error)?;
    let second_dna = l64_native::rna_to_dna(&canonical_rna).map_err(|error| {
        rna_input_error(
            file,
            &canonical_rna,
            "native roundtrip resequencing failed",
            error,
        )
    })?;
    if first_dna != second_dna {
        return Err("native RNA/DNA fixed point failed".into());
    }
    stdout
        .write_all(b"native RNA/DNA fixed point verified\n")
        .map_err(io_error)?;
    Ok(())
}

struct BundleCompileOptions {
    members: Vec<PathBuf>,
    out: PathBuf,
}

impl BundleCompileOptions {
    fn parse(args: &[OsString]) -> Result<Self, String> {
        let mut members = Vec::new();
        let mut out = None;
        let mut index = 0;
        while index < args.len() {
            let argument = args[index].to_string_lossy();
            match argument.as_ref() {
                "--out" => {
                    index += 1;
                    out = Some(PathBuf::from(
                        args.get(index).ok_or("--out requires a path")?,
                    ));
                }
                value if value.starts_with("--out=") => {
                    out = Some(PathBuf::from(&value[6..]));
                }
                value if value.starts_with('-') => {
                    return Err(format!(
                        "unsupported native compile-bundle option `{value}`"
                    ));
                }
                _ => members.push(PathBuf::from(&args[index])),
            }
            index += 1;
        }
        if members.is_empty() {
            return Err("usage: l64-cli compile-bundle <member.dna>... --out <bundle.l64b>".into());
        }
        Ok(Self {
            members,
            out: out.ok_or("usage: l64-cli compile-bundle <member.dna>... --out <bundle.l64b>")?,
        })
    }
}

struct BundleFileOptions {
    file: PathBuf,
}

impl BundleFileOptions {
    fn parse(args: &[OsString], command: &str) -> Result<Self, String> {
        let mut file = None;
        let mut index = 0;
        while index < args.len() {
            let argument = args[index].to_string_lossy();
            match argument.as_ref() {
                "--file" => {
                    index += 1;
                    file = Some(PathBuf::from(
                        args.get(index).ok_or("--file requires a path")?,
                    ));
                }
                value if value.starts_with("--file=") => {
                    file = Some(PathBuf::from(&value[7..]));
                }
                _ => {
                    return Err(format!(
                        "unsupported native {command} option `{argument}`; native contact accepts only --file"
                    ));
                }
            }
            index += 1;
        }
        Ok(Self {
            file: file.ok_or_else(|| format!("usage: l64-cli {command} --file <bundle.l64b>"))?,
        })
    }
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
                    return Err("--artifact-class was permanently deleted; native RNA is structural authority without a legacy class selector".into());
                }
                value if value.starts_with("--artifact-class=") => {
                    return Err("--artifact-class was permanently deleted; native RNA is structural authority without a legacy class selector".into());
                }
                "--persist-lineage" => {
                    return Err(
                        "--persist-lineage was permanently deleted after historical export".into(),
                    );
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

struct ReleaseOptions {
    rna: PathBuf,
    out: PathBuf,
}

impl ReleaseOptions {
    fn parse(args: &[OsString]) -> Result<Self, String> {
        let mut rna = None;
        let mut out = None;
        let mut index = 0;
        while index < args.len() {
            let argument = args[index].to_string_lossy();
            match argument.as_ref() {
                "--rna" => {
                    index += 1;
                    rna = Some(PathBuf::from(
                        args.get(index).ok_or("--rna requires a path")?,
                    ));
                }
                value if value.starts_with("--rna=") => rna = Some(PathBuf::from(&value[6..])),
                "--out" => {
                    index += 1;
                    out = Some(PathBuf::from(
                        args.get(index).ok_or("--out requires a path")?,
                    ));
                }
                value if value.starts_with("--out=") => out = Some(PathBuf::from(&value[6..])),
                "--artifact-class" => {
                    return Err("--artifact-class was permanently deleted; native RNA is structural authority without a legacy class selector".into());
                }
                value if value.starts_with("--artifact-class=") => {
                    return Err("--artifact-class was permanently deleted; native RNA is structural authority without a legacy class selector".into());
                }
                _ => {
                    return Err(format!(
                        "unsupported native export-genome-release option `{argument}`"
                    ));
                }
            }
            index += 1;
        }
        Ok(Self {
            rna: rna.ok_or("usage: l64-cli export-genome-release --rna <file> --out <dir>")?,
            out: out.ok_or("usage: l64-cli export-genome-release --rna <file> --out <dir>")?,
        })
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

static STAGE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn require_rna(path: &Path, source: &[u8]) -> Result<(), String> {
    if is_native_rna(source) {
        Ok(())
    } else {
        Err(format!("`{}` is not current L64R1 RNA", path.display()))
    }
}

fn require_magic(path: &Path, bytes: &[u8], magic: &[u8], description: &str) -> Result<(), String> {
    if bytes.starts_with(magic) {
        Ok(())
    } else {
        Err(format!("`{}` is not {description}", path.display()))
    }
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    write_new_with(path, |file| {
        file.write_all(bytes).map_err(|error| {
            format!(
                "failed to write staging output `{}`: {error}",
                path.display()
            )
        })
    })
}

fn write_new_with(
    path: &Path,
    write: impl FnOnce(&mut File) -> Result<(), String>,
) -> Result<(), String> {
    if path.exists() {
        return Err(format!(
            "refusing to overwrite existing output `{}`",
            path.display()
        ));
    }
    let parent = path
        .parent()
        .filter(|value| !value.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)
        .map_err(|error| format!("failed to create `{}`: {error}", parent.display()))?;
    let name = path
        .file_name()
        .ok_or_else(|| format!("invalid output path `{}`", path.display()))?;
    let stage_id = STAGE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let stage = parent.join(format!(
        ".{}.l64-stage-{}-{stage_id}",
        name.to_string_lossy(),
        std::process::id()
    ));
    let result = (|| {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&stage)
            .map_err(|error| {
                format!(
                    "failed to create staging file `{}`: {error}",
                    stage.display()
                )
            })?;
        write(&mut file)?;
        file.sync_all().map_err(|error| {
            format!("failed to sync staging file `{}`: {error}", stage.display())
        })?;
        fs::hard_link(&stage, path).map_err(|error| {
            if error.kind() == ErrorKind::AlreadyExists {
                format!("refusing to overwrite existing output `{}`", path.display())
            } else {
                format!("failed to promote new output `{}`: {error}", path.display())
            }
        })?;
        Ok(())
    })();
    let _ = fs::remove_file(&stage);
    result
}

fn open_bundle(path: &Path) -> Result<l64_transport::BundleDecoder<BufReader<File>>, String> {
    let file = File::open(path)
        .map_err(|error| format!("failed to open `{}`: {error}", path.display()))?;
    l64_transport::BundleDecoder::new(BufReader::new(file)).map_err(|error| {
        format!(
            "`{}` is not current L64B transport: {error}",
            path.display()
        )
    })
}

fn read_dna(path: &Path) -> Result<Vec<u8>, String> {
    const MAX_DNA_FILE_BYTES: u64 = l64_native::MAX_NATIVE_DNA_PAYLOAD_BYTES as u64 + 44;
    let metadata = fs::metadata(path)
        .map_err(|error| format!("failed to inspect `{}`: {error}", path.display()))?;
    if metadata.len() > MAX_DNA_FILE_BYTES {
        return Err(format!(
            "`{}` exceeds the native L64D file-size limit",
            path.display()
        ));
    }
    read(path)
}

fn read(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("failed to read `{}`: {error}", path.display()))
}

fn rna_input_error(
    path: &Path,
    source: &[u8],
    prefix: &str,
    error: l64_native::RnaError,
) -> String {
    format!(
        "{prefix} in `{}`:\n{}",
        path.display(),
        l64_native::rna_diagnostic(source, error)
    )
}

fn native_error(error: impl core::fmt::Display) -> String {
    format!("native authority rejected input: {error}")
}

fn io_error(error: std::io::Error) -> String {
    error.to_string()
}

#[cfg(test)]
#[path = "golden_tests.rs"]
mod golden_tests;

#[cfg(test)]
mod tests {
    use super::{ProcessStatus, dispatch};
    use std::{ffi::OsString, fs};

    fn args(items: &[&str]) -> Vec<OsString> {
        items.iter().map(OsString::from).collect()
    }

    #[test]
    fn help_exposes_only_current_native_commands() {
        let mut output = Vec::new();
        dispatch(&args(&["l64-cli", "--help"]), &mut output).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("compile-rna"));
        assert!(text.contains("run-rna"));
        assert!(text.contains("run-dna"));
        assert!(text.contains("certify-dna"));
        assert!(!text.contains("run-theorem"));
        assert!(!text.contains("research-import"));
    }

    #[test]
    fn run_rna_and_run_dna_share_the_same_execution_symbol() {
        let root = std::env::temp_dir().join(format!("l64-cli-run-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir(&root).unwrap();
        let rna = root.join("sample.rna");
        let dna = root.join("sample.dna");
        fs::write(
            &rna,
            b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n",
        )
        .unwrap();

        let mut rna_output = Vec::new();
        dispatch(
            &[
                OsString::from("l64-cli"),
                OsString::from("run-rna"),
                rna.clone().into_os_string(),
            ],
            &mut rna_output,
        )
        .unwrap();

        dispatch(
            &[
                OsString::from("l64-cli"),
                OsString::from("compile-rna"),
                rna.into_os_string(),
                OsString::from("--out"),
                dna.clone().into_os_string(),
            ],
            &mut Vec::new(),
        )
        .unwrap();

        let mut dna_output = Vec::new();
        dispatch(
            &[
                OsString::from("l64-cli"),
                OsString::from("run-dna"),
                dna.into_os_string(),
            ],
            &mut dna_output,
        )
        .unwrap();

        let rna_text = String::from_utf8(rna_output).unwrap();
        let dna_text = String::from_utf8(dna_output).unwrap();
        let rna_symbol = rna_text
            .lines()
            .find(|line| line.starts_with("symbol="))
            .unwrap();
        let dna_symbol = dna_text
            .lines()
            .find(|line| line.starts_with("symbol="))
            .unwrap();
        assert_eq!(rna_symbol, dna_symbol);
        assert!(rna_text.contains("source=rna"));
        assert!(dna_text.contains("source=dna"));
        assert!(rna_text.contains("authority_mutation=none"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn retired_authority_island_is_a_permanent_tombstone() {
        for command in ["legacy", "run-theorem", "research-status", "tower-step"] {
            let error = dispatch(&args(&["l64-cli", command]), &mut Vec::new()).unwrap_err();
            assert!(error.contains("permanently deleted after historical export"));
        }
    }

    #[test]
    fn command_help_and_version_are_direct() {
        let mut version = Vec::new();
        dispatch(&args(&["l64-cli", "--version"]), &mut version).unwrap();
        assert_eq!(
            String::from_utf8(version).unwrap(),
            format!("l64-cli {}\n", env!("CARGO_PKG_VERSION"))
        );

        let mut help = Vec::new();
        dispatch(&args(&["l64-cli", "help", "run-rna"]), &mut help).unwrap();
        let text = String::from_utf8(help).unwrap();
        assert!(text.contains("usage: l64-cli run-rna <file.rna>"));
        assert!(text.contains("compile transiently"));
    }

    #[test]
    fn wrong_native_format_fails_at_the_requested_contact() {
        let root = std::env::temp_dir().join(format!("l64-cli-format-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir(&root).unwrap();
        let file = root.join("not-rna.txt");
        fs::write(&file, b"not native authority\n").unwrap();
        let error = dispatch(
            &[
                OsString::from("l64-cli"),
                OsString::from("run-rna"),
                file.into_os_string(),
            ],
            &mut Vec::new(),
        )
        .unwrap_err();
        assert!(error.contains("is not current L64R1 RNA"));
        assert!(!error.contains("legacy theorem"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn compile_rna_refuses_overwrite_and_preserves_existing_bytes() {
        let root = std::env::temp_dir().join(format!("l64-cli-output-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir(&root).unwrap();
        let rna = root.join("sample.rna");
        let dna = root.join("sample.dna");
        fs::write(
            &rna,
            b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n",
        )
        .unwrap();
        fs::write(&dna, b"sentinel").unwrap();
        let error = dispatch(
            &[
                OsString::from("l64-cli"),
                OsString::from("compile-rna"),
                rna.into_os_string(),
                OsString::from("--out"),
                dna.clone().into_os_string(),
            ],
            &mut Vec::new(),
        )
        .unwrap_err();
        assert!(error.contains("refusing to overwrite"));
        assert_eq!(fs::read(&dna).unwrap(), b"sentinel");
        assert!(fs::read_dir(&root).unwrap().all(|entry| {
            !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .contains(".l64-stage-")
        }));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn dead_artifact_class_option_is_not_silently_accepted() {
        let root = std::env::temp_dir().join(format!("l64-cli-class-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir(&root).unwrap();
        let rna = root.join("sample.rna");
        fs::write(&rna, b"L64R1 0x4c36344e41544956\na 1 0x41\n").unwrap();
        let error = dispatch(
            &[
                OsString::from("l64-cli"),
                OsString::from("compile-rna"),
                rna.into_os_string(),
                OsString::from("--artifact-class"),
                OsString::from("gene"),
            ],
            &mut Vec::new(),
        )
        .unwrap_err();
        assert!(error.contains("--artifact-class was permanently deleted"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn verdicts_map_to_stable_process_statuses() {
        assert_eq!(
            ProcessStatus::from_verdict(l64_certification::CertificationVerdict::Certified).code(),
            0
        );
        assert_eq!(
            ProcessStatus::from_verdict(l64_certification::CertificationVerdict::Open).code(),
            10
        );
        assert_eq!(
            ProcessStatus::from_verdict(l64_certification::CertificationVerdict::Incomplete).code(),
            11
        );
        assert_eq!(
            ProcessStatus::from_verdict(l64_certification::CertificationVerdict::Invalid).code(),
            12
        );
    }

    #[test]
    fn run_rna_returns_verdict_status_and_exact_source_span() {
        let root = std::env::temp_dir().join(format!("l64-cli-status-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir(&root).unwrap();
        let open = root.join("open.rna");
        fs::write(
            &open,
            b"L64R1 0x4345525449465931\na 1 0x52\nq 2 1 0 0 0 0 0 0 0\nv 3 2\nr 4 3 2\n",
        )
        .unwrap();
        let mut output = Vec::new();
        let status = dispatch(
            &[
                OsString::from("l64-cli"),
                OsString::from("run-rna"),
                open.into_os_string(),
            ],
            &mut output,
        )
        .unwrap();
        assert_eq!(status, ProcessStatus::Open);
        assert!(
            String::from_utf8(output)
                .unwrap()
                .contains("certification_verdict=OPEN")
        );

        let malformed = root.join("malformed.rna");
        fs::write(&malformed, b"L64R1 0x1\na nope 0x41\n").unwrap();
        let error = dispatch(
            &[
                OsString::from("l64-cli"),
                OsString::from("run-rna"),
                malformed.into_os_string(),
            ],
            &mut Vec::new(),
        )
        .unwrap_err();
        assert!(error.contains("invalid number at RNA 2:3"));
        assert!(error.contains("2 | a nope 0x41"));
        assert!(error.contains("|   ^^^^"));
        fs::remove_dir_all(root).unwrap();
    }
}
