use l64_native::{Dimension, Graph, LocusWord, Proposal, ROOT_CONTEXT, Route};
use l64_projection::ProjectionSet;

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x50524f4a45435431)).composed(LocusWord(index))
}

fn main() {
    let mut graph = Graph::new();
    let scalar = graph.declare_atom_type(route(1), LocusWord(0x52)).unwrap();
    let quantity = graph
        .declare_quantity_type(route(2), scalar, Dimension::new([0, 0, 0, 0, 0, 0, 0]))
        .unwrap();
    let value = graph
        .insert_value(route(3), ROOT_CONTEXT, quantity)
        .unwrap();
    graph
        .transact(Proposal::square_root(
            route(4),
            ROOT_CONTEXT,
            value,
            quantity,
        ))
        .unwrap();

    let projection = ProjectionSet::derive(&graph, ROOT_CONTEXT, 8).unwrap();
    projection.verify(&graph).unwrap();
    print!("{}", projection.render_text());
}
