# Locus64 Interaction Language Specification

Status note: RNA and DNA are the active public surfaces. Bundle-entry text remains an authoring convenience for generating `.dna` bundle packets, but QC0/QA0/QM0/QK0 are no longer public interaction languages or workspace crates.

This is the concrete syntax and record-shape specification for interacting with Locus64.

There are three practical interaction forms:

1. **Command language**: shell commands accepted by `l64`.
2. **RNA language**: small symbolic input compiled into DNA.
3. **Bundle-entry text**: one record-kind plus one JSON object per line, compiled into `.dna`.

If you are driving Locus64 indirectly from ChatGPT or another system, generate RNA or bundle-entry text, compile it into DNA, and then use the `.dna` artifact for certification/execution.

Native structural law is moving toward codon/lexon governance:

- **Codons** are finite structural operators/classes with arity, phase/admission law, and opcode identity.
- **Lexons** are scoped symbolic bindings for domain-local objects such as chain rule, derivative, composition, and bridge-sector patterns.
- **Macro-codons** are recurring structural/reaction patterns such as roundtrip, closure frontier, view receipt, integration receipt, and expression receipt. They are not lexons and do not share the lexon registry.
- Aliases are ingress-only. Generated native artifacts should emit canonical symbols, not structural aliases as words.
- Header truth and codon phase law outrank filename extensions. A source/product/receipt/view decision must come from the native header and phase table, not from `.rna`, `.pep`, `.prot`, JSON shape, or command habit.
- Opcode allocation is stable law: never reuse an opcode, deprecate by tombstone, migrate by receipt, and do not rely on Rust enum variant order for persisted numeric meaning.
- Generated-word bans apply to native structural regions only. Words such as `claim_page`, `dependency_spine`, `theorem`, or `campaign` may appear in gloss/comments/views, but must not survive as generated native structural tokens.
- Digests and hashes are machine memo bindings, not public proof identity.
- Products, views, receipts, and projections remain rejected as source unless explicitly reconstructed into canonical RNA.

This law exists to prevent future bundle/product records from becoming JSON with different punctuation. Bundle-entry text remains transitional until representative workflows are rerouted through substrate-native records.

Current representative registries:

- Codon module: `l64-core/src/codons.rs`
- Lexon module: `l64-core/src/lexons.rs`
- Macro-codon module: `l64-core/src/macro_codons.rs`

The controlled CKI registry fixture lives at `fixtures/cki_registry.genome.rna`. It is a regression fixture for the RNA/DNA fixed-point membrane, not independent authority and not a third public surface.

## 1. Command Language

Primary executable:

```text
l64 <command> [args...]
```

`l64` is a wrapper. Keep `l64`, `l64-cli`, and `l64-admin` in the same directory.

Core commands:

```text
l64 normalize-rna <file>
l64 compile-rna <file> [--out <file.dna>] [--artifact-class gene|haplotype|chromosome|genome] [--persist-lineage]
l64 sequence-dna <file.dna>
l64 inspect-dna <file.dna>
l64 verify-roundtrip <file.rna> [--artifact-class gene|haplotype|chromosome|genome]
l64 export-genome-release --rna <file.rna> --out <dir> [--artifact-class gene|haplotype|chromosome|genome]

l64 compile-bundle <file.locus.rna|file.dna> [--out <file.dna>]
l64 certify-bundle --file <file.dna> --conflict-policy exact-match
l64 certify-derived --campaign <campaign-id>
l64 observe-run --report <report-id>
l64 export-report-dna --report-id <report-id> --out <report-id>.dna
l64 export-validation-dna-bundle --id <report-id> --out <report-id>.validation.dna

l64 research-derive-from-report --report-id <report-id> --persist
l64 research-promotion-readiness <report-id>
l64 research-status

l64 dispatch-coverage --report-id <report-id>
l64 derive-frontier --report-id <report-id>
l64 tower-step --report-id <report-id>
```

## 2. RNA Language

