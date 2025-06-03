//! Integration test for `RR-0836` (stability).
//! Extended: Gate surfaces append seek extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0836_gate_surfaces_append_see_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4d, 0x4f];
    let full = relayring::capabilities::rr_0836_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0836: bulk Extended: Gate surfaces append seek extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0836_gate_surfaces_append_see_extended::evaluate(&fixture[..end]).expect("RR-0836: stable prefix");
        assert!(partial.consumed <= end, "RR-0836: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0836: full prefix should match bulk checksum");
        }
    }
}
