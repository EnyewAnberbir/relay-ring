//! Integration test for `RR-0877` (stability).
//! Extended: Gate surfaces seal index harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0877_gate_surfaces_seal_index_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x76, 0x78];
    let full = relayring::capabilities::rr_0877_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0877: bulk Extended: Gate surfaces seal index harden index v12");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0877_gate_surfaces_seal_index_extended::evaluate(&fixture[..end]).expect("RR-0877: stable prefix");
        assert!(partial.consumed <= end, "RR-0877: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0877: full prefix should match bulk checksum");
        }
    }
}