RNA is a symbolic input language for lower-chain identity/structure roots.

Pipeline:

```text
raw RNA -> TOKENIZE -> RNORM -> SSR -> CNORM -> DNA
```

### 2.1 Token Classes

The tokenizer classifies characters into:

| Class | Characters / rule |
| --- | --- |
| `Atom` | lowercase, digits, Greek/non-control symbolic characters not otherwise classified |
| `Binder` | ASCII uppercase `A-Z` |
| `Operator` | `_`, `|`, `:`, `=`, `+`, `-`, `*`, `/`, `^`, `≔`, `‖`, `→` |
| `GroupLeft` | `(`, `[`, `{` |
| `GroupRight` | `)`, `]`, `}` |
| `Separator` | `,`, `;` |
| `StrandRef` | `.` |
| `Meta` | whitespace |

Non-whitespace control characters are invalid.

### 2.2 Current Practical RNA Form

The current implementation preserves tokens and normalizes whitespace/group/splice structure. It does **not** parse full mathematical notation as a theorem language.

Safe forms:

```text
i := s || k
ι ≔ σ ‖ κ
A.x := y
(a := b)
```

Splice markers:

```text
iei <payload> eie
```

Unclosed splice regions fail.

### 2.3 RNA Commands

```powershell
l64 normalize-rna .\claim_root.gene.rna
l64 compile-rna .\claim_root.gene.rna --out .\claim_root.gene.dna --artifact-class gene --persist-lineage
l64 sequence-dna .\claim_root.gene.dna
l64 inspect-dna .\claim_root.gene.dna
l64 verify-roundtrip .\claim_root.gene.rna --artifact-class gene
l64 export-genome-release --rna .\claim_root.gene.rna --out .\claim_root.release --artifact-class gene
```

### 2.4 RNA Output Guarantees

`compile-rna` accepts source RNA only. It rejects inspection reports, receipts, views, closure maps, claim pages, stress maps, replay records, and JSON report/projection text before lower-chain tokenization.

Successful `compile-rna` returns an inspection report containing:

- byte count
- packet summary
- normalized RNA
- tokenization/RNORM/SSR/CNORM receipts
- DNA validation report
- lineage record

`sequence-dna` emits canonical reconstructable RNA, not JSON.

`inspect-dna` emits a JSON inspection report for humans and tooling. Inspection output is not source and must not be passed to `compile-rna`.

`verify-roundtrip` proves the public fixed-point law:

```text
RNA -> DNA -> canonical RNA -> DNA
```

The canonical hash must be preserved. Packet bytes may differ when intentionally variable metadata changes, but same-subject library roundtrips are expected to stabilize byte-for-byte.

Use RNA/DNA for identity and canonicalization. Bundle-entry text remains a transitional authoring convenience compiled into `.dna`; it is not a public authority surface.

`export-genome-release` creates derived review artifacts from the same RNA/DNA authority spine. Its manifest, claim pages, dependency spine, closure map, closure frontier, stress map, lineage, replay record, views, and view receipts are projections or records. They use role-specific projection/record/receipt filenames, are not source RNA, and are rejected by `compile-rna`; generated source RNA and canonical RNA are the re-compilable text artifacts.

## 3. Bundle-Entry Authoring Text

Bundle-entry text is a transitional authoring convenience for generating `.dna` bundle packets. It is not a public authority surface and is not accepted by normal bundle execution without first compiling it through `compile-bundle`.

### 3.1 File Structure

Bundle-entry text is line-oriented:

```text
!l64-bundle v1
<entry-kind> <entry-json>
<entry-kind> <entry-json>
...
```

Rules:

- First non-empty line must be `!l64-bundle v1`.
- Each later non-empty line must contain one entry kind, one space, and one JSON object.
- JSON must be valid on a single line.
- Entry order can matter operationally for readability, but parsing accepts entries by kind.
- Comments are not part of bundle-entry projection syntax. Do not emit comments inside projection entry files.

