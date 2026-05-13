use mf_core::{CanonId, EqClassId};
use mf_registry::SeedRegistry;
use mf_runtime::{RuntimeObjectId, RuntimeWorld};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CanonError {
    #[error("unknown runtime object")]
    UnknownObject,
}

#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, value: usize) -> usize {
        if self.parent[value] != value {
            let root = self.find(self.parent[value]);
            self.parent[value] = root;
        }
        self.parent[value]
    }

    pub fn union(&mut self, left: usize, right: usize) {
        let left_root = self.find(left);
        let right_root = self.find(right);
        if left_root == right_root {
            return;
        }
        if self.rank[left_root] < self.rank[right_root] {
            self.parent[left_root] = right_root;
        } else if self.rank[left_root] > self.rank[right_root] {
            self.parent[right_root] = left_root;
        } else {
            self.parent[right_root] = left_root;
            self.rank[left_root] += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct CanonEngine {
    world: RuntimeWorld,
    union_find: UnionFind,
    canonical_map: HashMap<usize, CanonId>,
    eq_class_map: HashMap<usize, EqClassId>,
}

impl CanonEngine {
    pub fn compile(registry: &SeedRegistry) -> Self {
        let world = RuntimeWorld::compile_seed(registry);
        let mut union_find = UnionFind::new(world.objects.len());

        for class in &registry.bundle().equivalence_classes {
            if let Some((first, rest)) = class.members.split_first() {
                if let Some(first_id) = world.lookup_object(first) {
                    for member in rest {
                        if let Some(member_id) = world.lookup_object(member) {
                            union_find.union(first_id.0 as usize, member_id.0 as usize);
                        }
                    }
                }
            }
        }

        let mut canonical_map = HashMap::new();
        let mut eq_class_map = HashMap::new();
        let mut next_canon: u32 = 0;
        let mut next_eq: u32 = 0;
        for index in 0..world.objects.len() {
            let root = union_find.find(index);
            canonical_map.entry(root).or_insert_with(|| {
                let id: CanonId = next_canon;
                next_canon += 1;
                id
            });
            eq_class_map.entry(root).or_insert_with(|| {
                let id = next_eq;
                next_eq += 1;
                id
            });
        }

        Self {
            world,
            union_find,
            canonical_map,
            eq_class_map,
        }
    }

    pub fn equivalent(&mut self, left: RuntimeObjectId, right: RuntimeObjectId) -> bool {
        self.union_find.find(left.0 as usize) == self.union_find.find(right.0 as usize)
    }

    pub fn canonicalize(&mut self, object: RuntimeObjectId) -> Result<CanonId, CanonError> {
        let root = self.union_find.find(object.0 as usize);
        self.canonical_map
            .get(&root)
            .copied()
            .ok_or(CanonError::UnknownObject)
    }

    pub fn eq_class(&mut self, object: RuntimeObjectId) -> Result<EqClassId, CanonError> {
        let root = self.union_find.find(object.0 as usize);
        self.eq_class_map
            .get(&root)
            .copied()
            .ok_or(CanonError::UnknownObject)
    }

    pub fn world(&self) -> &RuntimeWorld {
        &self.world
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_equivalence_classes_canonicalize() {
        let registry = SeedRegistry::load().unwrap();
        let mut engine = CanonEngine::compile(&registry);
        let left = engine.world().lookup_object("OBJ_CTX_SET").unwrap();
        let right = engine.world().lookup_object("OBJ_CTX_SET_CANON").unwrap();
        assert!(engine.equivalent(left, right));
        assert_eq!(
            engine.canonicalize(left).unwrap(),
            engine.canonicalize(right).unwrap()
        );
    }
}
