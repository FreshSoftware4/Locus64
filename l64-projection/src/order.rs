use l64_native::ClosureState;

pub(crate) fn closure_rank(state: ClosureState) -> u8 {
    match state {
        ClosureState::Invalid => 0,
        ClosureState::Open => 1,
        ClosureState::Closed => 2,
    }
}
