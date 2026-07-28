#![forbid(unsafe_code)]

use l64_native::{DnaError, ROOT_CONTEXT, RnaError, decode_dna, dna_to_rna, rna_to_dna};
use l64_projection::{ProjectionError, ProjectionSet};
use std::{
    fmt::{self, Write as _},
    fs, io,
    path::{Path, PathBuf},
};

pub const RELEASE_VERSION: u16 = 1;

#[derive(Debug)]
pub enum ReleaseError {
    OutputExists(PathBuf),
    InvalidOutput(PathBuf),
    Native(String),
    Rna {
        phase: &'static str,
        error: RnaError,
    },
    Dna {
        phase: &'static str,
        error: DnaError,
    },
    Projection(ProjectionError),
    Io(io::Error),
}

impl fmt::Display for ReleaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutputExists(path) => {
                write!(f, "release output `{}` already exists", path.display())
            }
            Self::InvalidOutput(path) => {
                write!(f, "invalid release output path `{}`", path.display())
            }
            Self::Native(error) => f.write_str(error),
            Self::Rna { phase, error } => write!(f, "release {phase} failed: {error}"),
            Self::Dna { phase, error } => write!(f, "release {phase} failed: {error}"),
            Self::Projection(error) => write!(f, "release projection failed: {error}"),
            Self::Io(error) => write!(f, "release filesystem operation failed: {error}"),
        }
    }
}

impl ReleaseError {
    pub const fn rna_error(&self) -> Option<RnaError> {
        match self {
            Self::Rna { error, .. } => Some(*error),
            _ => None,
        }
    }
}

impl std::error::Error for ReleaseError {}

impl From<ProjectionError> for ReleaseError {
    fn from(value: ProjectionError) -> Self {
        Self::Projection(value)
    }
}

impl From<io::Error> for ReleaseError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeRelease {
    pub root: PathBuf,
    pub source: PathBuf,
    pub authority: PathBuf,
    pub projection: PathBuf,
    pub record: PathBuf,
}

pub fn export_native_release(
    source: &[u8],
    out_root: &Path,
) -> Result<NativeRelease, ReleaseError> {
    if out_root.exists() {
        return Err(ReleaseError::OutputExists(out_root.to_path_buf()));
    }
    let parent = out_root
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let name = out_root
        .file_name()
        .ok_or_else(|| ReleaseError::InvalidOutput(out_root.to_path_buf()))?;
    fs::create_dir_all(parent)?;

    let canonical_rna = l64_native::normalize_rna(source).map_err(|error| ReleaseError::Rna {
        phase: "normalization",
        error,
    })?;
    let dna = rna_to_dna(&canonical_rna).map_err(|error| ReleaseError::Rna {
        phase: "compilation",
        error,
    })?;
    let graph = decode_dna(&dna).map_err(|error| ReleaseError::Dna {
        phase: "decoding",
        error,
    })?;
    let sequenced = dna_to_rna(&dna).map_err(|error| ReleaseError::Rna {
        phase: "sequencing",
        error,
    })?;
    if sequenced != canonical_rna {
        return Err(ReleaseError::Native(
            "canonical RNA fixed point failed".into(),
        ));
    }
    let projection = ProjectionSet::derive(&graph, ROOT_CONTEXT, 16)?;
    projection.verify(&graph)?;

    let stage = parent.join(format!(
        ".{}.l64-release-{}",
        name.to_string_lossy(),
        std::process::id()
    ));
    if stage.exists() {
        fs::remove_dir_all(&stage)?;
    }
    let result = write_stage(&stage, &canonical_rna, &dna, &projection).and_then(|_| {
        fs::rename(&stage, out_root)?;
        Ok(NativeRelease {
            root: out_root.to_path_buf(),
            source: out_root.join("source.rna"),
            authority: out_root.join("authority.dna"),
            projection: out_root.join("projection.txt"),
            record: out_root.join("release.record"),
        })
    });
    if result.is_err() && stage.exists() {
        let _ = fs::remove_dir_all(&stage);
    }
    result
}

fn write_stage(
    stage: &Path,
    canonical_rna: &[u8],
    dna: &[u8],
    projection: &ProjectionSet,
) -> Result<(), ReleaseError> {
    fs::create_dir(stage)?;
    fs::write(stage.join("source.rna"), canonical_rna)?;
    fs::write(stage.join("authority.dna"), dna)?;
    let projection_text = projection.render_text();
    fs::write(stage.join("projection.txt"), projection_text.as_bytes())?;

    let mut record = String::new();
    let _ = writeln!(record, "L64 NATIVE RELEASE v{RELEASE_VERSION}");
    let _ = writeln!(record, "authority=authority.dna");
    let _ = writeln!(record, "source=source.rna");
    let _ = writeln!(record, "projection=projection.txt");
    let _ = writeln!(record, "projection_authority=non_authoritative");
    let _ = writeln!(record, "state_symbol={}", projection.source.symbol);
    let _ = writeln!(record, "source_bytes={}", canonical_rna.len());
    let _ = writeln!(record, "dna_bytes={}", dna.len());
    let _ = writeln!(record, "projection_verified=true");
    fs::write(stage.join("release.record"), record)?;
    Ok(())
}
