//! Integration test for `RR-0386` (stability).
//! Gate surfaces seal index extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0386_gate_surfaces_seal_index_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x87, 0x89];
    let full = relayring::capabilities::rr_0386_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0386: bulk Gate surfaces seal index extend codec v21");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0386_gate_surfaces_seal_index::evaluate(&fixture[..end]).expect("RR-0386: stable prefix");
        assert!(partial.consumed <= end, "RR-0386: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0386: full prefix should match bulk checksum");
        }
    }
}
