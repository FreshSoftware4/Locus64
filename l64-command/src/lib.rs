use clap::ValueEnum;
use l64_core::{BundleConflictPolicy, OptimizerPolicy};

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
