# Locus64 Semantic Usage Guide

This guide explains how to use Locus64 as semantic governance infrastructure for evolving derivational frameworks, especially speculative or research-heavy systems where assumptions, branches, observational constraints, and unresolved pressure points must stay traceable.

The intended use is not to turn Locus64 into a physics engine or a generic theorem prover. The intended use is:

```text
closure-constrained derivational infrastructure
```

Locus64 should govern derivational legitimacy, lineage, challenge state, adequacy, reuse, and promotion/deprecation. Domain-specific symbolic algebra, tensor manipulation, numerical simulation, and paper writing can remain external tools whose outputs are imported as governed artifacts.

## Core Mental Model

Use Locus64 to convert an evolving body of ideas into a governed graph:

```text
human symbolic work -> RNA -> canonical lower chain -> DNA -> claims/evidence/challenges/lineage
```

The semantic split is:

- `RNA`: human-authored symbolic surface for equations, derivation statements, branch notes, and claim authoring.
- `DNA`: canonical machine artifact carrying normalized structure and lineage references.
- `ClaimPacket`: a governed claim with assumptions, target sector, caveats, and authority state.
- `EvidenceContract`: what evidence must exist before the claim can be promoted.
- `BenchmarkReceipt`: observational, computational, or stress-test evidence.
- `ChallengeReceipt` or `ChallengeRecord`: an unresolved or resolved objection/failure surface.
- `ReproducibilityPacket`: derivation/code/artifact refs needed to replay or audit the claim.
- `ResearchLineageRecord`: the causal chain linking lower-chain truth, reports, and research artifacts.

## Cosmology Mapping

For a cosmological framework, use this mapping:

| Framework object | Locus64 representation |
| --- | --- |
| primitive substrate assumption | canonical RNA/DNA root plus claim assumption |
| derived operational observable | benchmark target or projection artifact |
| gauge-equivalent description | transport-equivalent coverage or claim relation |
| unresolved pressure point | challenge record, remediation entry, or residual obligation |
| derivation chain | lineage receipt plus reproducibility packet |
| observational consistency check | benchmark receipt and adequacy clause |
| branch split | governed route, challenge, promotion, or deprecation state |
| surviving invariant equation | canonical normalized form |
| emergent effective description | warm-host research projection |
| equivalence-class ambiguity | conflict-resolution and transport legality surface |
| predictive test | benchmark packet |
| under-modeled anomaly | distress vector or residual verification receipt |

## Recommended Workflow

### 1. Author Symbolic RNA

Write a compact `.gene.rna` file for each claim root or derivation fragment.

Example:

```text
distributed_density_renormalization_preserves_tolman_scaling
```

Current RNA syntax is structural and intentionally minimal. If the equation is too rich for current RNA syntax, store the readable equation in a claim/reproducibility artifact and use RNA as the stable symbolic identity or derivation root.

### 2. Compile RNA To DNA

```powershell
mf normalize-rna .\distributed_tolman.gene.rna
mf compile-rna .\distributed_tolman.gene.rna --out .\distributed_tolman.gene.dna --artifact-class gene --persist-lineage
mf sequence-dna .\distributed_tolman.gene.dna
```

This creates a lower-chain record:

```text
Tokenization -> RnaNormalization -> StructuralResolution -> CanonicalNormalization
```

The DNA packet is the machine-facing artifact. Do not treat prose, filenames, or notebook cells as canonical authority.

### 3. Create A Claim Packet

A claim packet should capture:

- stable claim id
- claim class
- authority state
- target sector
- theorem-style statement
- assumptions
- caveats

Template:

```text
CLAIM:
Distributed density renormalization preserves Tolman scaling.

ASSUMPTIONS:
A1: observed metric is density-renormalized from absolute metric.
A2: photon propagation accumulates distributed density response.
A3: surface brightness is evaluated through the observed projection.

DERIVATION:
D1: define density response along null path.
D2: derive redshift accumulation.
D3: compute surface-brightness scaling.

OBSERVABLE:
B(z) proportional to (1+z)^-4.

STATUS:
constrained-surviving

FAILURE SURFACES:
acoustic peak structure unresolved
structure growth unresolved

DEPENDENCIES:
metric conformal renormalization
photon propagation renormalization

LINEAGE:
distributed_tolman.gene.dna
```

If using a `.qc0` bundle, represent these with surfaced `claim-packet`, `evidence-contract`, `benchmark-receipt`, `challenge-receipt`, `reproducibility-packet`, and `adequacy` entries.

### 4. Attach Evidence Contracts

An evidence contract states what must be true before promotion.

Typical cosmology evidence contract fields:

- required evidence kinds: derivation, benchmark, stress, observational comparison
- benchmark roles: target case, control, stress, trace
- admissibility thresholds: exact, bounded approximation, reproducible, independently checked
- promotion ceiling: exploratory, constrained, certified, integrated

Use evidence contracts to prevent claims from being promoted because they are rhetorically convincing.

### 5. Attach Benchmark Receipts

Use benchmark receipts for observational or computational checks:

