use anyhow::{Context, Result, anyhow};
use clap::Parser;
use l64_atlas::{CompiledAtlas, CompiledEdge};
use l64_bundle::{import_bundle_file, load_bundle_world};
use l64_core::{AtlasCell, BundleConflictPolicy, ClaimPacket, RegistryLookup};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Debug, Parser)]
#[command(name = "l64-atlas-export")]
#[command(about = "Compile and export a Locus64 atlas from seed or bundle overlay authority")]
struct Cli {
    /// Import a .dna bundle before compiling the atlas.
    #[arg(long, conflicts_with = "bundle")]
    file: Option<String>,

    /// Load a previously imported bundle by ID.
    #[arg(long, conflicts_with = "file")]
    bundle: Option<String>,

    /// Compile only the bundle-local overlay instead of seed + overlay.
    #[arg(long, default_value_t = false)]
    overlay_only: bool,

    /// Include complete compiled edge records in JSON output.
    #[arg(long, default_value_t = false)]
    dump_edges: bool,

    /// Include source atlas-cell records in JSON output.
    #[arg(long, default_value_t = false)]
    dump_cells: bool,

    /// Include engine-resolved governed claim packets in JSON output.
    #[arg(long, default_value_t = false)]
    dump_claims: bool,

    /// Write JSON output to a file instead of stdout.
    #[arg(long)]
    out: Option<String>,

    /// Write a Graphviz DOT projection of compiled atlas edges.
    #[arg(long)]
    dot_out: Option<String>,
}

#[derive(Debug, Serialize)]
struct RouteBucket {
    source_regime: String,
    target_regime: String,
    edge_count: usize,
    atlas_cell_ids: Vec<String>,
    burden_classes: Vec<String>,
    minimum_loss_count: usize,
    minimum_surface_penalty: usize,
}

#[derive(Debug, Serialize)]
struct AtlasExport {
    source: String,
    bundle_id: Option<String>,
    overlay_only: bool,
    edge_count: usize,
    atlas_cell_count: usize,
    claim_packet_count: usize,
    regime_pair_count: usize,
    burden_pair_count: usize,
    route_buckets: Vec<RouteBucket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edges: Option<Vec<CompiledEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cells: Option<Vec<AtlasCell>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    claims: Option<Vec<ClaimPacket>>,
}

fn build_export<R: RegistryLookup + ?Sized>(
    registry: &R,
    source: String,
    bundle_id: Option<String>,
    overlay_only: bool,
    dump_edges: bool,
    dump_cells: bool,
    dump_claims: bool,
) -> Result<AtlasExport> {
    let atlas = CompiledAtlas::compile(registry).map_err(anyhow::Error::msg)?;
    let cells = registry.atlas_cells();
    let claims = registry.claim_packets();
    let mut grouped: BTreeMap<(String, String), Vec<&CompiledEdge>> = BTreeMap::new();
    for edge in &atlas.edges {
        grouped
            .entry((edge.src.clone(), edge.tgt.clone()))
            .or_default()
            .push(edge);
    }
    let route_buckets = grouped
        .into_iter()
        .map(|((src, tgt), edges)| {
            let mut cell_ids = edges
                .iter()
                .map(|edge| edge.atlas_cell_id.clone())
                .collect::<Vec<_>>();
            cell_ids.sort();
            cell_ids.dedup();
            let mut burdens = edges
                .iter()
                .map(|edge| format!("{:?}", edge.burden_class))
                .collect::<Vec<_>>();
            burdens.sort();
            burdens.dedup();
            RouteBucket {
                source_regime: src,
                target_regime: tgt,
                edge_count: edges.len(),
                atlas_cell_ids: cell_ids,
                burden_classes: burdens,
                minimum_loss_count: edges.iter().map(|edge| edge.loss_count).min().unwrap_or(0),
                minimum_surface_penalty: edges
                    .iter()
                    .map(|edge| edge.surface_penalty)
                    .min()
                    .unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();
    let summary = atlas.compile_summary();
    Ok(AtlasExport {
        source,
        bundle_id,
        overlay_only,
        edge_count: summary.edge_count,
        atlas_cell_count: cells.len(),
        claim_packet_count: claims.len(),
        regime_pair_count: summary.indexed_src_tgt_pairs,
        burden_pair_count: summary.indexed_burden_pairs,
        route_buckets,
        edges: dump_edges.then_some(atlas.edges),
        cells: dump_cells.then_some(cells),
        claims: dump_claims.then_some(claims),
    })
}

fn escape_dot(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn write_dot(path: &str, export: &AtlasExport) -> Result<()> {
    let edges = export
        .edges
        .as_ref()
        .ok_or_else(|| anyhow!("--dot-out requires --dump-edges"))?;
    let mut out = String::from("digraph locus64_atlas {\n  rankdir=LR;\n");
    for edge in edges {
        let label = format!(
            "{}\\n{:?}\\nloss={} surface={}",
            edge.atlas_cell_id, edge.burden_class, edge.loss_count, edge.surface_penalty
        );
        out.push_str(&format!(
            "  \"{}\" -> \"{}\" [label=\"{}\"];\n",
            escape_dot(&edge.src),
            escape_dot(&edge.tgt),
            escape_dot(&label)
        ));
    }
    out.push_str("}\n");
    fs::write(path, out).with_context(|| format!("failed to write DOT output `{path}`"))?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let export = if let Some(file) = cli.file.as_deref() {
        let world = import_bundle_file(
            Path::new(file),
            None,
            BundleConflictPolicy::ExactMatch,
            None,
        )?;
        let bundle_id = Some(world.manifest.id.clone());
        if cli.overlay_only {
            let local = world.overlay.local_only();
            build_export(
                &local,
                format!("file:{file}"),
                bundle_id,
                true,
                cli.dump_edges,
                cli.dump_cells,
                cli.dump_claims,
            )?
        } else {
            build_export(
                &world.overlay,
                format!("file:{file}"),
                bundle_id,
                false,
                cli.dump_edges,
                cli.dump_cells,
                cli.dump_claims,
            )?
        }
    } else if let Some(bundle_id) = cli.bundle.as_deref() {
        let world = load_bundle_world(bundle_id)?;
        if cli.overlay_only {
            let local = world.overlay.local_only();
            build_export(
                &local,
                format!("bundle:{bundle_id}"),
                Some(bundle_id.to_string()),
                true,
                cli.dump_edges,
                cli.dump_cells,
                cli.dump_claims,
            )?
        } else {
            build_export(
                &world.overlay,
                format!("bundle:{bundle_id}"),
                Some(bundle_id.to_string()),
                false,
                cli.dump_edges,
                cli.dump_cells,
                cli.dump_claims,
            )?
        }
    } else {
        let registry = l64_registry::SeedRegistry::load()?;
        build_export(
            &registry,
            "seed".into(),
            None,
            false,
            cli.dump_edges,
            cli.dump_cells,
            cli.dump_claims,
        )?
    };

    if let Some(dot_out) = cli.dot_out.as_deref() {
        write_dot(dot_out, &export)?;
    }
    let json = serde_json::to_string_pretty(&export)?;
    if let Some(out) = cli.out.as_deref() {
        fs::write(out, json).with_context(|| format!("failed to write JSON output `{out}`"))?;
    } else {
        println!("{json}");
    }
    Ok(())
}
