use clap::ValueEnum;
use mf_core::{BundleConflictPolicy, OptimizerPolicy, SurfaceKind};

#[derive(Debug, Clone, ValueEnum)]
pub enum SurfaceArg {
    Qc0,
    Qm0,
    Qk0,
    Qa0,
}

impl From<SurfaceArg> for SurfaceKind {
    fn from(value: SurfaceArg) -> Self {
        match value {
            SurfaceArg::Qc0 => SurfaceKind::Qc0,
            SurfaceArg::Qm0 => SurfaceKind::Qm0,
            SurfaceArg::Qk0 => SurfaceKind::Qk0,
            SurfaceArg::Qa0 => SurfaceKind::Qa0,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AdminSurfaceArg {
    Qc0,
    Qm0,
    Qa0,
}

impl From<AdminSurfaceArg> for SurfaceKind {
    fn from(value: AdminSurfaceArg) -> Self {
        match value {
            AdminSurfaceArg::Qc0 => SurfaceKind::Qc0,
            AdminSurfaceArg::Qm0 => SurfaceKind::Qm0,
            AdminSurfaceArg::Qa0 => SurfaceKind::Qa0,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum BundlePolicyArg {
    ExactMatch,
    Shadow,
    Reject,
    NamespacedImport,
}

impl From<BundlePolicyArg> for BundleConflictPolicy {
    fn from(value: BundlePolicyArg) -> Self {
        match value {
            BundlePolicyArg::ExactMatch => BundleConflictPolicy::ExactMatch,
            BundlePolicyArg::Shadow => BundleConflictPolicy::Shadow,
            BundlePolicyArg::Reject => BundleConflictPolicy::Reject,
            BundlePolicyArg::NamespacedImport => BundleConflictPolicy::NamespacedImport,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OptimizerPolicyArg {
    Conservative,
    SymbolicFidelityFirst,
    ExecutionFirst,
    LowLoss,
    BenchmarkFriendly,
}

impl From<OptimizerPolicyArg> for OptimizerPolicy {
    fn from(value: OptimizerPolicyArg) -> Self {
        match value {
            OptimizerPolicyArg::Conservative => OptimizerPolicy::Conservative,
            OptimizerPolicyArg::SymbolicFidelityFirst => OptimizerPolicy::SymbolicFidelityFirst,
            OptimizerPolicyArg::ExecutionFirst => OptimizerPolicy::ExecutionFirst,
            OptimizerPolicyArg::LowLoss => OptimizerPolicy::LowLoss,
            OptimizerPolicyArg::BenchmarkFriendly => OptimizerPolicy::BenchmarkFriendly,
        }
    }
}
