//! Integration test for `RR-0892` (stability).
//! Extended: Gate surfaces seal index integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0892_gate_surfaces_seal_index_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x85, 0x87];
    let full = relayring::capabilities::rr_0892_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0892: bulk Extended: Gate surfaces seal index integrate validator v27");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0892_gate_surfaces_seal_index_extended::evaluate(&fixture[..end]).expect("RR-0892: stable prefix");
        assert!(partial.consumed <= end, "RR-0892: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0892: full prefix should match bulk checksum");
        }
    }
}
