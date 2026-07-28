use super::{ProcessStatus, dispatch};
use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

struct Case {
    name: &'static str,
    status: ProcessStatus,
}

const CASES: &[Case] = &[
    Case {
        name: "certified_triangle",
        status: ProcessStatus::Success,
    },
    Case {
        name: "equality_chain",
        status: ProcessStatus::Success,
    },
    Case {
        name: "open_obligation",
        status: ProcessStatus::Open,
    },
    Case {
        name: "invalid_child_context",
        status: ProcessStatus::Invalid,
    },
    Case {
        name: "matrix_multiply",
        status: ProcessStatus::Success,
    },
];

fn corpus() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("samples")
        .join("golden")
}

fn fixture(name: &str) -> PathBuf {
    corpus().join(format!("{name}.rna"))
}

fn expected(name: &str) -> Vec<u8> {
    fs::read(corpus().join("expected").join(name)).unwrap()
}

fn temp_root(label: &str) -> PathBuf {
    let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "l64 golden π {label} {} {serial}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root
}

fn run(args: Vec<OsString>) -> (ProcessStatus, Vec<u8>) {
    let mut output = Vec::new();
    let status = dispatch(&args, &mut output).unwrap();
    (status, output)
}

fn command(command: &str, path: &Path) -> Vec<OsString> {
    vec![
        OsString::from("l64-cli"),
        OsString::from(command),
        path.as_os_str().to_owned(),
    ]
}

fn command_out(command: &str, input: &Path, out: &Path) -> Vec<OsString> {
    vec![
        OsString::from("l64-cli"),
        OsString::from(command),
        input.as_os_str().to_owned(),
        OsString::from("--out"),
        out.as_os_str().to_owned(),
    ]
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2 + 1);
    for byte in bytes {
        use std::fmt::Write as _;
        write!(out, "{byte:02x}").unwrap();
    }
    out.push('\n');
    out
}

#[test]
fn golden_portability_authority_snapshots_are_exact() {
    let root = temp_root("authority snapshots");
    for case in CASES {
        let rna = fixture(case.name);
        let dna = root.join(format!("{}.dna", case.name));

        let (run_status, run_output) = run(command("run-rna", &rna));
        assert_eq!(run_status, case.status, "{} run status", case.name);
        assert_eq!(
            run_output,
            expected(&format!("{}.run.txt", case.name)),
            "{} run snapshot",
            case.name
        );

        let (normalize_status, normalized) = run(command("normalize-rna", &rna));
        assert_eq!(normalize_status, ProcessStatus::Success);
        assert_eq!(
            normalized,
            expected(&format!("{}.normalized.rna", case.name)),
            "{} normalized RNA",
            case.name
        );

        let (compile_status, _) = run(command_out("compile-rna", &rna, &dna));
        assert_eq!(compile_status, ProcessStatus::Success);
        assert_eq!(
            hex(&fs::read(&dna).unwrap()).as_bytes(),
            expected(&format!("{}.dna.hex", case.name)),
            "{} canonical DNA",
            case.name
        );

        let (sequence_status, sequenced) = run(command("sequence-dna", &dna));
        assert_eq!(sequence_status, ProcessStatus::Success);
        assert_eq!(sequenced, normalized, "{} RNA/DNA fixed point", case.name);

        let (certify_status, certification) = run(command("certify-dna", &dna));
        assert_eq!(
            certify_status, case.status,
            "{} certification status",
            case.name
        );
        assert_eq!(
            certification,
            expected(&format!("{}.certify.txt", case.name)),
            "{} certification snapshot",
            case.name
        );
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn golden_portability_crlf_and_unicode_paths_preserve_authority() {
    let root = temp_root("CRLF and Unicode");
    for case in CASES {
        let source = fs::read(fixture(case.name)).unwrap();
        let crlf = String::from_utf8(source)
            .unwrap()
            .replace("\n", "\r\n")
            .into_bytes();
        let crlf_rna = root.join(format!("{} portable source.rna", case.name));
        let crlf_dna = root.join(format!("{} portable authority.dna", case.name));
        fs::write(&crlf_rna, crlf).unwrap();

        let (_, normalized) = run(command("normalize-rna", &crlf_rna));
        assert_eq!(
            normalized,
            expected(&format!("{}.normalized.rna", case.name)),
            "{} CRLF normalization",
            case.name
        );
        let _ = run(command_out("compile-rna", &crlf_rna, &crlf_dna));
        assert_eq!(
            hex(&fs::read(&crlf_dna).unwrap()).as_bytes(),
            expected(&format!("{}.dna.hex", case.name)),
            "{} CRLF canonical DNA",
            case.name
        );
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn golden_portability_product_journey_is_exact() {
    let root = temp_root("product journey");
    let triangle = root.join("triangle authority.dna");
    let equality = root.join("equality authority.dna");
    let open = root.join("open authority.dna");
    let bundle = root.join("mixed ordered transport.l64b");
    let release = root.join("triangle native release");

    for (name, out) in [
        ("certified_triangle", &triangle),
        ("equality_chain", &equality),
        ("open_obligation", &open),
    ] {
        let _ = run(command_out("compile-rna", &fixture(name), out));
    }

    let (compare_status, compare) = run(vec![
        OsString::from("l64-cli"),
        OsString::from("compare-dna"),
        triangle.as_os_str().to_owned(),
        equality.as_os_str().to_owned(),
    ]);
    assert_eq!(compare_status, ProcessStatus::Success);
    assert_eq!(compare, expected("triangle_to_equality.compare.txt"));

    let (bundle_status, _) = run(vec![
        OsString::from("l64-cli"),
        OsString::from("compile-bundle"),
        triangle.as_os_str().to_owned(),
        open.as_os_str().to_owned(),
        OsString::from("--out"),
        bundle.as_os_str().to_owned(),
    ]);
    assert_eq!(bundle_status, ProcessStatus::Success);

    for (command_name, snapshot) in [
        ("run-bundle", "mixed_bundle.run.txt"),
        ("certify-bundle", "mixed_bundle.certify.txt"),
        ("observe-bundle", "mixed_bundle.observe.txt"),
    ] {
        let (status, output) = run(vec![
            OsString::from("l64-cli"),
            OsString::from(command_name),
            OsString::from("--file"),
            bundle.as_os_str().to_owned(),
        ]);
        assert_eq!(status, ProcessStatus::Open, "{command_name} status");
        assert_eq!(output, expected(snapshot), "{command_name} snapshot");
    }

    let (release_status, _) = run(vec![
        OsString::from("l64-cli"),
        OsString::from("export-genome-release"),
        OsString::from("--rna"),
        fixture("certified_triangle").into_os_string(),
        OsString::from("--out"),
        release.as_os_str().to_owned(),
    ]);
    assert_eq!(release_status, ProcessStatus::Success);
    assert_eq!(
        fs::read(release.join("source.rna")).unwrap(),
        expected("certified_triangle.normalized.rna")
    );
    assert_eq!(
        hex(&fs::read(release.join("authority.dna")).unwrap()).as_bytes(),
        expected("triangle.release.authority.dna.hex")
    );
    assert_eq!(
        fs::read(release.join("projection.txt")).unwrap(),
        expected("triangle.release.projection.txt")
    );
    assert_eq!(
        fs::read(release.join("release.record")).unwrap(),
        expected("triangle.release.record")
    );

    let mut names = fs::read_dir(&release)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(
        names,
        [
            "authority.dna",
            "projection.txt",
            "release.record",
            "source.rna"
        ]
        .into_iter()
        .map(OsString::from)
        .collect::<Vec<_>>()
    );

    fs::remove_dir_all(root).unwrap();
}
