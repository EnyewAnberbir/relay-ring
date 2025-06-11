//! Integration test for `RR-0883` (stability).
//! Extended: Gate surfaces seal index refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0883_gate_surfaces_seal_index_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x7e];
    let full = relayring::capabilities::rr_0883_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0883: bulk Extended: Gate surfaces seal index refactor mutator v18");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0883_gate_surfaces_seal_index_extended::evaluate(&fixture[..end]).expect("RR-0883: stable prefix");
        assert!(partial.consumed <= end, "RR-0883: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0883: full prefix should match bulk checksum");
        }
    }
}
