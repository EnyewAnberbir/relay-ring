//! Integration test for `RR-0369` (stability).
//! Gate surfaces seal index optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0369_gate_surfaces_seal_index_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x76, 0x78];
    let full = relayring::capabilities::rr_0369_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0369: bulk Gate surfaces seal index optimize registry v4");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0369_gate_surfaces_seal_index::evaluate(&fixture[..end]).expect("RR-0369: stable prefix");
        assert!(partial.consumed <= end, "RR-0369: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0369: full prefix should match bulk checksum");
        }
    }
}