Header:

```text
!l64-bundle v1
```

`capability_id` may be `null`.

### 3.2 Accepted Entry Kinds

The parser accepts these entry kind strings:

```text
object
regime
bridge
proof
mechanization
theorem
obligation
target
ledger
certificate
campaign
portfolio
route-class
diagnostic
policy-object
surface-policy
format-receipt
roundtrip-report
capability-matrix
surface-budget
surface-deficiency
adequacy
burden-pack
claim-packet
evidence-contract
benchmark-receipt
challenge-receipt
reproducibility-packet
```

For most semantic/governance workflows, use:

```text
proof
bridge
atlas
theorem
obligation
target
ledger
campaign
burden-pack
claim-packet
evidence-contract
benchmark-receipt
challenge-receipt
reproducibility-packet
adequacy
```

Note: `atlas` is accepted by the broader surface parser in the current samples and certification flow.

## 4. Core Record Shapes

All enum values are case-sensitive.

### 4.1 `claim-packet`

Use for a governed claim.

```text
claim-packet {"id":"CLM_ID","claim_class":"Kernel","authority_state":"Evidence","target_sector":"cosmology","statement":"claim text","assumptions":["A1","A2"],"open_caveats":["gap"]}
```

Fields:

| Field | Type | Required |
| --- | --- | --- |
| `id` | string | yes |
| `claim_class` | `Kernel` \| `Interoperability` \| `Host` | yes |
| `authority_state` | `Derived` \| `Benchmark` \| `Evidence` | yes |
| `target_sector` | string | yes |
| `statement` | string | yes |
| `assumptions` | string array | no, default `[]` |
| `open_caveats` | string array | no, default `[]` |

### 4.2 `evidence-contract`

Use to define required evidence before promotion.

```text
evidence-contract {"id":"ECT_ID","required_evidence_kinds":["derivation","observation"],"required_benchmark_roles":["TargetCase","Stress"],"requires_stress":true,"requires_challenge":true,"admissibility_thresholds":["reproducible"],"promotion_ceiling":"Certified"}
```

Fields:

| Field | Type |
| --- | --- |
| `id` | string |
| `required_evidence_kinds` | string array |
| `required_benchmark_roles` | `TargetCase` \| `Control` \| `Stress` \| `Trace` array |
| `requires_stress` | bool |
| `requires_challenge` | bool |
| `admissibility_thresholds` | string array |
| `promotion_ceiling` | certification verdict |

### 4.3 `benchmark-receipt`

Use for an observational/computational/stress result.

```text
benchmark-receipt {"id":"BMR_ID","claim_packet_id":"CLM_ID","role":"TargetCase","verdict":"Certified","metrics":{"score":"1.0","dataset":"tolman"},"reproducibility_ref":"RPK_ID"}
```

Fields:

| Field | Type |
| --- | --- |
| `id` | string |
| `claim_packet_id` | string |
| `role` | `TargetCase` \| `Control` \| `Stress` \| `Trace` |
| `verdict` | certification verdict |
| `metrics` | object with string values |
| `reproducibility_ref` | string |

### 4.4 `challenge-receipt`

Use for objections, anomalies, failed branches, or required responses.

```text
challenge-receipt {"id":"CHR_ID","claim_packet_id":"CLM_ID","grounds":["Etherington duality failure"],"required_response":"replace endpoint-only branch","status":"Open"}
```

Fields:

| Field | Type |
| --- | --- |
| `id` | string |
| `claim_packet_id` | string |
| `grounds` | string array |
| `required_response` | string |
| `status` | `Open` \| `Addressed` \| `Retired` |

### 4.5 `reproducibility-packet`

Use for replay/audit references.

```text
reproducibility-packet {"id":"RPK_ID","claim_packet_id":"CLM_ID","derivation_path":["D1","D2"],"code_refs":["notebook.ipynb"],"benchmark_refs":["BMR_ID"],"artifact_refs":["claim_root.gene.dna"]}
```

