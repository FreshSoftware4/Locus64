use l64_certification::CertificationVerdict;
use l64_native::rna_to_dna;
use l64_transport::{BundleError, bundle_bytes, certify_bundle, decode_bundle, execute_bundle};

const TRIANGLE: &[u8] = b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n";
const EQUALITY: &[u8] =
    b"L64R1 0x4551\na 1 0x41\na 2 0x41\ne 3 2 1 2 0\ne 4 3 2 1 0 3\ne 5 4 1 1 0 3 4\n";

#[test]
fn bundle_is_exact_ordered_dna_transport() {
    let first = rna_to_dna(TRIANGLE).unwrap();
    let second = rna_to_dna(EQUALITY).unwrap();
    let encoded = bundle_bytes(&[&first, &second]).unwrap();
    let decoded = decode_bundle(&encoded).unwrap();
    assert_eq!(decoded.len(), 2);
    assert_eq!(decoded.members()[0].dna(), first);
    assert_eq!(decoded.members()[1].dna(), second);
    assert_eq!(decoded.bytes(), encoded);
}

#[test]
fn execution_verifies_each_projection_without_composite_authority() {
    let first = rna_to_dna(TRIANGLE).unwrap();
    let second = rna_to_dna(EQUALITY).unwrap();
    let encoded = bundle_bytes(&[&first, &second]).unwrap();
    let execution = execute_bundle(&encoded).unwrap();
    assert_eq!(execution.members.len(), 2);
    let text = execution.render_text();
    assert!(text.contains("composite_authority=none"));
    assert!(text.contains("ordering=transport_sequence"));
    assert!(text.contains("member.0.source=bundle_member"));
    assert!(text.contains("member.1.source=bundle_member"));
    assert_eq!(text.matches("projection_verified=true").count(), 2);
    assert_eq!(text.matches("certification_verdict=CERTIFIED").count(), 2);
    assert!(text.contains("composite_execution=none"));
}

#[test]
fn bundle_certification_is_owned_by_transport_and_remains_member_local() {
    let first = rna_to_dna(TRIANGLE).unwrap();
    let second = rna_to_dna(EQUALITY).unwrap();
    let encoded = bundle_bytes(&[&first, &second]).unwrap();
    let certification = certify_bundle(&encoded).unwrap();
    assert_eq!(certification.members.len(), 2);
    assert!(
        certification
            .members
            .iter()
            .all(|member| member.verdict == CertificationVerdict::Certified)
    );
    let text = certification.render_text();
    assert!(text.contains("composite_certificate=none"));
    assert!(!text.contains("bundle_verdict="));
}

#[test]
fn bundle_rejects_empty_and_non_dna_members() {
    assert_eq!(bundle_bytes(&[]), Err(BundleError::Empty));
    assert!(matches!(
        bundle_bytes(&[b"not dna"]),
        Err(BundleError::Dna { index: 0, .. })
    ));
}

#[test]
fn bundle_rejects_trailing_or_truncated_bytes() {
    let dna = rna_to_dna(TRIANGLE).unwrap();
    let encoded = bundle_bytes(&[&dna]).unwrap();
    let mut trailing = encoded.clone();
    trailing.push(0);
    assert_eq!(
        decode_bundle(&trailing).unwrap_err(),
        BundleError::TrailingBytes
    );
    assert_eq!(
        decode_bundle(&encoded[..encoded.len() - 1]).unwrap_err(),
        BundleError::Truncated
    );
}

#[test]
fn streaming_encoder_matches_exact_transport_and_decoder_releases_members_sequentially() {
    use l64_transport::{BundleDecoder, BundleEncoder};
    use std::io::Cursor;

    let first = rna_to_dna(TRIANGLE).unwrap();
    let second = rna_to_dna(EQUALITY).unwrap();
    let expected = bundle_bytes(&[&first, &second]).unwrap();

    let mut encoded = Cursor::new(Vec::new());
    {
        let mut encoder = BundleEncoder::new(&mut encoded).unwrap();
        encoder.push_member(&first).unwrap();
        encoder.push_member(&second).unwrap();
        let summary = encoder.finish().unwrap();
        assert_eq!(summary.members, 2);
        assert_eq!(summary.bytes, expected.len());
    }
    assert_eq!(encoded.into_inner(), expected);

    let mut decoder = BundleDecoder::new(Cursor::new(expected)).unwrap();
    assert_eq!(decoder.member_count(), 2);
    let first_member = decoder.next_member().unwrap().unwrap();
    assert_eq!(first_member.index(), 0);
    assert_eq!(first_member.dna(), first);
    drop(first_member);
    let second_member = decoder.next_member().unwrap().unwrap();
    assert_eq!(second_member.index(), 1);
    assert_eq!(second_member.dna(), second);
    drop(second_member);
    assert!(decoder.next_member().unwrap().is_none());
}

#[test]
fn streaming_decoder_never_requests_the_whole_bundle_buffer() {
    use l64_transport::{BundleDecoder, bundle_bytes};
    use std::{cell::Cell, io::Read, rc::Rc};

    struct TrackingReader {
        bytes: std::io::Cursor<Vec<u8>>,
        max_request: Rc<Cell<usize>>,
    }

    impl Read for TrackingReader {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            self.max_request
                .set(self.max_request.get().max(buffer.len()));
            self.bytes.read(buffer)
        }
    }

    let dna = rna_to_dna(TRIANGLE).unwrap();
    let members = (0..128).map(|_| dna.as_slice()).collect::<Vec<_>>();
    let encoded = bundle_bytes(&members).unwrap();
    let max_request = Rc::new(Cell::new(0));
    let reader = TrackingReader {
        bytes: std::io::Cursor::new(encoded.clone()),
        max_request: Rc::clone(&max_request),
    };
    let mut decoder = BundleDecoder::new(reader).unwrap();
    let mut count = 0;
    while decoder.next_member().unwrap().is_some() {
        count += 1;
    }
    assert_eq!(count, 128);
    assert!(max_request.get() <= dna.len());
    assert!(max_request.get() < encoded.len() / 64);
}
