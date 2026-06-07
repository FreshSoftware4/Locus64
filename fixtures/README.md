# Locus64 Fixtures

Fixtures are controlled test inputs, not authority by themselves.

## `cki_registry.genome.rna`

Classification: fixture candidate promoted to controlled regression fixture.

Role:

- exercises a nontrivial authored RNA source through the public RNA/DNA membrane
- proves `RNA -> DNA -> canonical RNA -> DNA` fixed-point stability over a dependency-heavy external proving slice
- remains source input only; generated DNA, products, reports, and views must be derived by commands during tests or release generation

Restrictions:

- do not treat this file as semantic authority outside the RNA/DNA pipeline
- do not hand-edit generated outputs into fixtures without a matching command-level reproduction test
- do not use this fixture to justify new public surfaces