Fields:

| Field | Type |
| --- | --- |
| `id` | string |
| `claim_packet_id` | string |
| `derivation_path` | string array |
| `code_refs` | string array |
| `benchmark_refs` | string array |
| `artifact_refs` | string array |

### 4.6 `burden-pack`

Use to bind obligations, adequacy clauses, and evidence contracts into one certifiable burden.

```text
burden-pack {"id":"BPK_ID","allowed_host_cluster":["R_TOP","R_CALC"],"obligation_ids":["OBL_ID"],"adequacy_clause_ids":["ADQ_ID"],"required_proof_shape_family":"Square","route_class_constraints":[],"evidence_contract_ids":["ECT_ID"],"promotion_ceiling":"Certified","blocker_taxonomy":["DEvidenceContract","DBenchmarkGap","DStressGap","DChallengeGap"]}
```

Fields:

| Field | Type |
| --- | --- |
| `id` | string |
| `allowed_host_cluster` | string array |
| `obligation_ids` | string array |
| `adequacy_clause_ids` | string array |
| `required_proof_shape_family` | proof shape family |
| `route_class_constraints` | string array |
| `evidence_contract_ids` | string array |
| `promotion_ceiling` | certification verdict |
| `blocker_taxonomy` | deficiency class array |

### 4.7 `adequacy`

Use to connect claim/evidence/challenge objects to certification adequacy.

```text
adequacy {"id":"ADQ_ID","kind":"EvidenceContractInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_ID"],"burden_pack_ids":["BPK_ID"],"claim_packet_ids":["CLM_ID"],"evidence_contract_ids":["ECT_ID"],"benchmark_receipt_ids":[],"challenge_receipt_ids":[],"reproducibility_packet_ids":["RPK_ID"],"description":"evidence contract present","blocking":true}
```

`kind` values:

```text
ObjectInterpretation
ThreadInterpretation
EquivalenceInterpretation
TollInterpretation
KnotInterpretation
BridgeSoundness
ProjectionInterpretation
ContainmentInterpretation
ClosureInterpretation
RunningLawInterpretation
EvidenceContractInterpretation
BenchmarkInterpretation
StressInterpretation
ChallengeInterpretation
```

### 4.8 `theorem`

Use as the central certifiable statement.

```text
theorem {"id":"THS_ID","statement":"claim statement","hosts":["R_TOP","R_CALC"],"bridges":["B_ID"],"operators":["OPR.ID"],"target_equivalence":"eq","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_ID"]}
```

Fields:

| Field | Type |
| --- | --- |
| `id` | string |
| `statement` | string |
| `hosts` | string array |
| `bridges` | string array |
| `operators` | string array |
| `target_equivalence` | string |
| `obligations` | obligation kind array |
| `primary_zone` | proof mechanism zone |
| `verdict` | certification verdict |
| `proof_shapes` | string array |

### 4.9 `obligation`

```text
obligation {"id":"OBL_ID","kind":"OblAdm","description":"admissibility check","status":"Benchmarked"}
```

`kind` values:

```text
OblEq
OblAdm
OblLoc
OblGlu
OblTol
OblRed
OblBrg
OblRbk
OblAde
OblFin
OblObs
OblKnt
```

### 4.10 `target`

```text
target {"id":"TGT_ID","burden_class":"ImportedKernelClaim","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"eq","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"OpenBlocked","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}
```

### 4.11 `ledger`

```text
ledger {"id":"TRL_ID","theorem":"THS_ID","paths":[["B_ID"]],"budget":{"max_loss":1,"allow_lossy_supported":false,"require_proof":true},"losses":[],"receipts":["Trace_ID"],"normalized_path":["B_ID"]}
```

### 4.12 `campaign`

```text
campaign {"id":"CPG_ID","theorem":"THS_ID","target_profile":"TGT_ID","route_ledger":"TRL_ID","obligations":["OBL_ID"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["claim"]}
```