- CMB acoustic peaks
- BAO scale
- SN1a distance-redshift relation
- lensing
- rotation curves
- Tolman surface-brightness tests
- Hubble tension
- structure formation
- nucleosynthesis

Each benchmark should declare:

- role: target case, control, stress, or trace
- verdict
- metrics
- reproducibility reference
- tolerance or exactness class

### 6. Persist Challenges

Do not delete failed branches. Persist them.

Examples:

| Challenge | Suggested state |
| --- | --- |
| endpoint-only renormalization violates Etherington duality | blocked or deprecated |
| distributed renormalization preserves Tolman scaling | constrained-surviving |
| acoustic peak recovery unresolved | open challenge |
| nucleosynthesis compatibility unresolved | open challenge |
| MOND-like scaling exploratory | exploratory challenge |
| residual geometric expansion unresolved | residual obligation |

This preserves derivational memory and prevents accidental reintroduction of broken branches.

### 7. Derive Research Lineage

After certification or report generation:

```powershell
mf research-derive-from-report --report-id REPORT_ID --persist
mf research-promotion-readiness REPORT_ID
mf research-status
```

Lineage should answer:

- what claim or report is this derived from?
- what lower-chain DNA or canonical hash grounds it?
- what evidence is attached?
- what challenges remain open?
- what promotion state is justified?

### 8. Use Coverage And Reuse Carefully

Locus64 can skip repeated work only when reuse is lawful.

Reuse requires:

- canonical identity match or declared transport-equivalence
- valid lineage
- replay legality
- matching policy/tolerance envelope
- residual obligations explicitly recorded

For cosmology, this matters when two branches are observationally equivalent under a projection but not ontologically identical.

## Promotion States

Use these semantic states consistently:

| State | Meaning |
| --- | --- |
| exploratory | authored but weakly constrained |
| constrained | partial observational or derivational support |
| challenged | live inconsistency or unresolved blocker |
| integrated | compatible with current closure/evidence requirements |
| blocked | contradiction or failed adequacy |
| deprecated | superseded by a stronger branch |

Do not collapse `challenged`, `blocked`, and `deprecated`; they mean different things operationally.

## Example: Branch Governance

For branch splits, create separate claim packets and link them by challenge/promotion records.

```text
Branch A:
endpoint-only renormalization
Status:
blocked
Challenge:
Etherington duality failure

Branch B:
distributed renormalization
Status:
constrained-surviving
Challenge:
acoustic peaks unresolved

Branch C:
residual geometric expansion
Status:
exploratory
Challenge:
not yet benchmarked against BAO/CMB
```

This is better than replacing old notes because it preserves why a branch failed or survived.

## What Not To Do

Do not use Locus64 as:

- a symbolic algebra replacement
- a tensor calculus engine
- a numerical cosmology simulator
- a prose database
- an ontology freezer

Use it as:

- claim governance
- derivation lineage
- adequacy routing
- challenge persistence
- benchmark accounting
- promotion/deprecation control
- canonical artifact conversion

## Practical File Layout For A Research Project

Suggested external project layout:

```text
cosmology-work/
  rna/
    distributed_tolman.gene.rna
    endpoint_renorm.gene.rna
  dna/
    distributed_tolman.gene.dna
  claims/
    distributed_tolman.claim.qc0
  benchmarks/
    tolman_surface_brightness.benchmark.json
    hubble_tension.benchmark.json
  challenges/
    etherington_endpoint.challenge.json
    acoustic_peaks.challenge.json
  repro/
    distributed_tolman.repro.json
  exports/
    reports/
```

Locus64 can live separately and be called by scripts or ChatGPT-generated workflows.

## Minimal Command Loop

```powershell
# 1. Compile identity/derivation root.
mf compile-rna .\rna\distributed_tolman.gene.rna --out .\dna\distributed_tolman.gene.dna --artifact-class gene --persist-lineage

# 2. Certify or validate surfaced bundle.
mf certify-bundle --file .\claims\distributed_tolman.claim.qc0 --conflict-policy exact-match

# 3. Inspect report.
mf observe-run --report REPORT_ID

# 4. Persist research lineage.
mf research-derive-from-report --report-id REPORT_ID --persist

# 5. Check promotion readiness.
mf research-promotion-readiness REPORT_ID

# 6. Export artifacts for review or ChatGPT handoff.
mf export-report --id REPORT_ID --to qc0
```

## ChatGPT Integration Pattern

When moving between ChatGPT and Locus64:

1. Draft claim/derivation in natural language.
2. Collapse it into a claim packet template.
3. Create or update the RNA identity/root.
4. Compile RNA to DNA.
5. Attach evidence, benchmark, challenge, and reproducibility records.
6. Certify or validate.
7. Export report back to ChatGPT for interpretation.
8. Convert ChatGPT critique into new challenge records or remediation entries, not silent edits.

This keeps iterative reasoning from mutating assumptions invisibly.

## Key Principle

Locus64 should preserve the difference between:

- what is canonical
- what is projected
- what is observed
- what is challenged
- what is merely useful
- what is promoted

That distinction is the main value of the system for evolving semantic frameworks.
