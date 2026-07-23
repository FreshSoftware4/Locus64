use crate::Graph;

const CODEC_VERSION: u16 = 1;

pub fn canonical_bytes(graph: &Graph) -> Vec<u8> {
    let mut out =
        Vec::with_capacity(16 + graph.nodes_raw().len() * 24 + graph.ports_raw().len() * 8);
    out.extend_from_slice(b"L64N");
    out.extend_from_slice(&CODEC_VERSION.to_le_bytes());
    push_len(&mut out, graph.nodes_raw().len());
    push_len(&mut out, graph.ports_raw().len());
    push_len(&mut out, graph.contexts_raw().len());
    push_len(&mut out, graph.routes_raw().len());

    for node in graph.nodes_raw() {
        out.extend_from_slice(&(node.opcode() as u16).to_le_bytes());
        out.extend_from_slice(&node.context().to_le_bytes());
        out.extend_from_slice(&node.ty().unwrap_or(u32::MAX).to_le_bytes());
        out.extend_from_slice(&node.payload().to_le_bytes());
        let range = node.port_range();
        out.extend_from_slice(&(range.start as u32).to_le_bytes());
        out.extend_from_slice(&((range.end - range.start) as u16).to_le_bytes());
    }

    for port in graph.ports_raw() {
        out.extend_from_slice(&port.target().to_le_bytes());
        out.push(port.role() as u8);
        out.push(0);
        out.extend_from_slice(&port.ordinal().to_le_bytes());
    }

    for context in graph.contexts_raw() {
        out.extend_from_slice(&context.parent().to_le_bytes());
        out.extend_from_slice(&context.binding().unwrap_or(u32::MAX).to_le_bytes());
    }

    for (route, node) in graph.routes_raw() {
        out.extend_from_slice(&route.domain().0.to_le_bytes());
        push_len(&mut out, route.tail().len());
        for word in route.tail() {
            out.extend_from_slice(&word.0.to_le_bytes());
        }
        out.extend_from_slice(&node.to_le_bytes());
    }

    out
}

pub(crate) fn state_commitment(graph: &Graph) -> [u8; 32] {
    // This dependency-free structural stamp is deterministic and adequate for
    // transactional change detection. DNA v2 must replace it with the
    // repository's domain-separated cryptographic commitment boundary.
    let bytes = canonical_bytes(graph);
    let seeds = [
        0xcbf29ce484222325_u64,
        0x84222325cbf29ce4_u64,
        0x9e3779b97f4a7c15_u64,
        0xd6e8feb86659fd93_u64,
    ];
    let mut lanes = seeds;
    for (index, byte) in bytes.iter().copied().enumerate() {
        for (lane_index, lane) in lanes.iter_mut().enumerate() {
            let rotated = byte.rotate_left(((index + lane_index) & 7) as u32);
            *lane ^= u64::from(rotated) | ((index as u64) << ((lane_index + 1) * 7));
            *lane = lane.wrapping_mul(0x100000001b3_u64 ^ seeds[lane_index]);
            *lane ^= *lane >> 29;
            *lane = lane.rotate_left((11 + lane_index * 7) as u32);
        }
    }
    let mut out = [0_u8; 32];
    for (index, mut lane) in lanes.into_iter().enumerate() {
        lane ^= lane >> 33;
        lane = lane.wrapping_mul(0xff51afd7ed558ccd);
        lane ^= lane >> 33;
        lane = lane.wrapping_mul(0xc4ceb9fe1a85ec53);
        lane ^= lane >> 33;
        out[index * 8..(index + 1) * 8].copy_from_slice(&lane.to_le_bytes());
    }
    out
}

fn push_len(out: &mut Vec<u8>, len: usize) {
    let value = u32::try_from(len).expect("native graph section exceeds u32 length");
    out.extend_from_slice(&value.to_le_bytes());
}