## 5. Common Enum Values

### Certification Verdict

```text
Invalid
Underspecified
RouteFound
Benchmarked
Certified
Integrated
BlockedOpen
BlockedContradiction
```

### Proof Shape Family

```text
Minimal
Triangle
Square
Diamond
Pentagon
Hexagon
MixedBattery
```

### Proof Mechanism Zone

```text
PmzStructural
PmzBridge
PmzLocalToGlobal
PmzAggregation
PmzReduction
PmzCanonicalization
PmzSemantic
PmzSpectral
PmzObstruction
PmzProbabilistic
PmzOperational
```

### Campaign Class

```text
CBasic
CBridge
CWelded
COperator
CAtlas
COpen
```

### Burden Class

```text
ExtensionalCarrierReasoning
ProofRelevantAlgebra
DerivativeLocalWitnessExtraction
MeasurableNormalizedBranching
ProbabilisticJudgment
SimulationExecutableInference
CertifiedPropertyWitness
ImportedKernelClaim
ProjectionClosure
Containment
RunningLaw
General
```

### Reversibility Class

```text
Exact
Conservative
Enriching
Forgetting
LossySupported
Invalid
```

### Promotion Goal

```text
PromoteOperator
PromoteBridge
PromoteRouteClass
Retire
OpenBlocked
```

### Deficiency Class

```text
DNoRoute
DHighLoss
DBadEqTransport
DRollbackCliff
DNoAdequacy
DBridge
DEq
DSelector
DRoundtrip
DPromo
DNoCommutingProof
DNoOperatorPayoff
DOpenConjectural
DProjection
DContainment
DClosure
DRunningLaw
DEvidenceContract
DBenchmarkGap
DStressGap
DChallengeGap
DHostPackMissing
```

## 6. Minimal Complete Imported Claim Bundle

This is the smallest practical semantic bundle shape for a governed external claim:

