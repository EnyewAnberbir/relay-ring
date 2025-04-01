//! Integration test for `RR-0388` (stability).
//! Gate surfaces seal index wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0388_gate_surfaces_seal_index_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x89, 0x8b];
    let full = relayring::capabilities::rr_0388_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0388: bulk Gate surfaces seal index wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0388_gate_surfaces_seal_index::evaluate(&fixture[..end]).expect("RR-0388: stable prefix");
        assert!(partial.consumed <= end, "RR-0388: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0388: full prefix should match bulk checksum");
        }
    }
}
