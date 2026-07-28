use core::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LocusWord(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Route {
    domain: LocusWord,
    tail: Box<[LocusWord]>,
}

impl Route {
    pub fn root(domain: LocusWord) -> Self {
        Self {
            domain,
            tail: Box::new([]),
        }
    }

    pub fn from_parts(domain: LocusWord, tail: impl Into<Box<[LocusWord]>>) -> Self {
        Self {
            domain,
            tail: tail.into(),
        }
    }

    pub fn domain(&self) -> LocusWord {
        self.domain
    }

    pub fn tail(&self) -> &[LocusWord] {
        &self.tail
    }

    pub fn composed(&self, next: LocusWord) -> Self {
        let mut tail = Vec::with_capacity(self.tail.len() + 1);
        tail.extend_from_slice(&self.tail);
        tail.push(next);
        Self {
            domain: self.domain,
            tail: tail.into_boxed_slice(),
        }
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        self.domain
            .cmp(&other.domain)
            .then_with(|| self.tail.as_ref().cmp(other.tail.as_ref()))
    }
}