```text
!l64-bundle v1
proof {"id":"PS_COSMO","kind":"Square","nodes":["a","b","c","d"],"edges":[{"from":"a","to":"b","label":"derive"},{"from":"b","to":"d","label":"observe"},{"from":"a","to":"c","label":"project"},{"from":"c","to":"d","label":"compare"}],"equations":["observable=projection(assumption)"],"target_equivalence":"observational-equivalence","receipts":["RCP_COSMO"],"gate":"Pass"}
bridge {"id":"B_COSMO","src":"R_TOP","tgt":"R_CALC","id_pres":"preserved","eq_pres":"observational","forget":[],"enrich":["observable"],"loss":[],"reversibility":"Enriching","receipts":["RCP_COSMO"],"rollback":"allowed"}
atlas {"id":"A_COSMO","source_regime":"R_TOP","target_regime":"R_CALC","burden_class":"ImportedKernelClaim","proof_target":"cosmology-claim","candidate_paths":[["B_COSMO"]],"normalized_winner":["B_COSMO"],"winner_state":"Candidate","loss_profile":{"items":[]},"proof_shapes_checked":["PS_COSMO"],"recipe_maturity":"Stable","failure_signatures":[],"side_conditions":[],"surface_transition":{"compatibility":"AuthorityPreserving","penalties":[],"total_penalty":0}}
theorem {"id":"THS_COSMO","statement":"distributed density renormalization preserves Tolman scaling","hosts":["R_TOP","R_CALC"],"bridges":["B_COSMO"],"operators":["OPR.Cosmo"],"target_equivalence":"observational-equivalence","obligations":["OblAdm"],"primary_zone":"PmzStructural","verdict":"Benchmarked","proof_shapes":["PS_COSMO"]}
obligation {"id":"OBL_COSMO","kind":"OblAdm","description":"claim is admissible under stated assumptions","status":"Benchmarked"}
target {"id":"TGT_COSMO","burden_class":"ImportedKernelClaim","host_cluster":["R_TOP","R_CALC"],"target_equivalence":"observational-equivalence","allowed_bridge_classes":["Enriching"],"loss_ceiling":1,"rollback_ceiling":1,"required_receipt_class":"RC","required_proof_shape_family":"Square","promotion_goal":"OpenBlocked","primary_zone":"PmzStructural","surface_requirement":null,"preferred_surface_target":null,"optimizer_policy":null,"policy_binding_ids":[]}
ledger {"id":"TRL_COSMO","theorem":"THS_COSMO","paths":[["B_COSMO"]],"budget":{"max_loss":1,"allow_lossy_supported":false,"require_proof":true},"losses":[],"receipts":["Trace_COSMO"],"normalized_path":["B_COSMO"]}
campaign {"id":"CPG_COSMO","theorem":"THS_COSMO","target_profile":"TGT_COSMO","route_ledger":"TRL_COSMO","obligations":["OBL_COSMO"],"certificates":[],"dependencies":[],"campaign_class":"CBasic","verdict":"Benchmarked","payoff":["cosmology-claim"]}
burden-pack {"id":"BPK_COSMO","allowed_host_cluster":["R_TOP","R_CALC"],"obligation_ids":["OBL_COSMO"],"adequacy_clause_ids":["ADQ_COSMO_EVID","ADQ_COSMO_BENCH","ADQ_COSMO_STRESS","ADQ_COSMO_CHALLENGE"],"required_proof_shape_family":"Square","route_class_constraints":[],"evidence_contract_ids":["ECT_COSMO"],"promotion_ceiling":"Certified","blocker_taxonomy":["DEvidenceContract","DBenchmarkGap","DStressGap","DChallengeGap"]}
claim-packet {"id":"CLM_COSMO","claim_class":"Kernel","authority_state":"Evidence","target_sector":"cosmology","statement":"distributed density renormalization preserves Tolman scaling","assumptions":["observed metric is density-renormalized","redshift accumulates along path"],"open_caveats":["acoustic peaks unresolved"]}
evidence-contract {"id":"ECT_COSMO","required_evidence_kinds":["derivation","observation"],"required_benchmark_roles":["TargetCase","Stress"],"requires_stress":true,"requires_challenge":true,"admissibility_thresholds":["reproducible"],"promotion_ceiling":"Certified"}
benchmark-receipt {"id":"BMR_COSMO_TARGET","claim_packet_id":"CLM_COSMO","role":"TargetCase","verdict":"Certified","metrics":{"tolman_scaling":"pass"},"reproducibility_ref":"RPK_COSMO"}
benchmark-receipt {"id":"BMR_COSMO_STRESS","claim_packet_id":"CLM_COSMO","role":"Stress","verdict":"Benchmarked","metrics":{"cmb_acoustic_peaks":"unresolved"},"reproducibility_ref":"RPK_COSMO"}
challenge-receipt {"id":"CHR_COSMO","claim_packet_id":"CLM_COSMO","grounds":["acoustic peak structure unresolved"],"required_response":"attach CMB benchmark or leave residual obligation open","status":"Open"}
reproducibility-packet {"id":"RPK_COSMO","claim_packet_id":"CLM_COSMO","derivation_path":["A1","D1","D2"],"code_refs":[],"benchmark_refs":["BMR_COSMO_TARGET","BMR_COSMO_STRESS"],"artifact_refs":["CLM_COSMO"]}
adequacy {"id":"ADQ_COSMO_EVID","kind":"EvidenceContractInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_COSMO"],"burden_pack_ids":["BPK_COSMO"],"claim_packet_ids":["CLM_COSMO"],"evidence_contract_ids":["ECT_COSMO"],"benchmark_receipt_ids":[],"challenge_receipt_ids":[],"reproducibility_packet_ids":["RPK_COSMO"],"description":"evidence contract present","blocking":true}
adequacy {"id":"ADQ_COSMO_BENCH","kind":"BenchmarkInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_COSMO"],"burden_pack_ids":["BPK_COSMO"],"claim_packet_ids":["CLM_COSMO"],"evidence_contract_ids":["ECT_COSMO"],"benchmark_receipt_ids":["BMR_COSMO_TARGET","BMR_COSMO_STRESS"],"challenge_receipt_ids":[],"reproducibility_packet_ids":["RPK_COSMO"],"description":"benchmark receipts attached","blocking":true}
adequacy {"id":"ADQ_COSMO_STRESS","kind":"StressInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_COSMO"],"burden_pack_ids":["BPK_COSMO"],"claim_packet_ids":["CLM_COSMO"],"evidence_contract_ids":["ECT_COSMO"],"benchmark_receipt_ids":["BMR_COSMO_STRESS"],"challenge_receipt_ids":[],"reproducibility_packet_ids":["RPK_COSMO"],"description":"stress benchmark attached","blocking":true}
adequacy {"id":"ADQ_COSMO_CHALLENGE","kind":"ChallengeInterpretation","regime_ids":["R_TOP","R_CALC"],"bridge_ids":[],"theorem_ids":["THS_COSMO"],"burden_pack_ids":["BPK_COSMO"],"claim_packet_ids":["CLM_COSMO"],"evidence_contract_ids":["ECT_COSMO"],"benchmark_receipt_ids":[],"challenge_receipt_ids":["CHR_COSMO"],"reproducibility_packet_ids":["RPK_COSMO"],"description":"challenge record attached","blocking":true}
```

