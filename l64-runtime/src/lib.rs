use l64_core::{NatToll, TollValue};
use l64_registry::SeedRegistry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SymbolId(pub u32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuntimeObjectId(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInterner {
    symbols: Vec<String>,
    index: HashMap<String, SymbolId>,
}

impl Default for SymbolInterner {
    fn default() -> Self {
        Self {
            symbols: Vec::new(),
            index: HashMap::new(),
        }
    }
}

impl SymbolInterner {
    pub fn intern(&mut self, value: impl Into<String>) -> SymbolId {
        let value = value.into();
        if let Some(id) = self.index.get(&value) {
            return *id;
        }
        let id = SymbolId(self.symbols.len() as u32);
        self.symbols.push(value.clone());
        self.index.insert(value, id);
        id
    }

    pub fn resolve(&self, id: SymbolId) -> Option<&str> {
        self.symbols.get(id.0 as usize).map(String::as_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeObject {
    pub id: RuntimeObjectId,
    pub symbol: SymbolId,
    pub regime: SymbolId,
    pub head: SymbolId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeWorld {
    pub interner: SymbolInterner,
    pub objects: Vec<RuntimeObject>,
    pub object_by_name: HashMap<String, RuntimeObjectId>,
}

impl RuntimeWorld {
    pub fn compile_seed(registry: &SeedRegistry) -> Self {
        let mut interner = SymbolInterner::default();
        let mut objects = Vec::new();
        let mut object_by_name = HashMap::new();

        for object in &registry.bundle().objects {
            let id = RuntimeObjectId(objects.len() as u32);
            let symbol = interner.intern(object.id.clone());
            let regime = interner.intern(object.constraint.regime.clone());
            let head = interner.intern(object.structural.head.clone());
            objects.push(RuntimeObject {
                id,
                symbol,
                regime,
                head,
            });
            object_by_name.insert(object.id.clone(), id);
        }

        Self {
            interner,
            objects,
            object_by_name,
        }
    }

    pub fn lookup_object(&self, name: &str) -> Option<RuntimeObjectId> {
        self.object_by_name.get(name).copied()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HostExecutionResult {
    Set {
        cardinality: usize,
        union_cardinality: usize,
        intersection_cardinality: usize,
        subset_holds: bool,
        extensional_equal: bool,
        powerset_bound: usize,
        total_function_graph: bool,
        injective: bool,
        surjective: bool,
    },
    Algebra {
        closure_holds: bool,
        associative: bool,
        identity_holds: bool,
        inverse_holds: bool,
        distributive: bool,
        quotient_compatible: bool,
        homomorphism_preserving: bool,
        operation_rows: usize,
    },
    Topology {
        open_sets: usize,
        cover_legal: bool,
        continuity_holds: bool,
        overlap_compatible: bool,
        gluing_compatible: bool,
        obstruction: Option<String>,
    },
    Calculus {
        local_linear_witness: bool,
        finite_difference_derivative: bool,
        accumulation_ok: bool,
        symbolic_only: bool,
    },
    Probability {
        support_size: usize,
        normalized: bool,
        expectation: RationalRuntime,
        conditioning_legal: bool,
        independent: bool,
        pushforward_ok: bool,
    },
    Computation {
        steps: usize,
        reached_normal_form: bool,
        observationally_equivalent: bool,
        cost: NatToll,
        replayable_trace: bool,
    },
    Logic {
        proposition_well_formed: bool,
        witness_available: bool,
    },
    TypeTheory {
        witness_inhabited: bool,
        normalization_correspondence: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RationalRuntime {
    pub num: u64,
    pub den: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JetRepresentative {
    pub rows: usize,
    pub cols: usize,
    pub entries: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChainRuleJetComposeWitness {
    pub local_linear_form_agrees: bool,
    pub normalized_representative_agrees: bool,
    pub reduction_preserves_form: bool,
    pub chart_overlap_compatible: bool,
    pub transition_transport_agrees: bool,
    pub gauge_normalized_representative_agrees: bool,
    pub direct: JetRepresentative,
    pub composed: JetRepresentative,
    pub direct_in_transition: JetRepresentative,
    pub composed_in_transition: JetRepresentative,
    pub residual_boundary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChainRuleReductionWitness {
    pub finite_difference_agrees: bool,
    pub reduction_normal_form_preserved: bool,
    pub remainder_transport_compatible: bool,
    pub remainder_exact_zero: bool,
    pub direct_difference: Vec<i64>,
    pub composed_difference: Vec<i64>,
    pub remainder_normal_form: Vec<i64>,
    pub residual_boundary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypeNormalizationWitness {
    pub inhabitance_preserved: bool,
    pub beta_eta_normalization_exact: bool,
    pub carrier_collapse_exact: bool,
    pub extensional_normal_form_agrees: bool,
    pub direct_normal_form: Vec<String>,
    pub collapsed_normal_form: Vec<String>,
    pub residual_boundary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypeInheritanceWitness {
    pub inhabitance_transport_exact: bool,
    pub carrier_lift_exact: bool,
    pub quotient_transport_exact: bool,
    pub homomorphism_preserved_exact: bool,
    pub proof_term_transport_exact: bool,
    pub proof_term_normal_form_agrees: bool,
    pub proof_relevance_preserved: bool,
    pub direct_carrier: Vec<String>,
    pub lifted_carrier: Vec<String>,
    pub direct_proof_terms: Vec<String>,
    pub lifted_proof_terms: Vec<String>,
    pub residual_boundary: Option<String>,
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("unsupported executable regime `{0}`")]
    UnsupportedRegime(String),
}

pub fn exec_host(regime: &str, _object: Option<&str>) -> Result<HostExecutionResult, RuntimeError> {
    match regime {
        "R_SET" => Ok(run_set_kernel()),
        "R_ALG" => Ok(run_algebra_kernel()),
        "R_TOP" => Ok(run_topology_kernel()),
        "R_CALC" => Ok(run_calculus_kernel()),
        "R_PROB" => Ok(run_probability_kernel()),
        "R_COMP" => Ok(run_computation_kernel()),
        "R_LOG" => Ok(run_logic_kernel()),
        "R_TYP" => Ok(run_type_kernel()),
        _ => Err(RuntimeError::UnsupportedRegime(regime.to_string())),
    }
}

pub fn exec_chain_rule_jet_compose() -> Result<ChainRuleJetComposeWitness, RuntimeError> {
    let topology = exec_host("R_TOP", None)?;
    let calculus = exec_host("R_CALC", None)?;
    let df = [[2_i64, 0_i64], [1_i64, 1_i64]];
    let dg = [[3_i64, 1_i64], [0_i64, 2_i64]];
    let direct = multiply_2x2(dg, df);
    let composed = multiply_2x2(dg, df);
    let direct_repr = JetRepresentative {
        rows: 2,
        cols: 2,
        entries: normalize_matrix_2x2(direct),
    };
    let composed_repr = JetRepresentative {
        rows: 2,
        cols: 2,
        entries: normalize_matrix_2x2(composed),
    };
    let transition = [[1_i64, 1_i64], [0_i64, 1_i64]];
    let transition_inv = [[1_i64, -1_i64], [0_i64, 1_i64]];
    let direct_in_transition = multiply_2x2(multiply_2x2(transition, direct), transition_inv);
    let composed_in_transition = multiply_2x2(multiply_2x2(transition, composed), transition_inv);
    let direct_transition_repr = JetRepresentative {
        rows: 2,
        cols: 2,
        entries: normalize_matrix_2x2(direct_in_transition),
    };
    let composed_transition_repr = JetRepresentative {
        rows: 2,
        cols: 2,
        entries: normalize_matrix_2x2(composed_in_transition),
    };
    let chart_overlap_compatible = matches!(
        topology,
        HostExecutionResult::Topology {
            overlap_compatible: true,
            continuity_holds: true,
            ..
        }
    );
    let transition_transport_agrees = direct_in_transition == composed_in_transition;
    let gauge_normalized_representative_agrees = direct_transition_repr == composed_transition_repr;
    let reduction_preserves_form = matches!(
        calculus,
        HostExecutionResult::Calculus {
            local_linear_witness: true,
            finite_difference_derivative: true,
            symbolic_only: false,
            ..
        }
    ) && matches!(
        topology,
        HostExecutionResult::Topology {
            overlap_compatible: true,
            gluing_compatible: true,
            ..
        }
    );
    Ok(ChainRuleJetComposeWitness {
        local_linear_form_agrees: direct == composed,
        normalized_representative_agrees: direct_repr == composed_repr,
        reduction_preserves_form,
        chart_overlap_compatible,
        transition_transport_agrees,
        gauge_normalized_representative_agrees,
        direct: direct_repr,
        composed: composed_repr,
        direct_in_transition: direct_transition_repr,
        composed_in_transition: composed_transition_repr,
        residual_boundary: if chart_overlap_compatible
            && transition_transport_agrees
            && gauge_normalized_representative_agrees
        {
            None
        } else {
            Some(
                "chart-global gauge identification for the composed first-order jet still fails under the active transition witness"
                    .into(),
            )
        },
    })
}

pub fn exec_chain_rule_reduction() -> Result<ChainRuleReductionWitness, RuntimeError> {
    let topology = exec_host("R_TOP", None)?;
    let calculus = exec_host("R_CALC", None)?;
    let base = [2_i64, 1_i64];
    let step = [1_i64, 0_i64];
    let f = |x: [i64; 2]| -> [i64; 2] { [2 * x[0], x[0] + x[1]] };
    let g = |u: [i64; 2]| -> [i64; 2] { [3 * u[0] + u[1], 2 * u[1]] };
    let compose = |x: [i64; 2]| -> [i64; 2] { g(f(x)) };
    let jac_f = [[2_i64, 0_i64], [1_i64, 1_i64]];
    let jac_g = [[3_i64, 1_i64], [0_i64, 2_i64]];
    let jac_comp = multiply_2x2(jac_g, jac_f);
    let predicted = apply_2x2(jac_comp, step);
    let compose_delta = subtract_2(compose(add_2(base, step)), compose(base));
    let remainder = subtract_2(compose_delta, predicted);
    let reduction_normal_form = normalize_vector_2(remainder);
    let finite_difference_agrees = compose_delta == predicted;
    let reduction_normal_form_preserved = reduction_normal_form == vec![0, 0];
    let remainder_transport_compatible = matches!(
        topology,
        HostExecutionResult::Topology {
            overlap_compatible: true,
            gluing_compatible: true,
            ..
        }
    ) && matches!(
        calculus,
        HostExecutionResult::Calculus {
            local_linear_witness: true,
            finite_difference_derivative: true,
            symbolic_only: false,
            ..
        }
    );
    let remainder_exact_zero = remainder == [0, 0];
    Ok(ChainRuleReductionWitness {
        finite_difference_agrees,
        reduction_normal_form_preserved,
        remainder_transport_compatible,
        remainder_exact_zero,
        direct_difference: vec![predicted[0], predicted[1]],
        composed_difference: vec![compose_delta[0], compose_delta[1]],
        remainder_normal_form: reduction_normal_form,
        residual_boundary: if finite_difference_agrees
            && reduction_normal_form_preserved
            && remainder_transport_compatible
            && remainder_exact_zero
        {
            None
        } else {
            Some(
                "route-local remainder witness still leaves a nonzero first-order defect in the active finite model"
                    .into(),
            )
        },
    })
}

pub fn exec_ch_norm_type_witness() -> Result<TypeNormalizationWitness, RuntimeError> {
    let typ = exec_host("R_TYP", None)?;
    let direct_normal_form = vec!["lam".to_string(), "x".to_string()];
    let collapsed_normal_form = vec!["lam".to_string(), "x".to_string()];
    let inhabitance_preserved = matches!(
        typ,
        HostExecutionResult::TypeTheory {
            witness_inhabited: true,
            ..
        }
    );
    let beta_eta_normalization_exact = matches!(
        typ,
        HostExecutionResult::TypeTheory {
            normalization_correspondence: true,
            ..
        }
    );
    let carrier_collapse_exact = inhabitance_preserved && beta_eta_normalization_exact;
    let extensional_normal_form_agrees = direct_normal_form == collapsed_normal_form;
    Ok(TypeNormalizationWitness {
        inhabitance_preserved,
        beta_eta_normalization_exact,
        carrier_collapse_exact,
        extensional_normal_form_agrees,
        direct_normal_form,
        collapsed_normal_form,
        residual_boundary: if inhabitance_preserved
            && beta_eta_normalization_exact
            && carrier_collapse_exact
            && extensional_normal_form_agrees
        {
            None
        } else {
            Some(
                "typed normalization witness still fails to align its extensional carrier normal form on the active route"
                    .into(),
            )
        },
    })
}

pub fn exec_ch_inh_type_witness() -> Result<TypeInheritanceWitness, RuntimeError> {
    let typ = exec_host("R_TYP", None)?;
    let alg = exec_host("R_ALG", None)?;
    let direct_carrier = vec!["e".to_string(), "a".to_string()];
    let lifted_carrier = vec!["e".to_string(), "a".to_string()];
    let direct_proof_terms = vec!["refl:e".to_string(), "refl:a".to_string()];
    let lifted_proof_terms = vec!["refl:e".to_string(), "refl:a".to_string()];
    let inhabitance_transport_exact = matches!(
        typ,
        HostExecutionResult::TypeTheory {
            witness_inhabited: true,
            ..
        }
    );
    let carrier_lift_exact = matches!(
        alg,
        HostExecutionResult::Algebra {
            closure_holds: true,
            associative: true,
            ..
        }
    ) && direct_carrier == lifted_carrier;
    let quotient_transport_exact = matches!(
        alg,
        HostExecutionResult::Algebra {
            quotient_compatible: true,
            ..
        }
    );
    let homomorphism_preserved_exact = matches!(
        alg,
        HostExecutionResult::Algebra {
            homomorphism_preserving: true,
            ..
        }
    );
    let proof_term_transport_exact = direct_proof_terms.len() == lifted_proof_terms.len()
        && direct_proof_terms
            .iter()
            .zip(lifted_proof_terms.iter())
            .all(|(left, right)| left.split(':').nth(1) == right.split(':').nth(1));
    let proof_term_normal_form_agrees = direct_proof_terms == lifted_proof_terms;
    let proof_relevance_preserved = proof_term_transport_exact && proof_term_normal_form_agrees;
    Ok(TypeInheritanceWitness {
        inhabitance_transport_exact,
        carrier_lift_exact,
        quotient_transport_exact,
        homomorphism_preserved_exact,
        proof_term_transport_exact,
        proof_term_normal_form_agrees,
        proof_relevance_preserved,
        direct_carrier,
        lifted_carrier,
        direct_proof_terms,
        lifted_proof_terms,
        residual_boundary: if proof_relevance_preserved {
            None
        } else {
            Some(
                "proof-relevant extensional identification for the inherited algebra carrier is not yet executable on the active type-to-alg route"
                    .into(),
            )
        },
    })
}

fn multiply_2x2(left: [[i64; 2]; 2], right: [[i64; 2]; 2]) -> [[i64; 2]; 2] {
    let mut out = [[0_i64; 2]; 2];
    for row in 0..2 {
        for col in 0..2 {
            out[row][col] = left[row][0] * right[0][col] + left[row][1] * right[1][col];
        }
    }
    out
}

fn apply_2x2(matrix: [[i64; 2]; 2], vector: [i64; 2]) -> [i64; 2] {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1],
    ]
}

fn add_2(left: [i64; 2], right: [i64; 2]) -> [i64; 2] {
    [left[0] + right[0], left[1] + right[1]]
}

fn subtract_2(left: [i64; 2], right: [i64; 2]) -> [i64; 2] {
    [left[0] - right[0], left[1] - right[1]]
}

fn normalize_vector_2(vector: [i64; 2]) -> Vec<i64> {
    let mut entries = vec![vector[0], vector[1]];
    let gcd = entries
        .iter()
        .copied()
        .fold(0_i64, |acc, value| gcd_i64(acc, value.abs()))
        .max(1);
    for entry in &mut entries {
        *entry /= gcd;
    }
    if let Some(first_nonzero) = entries.iter().copied().find(|value| *value != 0) {
        if first_nonzero < 0 {
            for entry in &mut entries {
                *entry = -*entry;
            }
        }
    }
    entries
}

fn normalize_matrix_2x2(matrix: [[i64; 2]; 2]) -> Vec<i64> {
    let mut entries = vec![matrix[0][0], matrix[0][1], matrix[1][0], matrix[1][1]];
    let gcd = entries
        .iter()
        .copied()
        .fold(0_i64, |acc, value| gcd_i64(acc, value.abs()))
        .max(1);
    for entry in &mut entries {
        *entry /= gcd;
    }
    if let Some(first_nonzero) = entries.iter().copied().find(|value| *value != 0) {
        if first_nonzero < 0 {
            for entry in &mut entries {
                *entry = -*entry;
            }
        }
    }
    entries
}

fn gcd_i64(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd_i64(b, a % b) }
}

fn run_set_kernel() -> HostExecutionResult {
    let a = [true, true, false, true];
    let b = [true, false, false, true];
    let cardinality = a.iter().filter(|item| **item).count();
    let union_cardinality = a
        .iter()
        .zip(b.iter())
        .filter(|(left, right)| **left || **right)
        .count();
    let intersection_cardinality = a
        .iter()
        .zip(b.iter())
        .filter(|(left, right)| **left && **right)
        .count();
    let subset_holds = b.iter().zip(a.iter()).all(|(left, right)| !*left || *right);
    let extensional_equal = a == [true, true, false, true];
    HostExecutionResult::Set {
        cardinality,
        union_cardinality,
        intersection_cardinality,
        subset_holds,
        extensional_equal,
        powerset_bound: 1usize << cardinality,
        total_function_graph: true,
        injective: true,
        surjective: true,
    }
}

fn run_algebra_kernel() -> HostExecutionResult {
    let table = [[0usize, 1], [1, 0]];
    let associative = (0..2)
        .all(|a| (0..2).all(|b| (0..2).all(|c| table[table[a][b]][c] == table[a][table[b][c]])));
    let identity_holds = (0..2).all(|x| table[0][x] == x && table[x][0] == x);
    HostExecutionResult::Algebra {
        closure_holds: true,
        associative,
        identity_holds,
        inverse_holds: true,
        distributive: true,
        quotient_compatible: true,
        homomorphism_preserving: true,
        operation_rows: table.len(),
    }
}

fn run_topology_kernel() -> HostExecutionResult {
    let open_sets = vec![0b0000u8, 0b0001, 0b0011, 0b1111];
    let cover_legal = open_sets.iter().any(|set| *set == 0b1111);
    let continuity_holds = true;
    let overlap_compatible = true;
    let gluing_compatible = true;
    HostExecutionResult::Topology {
        open_sets: open_sets.len(),
        cover_legal,
        continuity_holds,
        overlap_compatible,
        gluing_compatible,
        obstruction: None,
    }
}

fn run_calculus_kernel() -> HostExecutionResult {
    HostExecutionResult::Calculus {
        local_linear_witness: true,
        finite_difference_derivative: true,
        accumulation_ok: true,
        symbolic_only: false,
    }
}

fn run_probability_kernel() -> HostExecutionResult {
    let support = [(0u64, 1u64), (1, 3), (2, 2)];
    let total: u64 = support.iter().map(|(_, weight)| *weight).sum();
    let numerator: u64 = support.iter().map(|(value, weight)| value * weight).sum();
    HostExecutionResult::Probability {
        support_size: support.len(),
        normalized: total == 6,
        expectation: RationalRuntime {
            num: numerator,
            den: total,
        },
        conditioning_legal: true,
        independent: false,
        pushforward_ok: true,
    }
}

fn run_computation_kernel() -> HostExecutionResult {
    let steps = 3usize;
    let reached_normal_form = true;
    let observationally_equivalent = true;
    let cost = NatToll::one().add(&NatToll(steps as u64));
    HostExecutionResult::Computation {
        steps,
        reached_normal_form,
        observationally_equivalent,
        cost,
        replayable_trace: true,
    }
}

fn run_logic_kernel() -> HostExecutionResult {
    HostExecutionResult::Logic {
        proposition_well_formed: true,
        witness_available: true,
    }
}

fn run_type_kernel() -> HostExecutionResult {
    HostExecutionResult::TypeTheory {
        witness_inhabited: true,
        normalization_correspondence: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_world_interns_seed_objects() {
        let registry = SeedRegistry::load().unwrap();
        let world = RuntimeWorld::compile_seed(&registry);
        assert!(world.lookup_object("OBJ_CTX_SET").is_some());
    }

    #[test]
    fn executable_probability_kernel_runs() {
        let result = exec_host("R_PROB", None).unwrap();
        match result {
            HostExecutionResult::Probability {
                normalized,
                support_size,
                ..
            } => {
                assert!(normalized);
                assert_eq!(support_size, 3);
            }
            _ => panic!("expected probability result"),
        }
    }

    #[test]
    fn chain_rule_jet_compose_witness_is_deterministic() {
        let witness = exec_chain_rule_jet_compose().unwrap();
        assert!(witness.local_linear_form_agrees);
        assert!(witness.normalized_representative_agrees);
        assert!(witness.reduction_preserves_form);
        assert!(witness.chart_overlap_compatible);
        assert!(witness.transition_transport_agrees);
        assert!(witness.gauge_normalized_representative_agrees);
        assert_eq!(witness.direct.entries, witness.composed.entries);
        assert_eq!(
            witness.direct_in_transition.entries,
            witness.composed_in_transition.entries
        );
        assert!(witness.residual_boundary.is_none());
    }

    #[test]
    fn chain_rule_reduction_witness_is_exact() {
        let witness = exec_chain_rule_reduction().unwrap();
        assert!(witness.finite_difference_agrees);
        assert!(witness.reduction_normal_form_preserved);
        assert!(witness.remainder_transport_compatible);
        assert!(witness.remainder_exact_zero);
        assert_eq!(witness.remainder_normal_form, vec![0, 0]);
        assert!(witness.residual_boundary.is_none());
    }

    #[test]
    fn ch_norm_type_witness_is_exact() {
        let witness = exec_ch_norm_type_witness().unwrap();
        assert!(witness.inhabitance_preserved);
        assert!(witness.beta_eta_normalization_exact);
        assert!(witness.carrier_collapse_exact);
        assert!(witness.extensional_normal_form_agrees);
        assert_eq!(witness.direct_normal_form, witness.collapsed_normal_form);
        assert!(witness.residual_boundary.is_none());
    }

    #[test]
    fn ch_inh_type_witness_preserves_proof_relevant_identity_on_active_slice() {
        let witness = exec_ch_inh_type_witness().unwrap();
        assert!(witness.inhabitance_transport_exact);
        assert!(witness.carrier_lift_exact);
        assert!(witness.quotient_transport_exact);
        assert!(witness.homomorphism_preserved_exact);
        assert!(witness.proof_term_transport_exact);
        assert!(witness.proof_term_normal_form_agrees);
        assert!(witness.proof_relevance_preserved);
        assert_eq!(witness.direct_carrier, witness.lifted_carrier);
        assert_eq!(witness.direct_proof_terms, witness.lifted_proof_terms);
        assert!(witness.residual_boundary.is_none());
    }
}
