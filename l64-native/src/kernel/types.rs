pub(crate) const JUDGMENT_LOCUS: LocusWord = LocusWord(u64::MAX - 1);
pub(crate) const EVIDENCE_LOCUS: LocusWord = LocusWord(u64::MAX);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum OpCode {
    TypeAtom = 1,
    TypeMatrix = 2,
    TypeFunction = 3,
    Value = 4,
    Compose = 5,
    MatMul = 6,
    ExtendContext = 7,
    TypeJudgment = 8,
    KernelWitness = 9,
    TypeQuantity = 10,
    Add = 11,
    Multiply = 12,
    Divide = 13,
    Constraint = 14,
    Sqrt = 15,
    Obligation = 16,
}

impl OpCode {
    pub(crate) fn from_raw(value: u16) -> Option<Self> {
        match value {
            1 => Some(Self::TypeAtom),
            2 => Some(Self::TypeMatrix),
            3 => Some(Self::TypeFunction),
            4 => Some(Self::Value),
            5 => Some(Self::Compose),
            6 => Some(Self::MatMul),
            7 => Some(Self::ExtendContext),
            8 => Some(Self::TypeJudgment),
            9 => Some(Self::KernelWitness),
            10 => Some(Self::TypeQuantity),
            11 => Some(Self::Add),
            12 => Some(Self::Multiply),
            13 => Some(Self::Divide),
            14 => Some(Self::Constraint),
            15 => Some(Self::Sqrt),
            16 => Some(Self::Obligation),
            _ => None,
        }
    }

    pub(crate) fn is_type(self) -> bool {
        matches!(
            self,
            Self::TypeAtom
                | Self::TypeMatrix
                | Self::TypeFunction
                | Self::TypeJudgment
                | Self::TypeQuantity
        )
    }

    pub(crate) fn is_untyped_authority(self) -> bool {
        self.is_type() || self == Self::Constraint
    }

    pub(crate) fn is_persisted_node(self) -> bool {
        self != Self::ExtendContext
    }

    pub(crate) fn is_executable(self) -> bool {
        matches!(
            self,
            Self::Compose | Self::MatMul | Self::Add | Self::Multiply | Self::Divide | Self::Sqrt
        )
    }

    pub(crate) fn arity(self) -> Option<u16> {
        match self {
            Self::Compose | Self::MatMul | Self::Add | Self::Multiply | Self::Divide => Some(2),
            Self::Sqrt => Some(1),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConstraintKind {
    NonNegative = 1,
}

impl ConstraintKind {
    pub(crate) fn from_raw(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::NonNegative),
            _ => None,
        }
    }

    pub(crate) fn payload(self, holds: bool) -> u64 {
        self as u64 | (u64::from(holds) << 8)
    }

    pub(crate) fn from_payload(payload: u64) -> Option<(Self, bool)> {
        if payload >> 9 != 0 {
            return None;
        }
        let kind = Self::from_raw(payload as u8)?;
        let truth = ((payload >> 8) & 1) != 0;
        Some((kind, truth))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConstraintState {
    Proven,
    Refuted,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EvidencePlan {
    Witness,
    Obligation {
        kind: ConstraintKind,
        subject: NodeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PortRole {
    Parameter = 1,
    Domain = 2,
    Codomain = 3,
    Argument = 4,
    Subject = 5,
    Premise = 6,
    Conclusion = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Port {
    target: NodeId,
    role: u8,
    flags: u8,
    ordinal: u16,
}

impl PortRole {
    pub(crate) fn from_raw(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Parameter),
            2 => Some(Self::Domain),
            3 => Some(Self::Codomain),
            4 => Some(Self::Argument),
            5 => Some(Self::Subject),
            6 => Some(Self::Premise),
            7 => Some(Self::Conclusion),
            _ => None,
        }
    }
}

impl Port {
    pub(crate) fn new(target: NodeId, role: PortRole, ordinal: u16) -> Self {
        Self {
            target,
            role: role as u8,
            flags: 0,
            ordinal,
        }
    }

    pub(crate) fn from_raw(target: NodeId, role: PortRole, flags: u8, ordinal: u16) -> Self {
        Self {
            target,
            role: role as u8,
            flags,
            ordinal,
        }
    }

    pub fn target(&self) -> NodeId {
        self.target
    }

    pub fn role(&self) -> PortRole {
        PortRole::from_raw(self.role).expect("stored port role is validated at insertion")
    }

    pub fn ordinal(&self) -> u16 {
        self.ordinal
    }

    pub(crate) fn flags(&self) -> u8 {
        self.flags
    }
}