Run it:

```powershell
l64 compile-bundle .\cosmo.locus.rna --out .\cosmo.dna
l64 certify-bundle --file .\cosmo.dna --conflict-policy exact-match
```

Then inspect the generated report id:

```powershell
l64 observe-run --report REPORT_THS_COSMO_CPG_COSMO
l64 research-derive-from-report --report-id REPORT_THS_COSMO_CPG_COSMO --persist
l64 research-promotion-readiness REPORT_THS_COSMO_CPG_COSMO
```

## 7. ChatGPT Output Contract

When asking ChatGPT to generate something for Locus64, ask for exactly one of:

```text
1. A .gene.rna file body.
2. A complete bundle-entry text body that can be compiled with `l64 compile-bundle`.
3. A single bundle entry line.
4. A patch to an existing bundle-entry text body.
```

For semantic claims, prefer:

```text
Generate a complete Locus64 bundle-entry text body.
Do not include comments.
Use one JSON object per line.
Include claim-packet, evidence-contract, benchmark-receipt, challenge-receipt, reproducibility-packet, adequacy, theorem, obligation, target, ledger, campaign, bridge, proof, and atlas entries.
```

Do not ask ChatGPT for “Locus64 prose.” Ask for valid bundle entry lines, compile them with `l64 compile-bundle`, and certify the resulting `.dna`.

## 8. Validation Loop

After generating bundle entry text:

```powershell
l64 compile-bundle .\file.locus.rna --out .\file.dna
l64 certify-bundle --file .\file.dna --conflict-policy exact-match
l64 import-bundle .\file.dna --conflict-policy exact-match
```

If validation fails:

- fix projection syntax first
- then regenerate the `.dna`
- then re-run certification/import checks
- then fix missing referenced ids
- then fix adequacy/evidence gaps

## 9. Common Failure Causes

- Missing `!l64-bundle v1` header.
- Multi-line JSON payloads.
- Wrong enum casing, e.g. `certified` instead of `Certified`.
- Referencing an id that has no entry.
- Claim has `requires_stress: true` but no `Stress` benchmark receipt.
- Claim has `requires_challenge: true` but no addressed/open challenge receipt.
- `metrics` values are not strings.
- Comments inserted into projection entry files.

## 10. Practical Rule

Use:

- RNA for canonical symbolic roots.
- QC0 for semantic/certification/governance records.
- DNA as generated machine artifact, not hand-authored input.

For indirect integration, the main language you need is **QC0**.
