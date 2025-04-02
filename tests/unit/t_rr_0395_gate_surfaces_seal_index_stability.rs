//! Integration test for `RR-0395` (stability).
//! Gate surfaces seal index implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0395_gate_surfaces_seal_index_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x92];
    let full = relayring::capabilities::rr_0395_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0395: bulk Gate surfaces seal index implement pipeline v30");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0395_gate_surfaces_seal_index::evaluate(&fixture[..end]).expect("RR-0395: stable prefix");
        assert!(partial.consumed <= end, "RR-0395: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0395: full prefix should match bulk checksum");
        }
    }
}
