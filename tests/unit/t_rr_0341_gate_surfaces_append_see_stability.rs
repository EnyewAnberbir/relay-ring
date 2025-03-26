//! Integration test for `RR-0341` (stability).
//! Gate surfaces append seek export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0341_gate_surfaces_append_see_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5a, 0x5c];
    let full = relayring::capabilities::rr_0341_gate_surfaces_append_see::evaluate(fixture).expect("RR-0341: bulk Gate surfaces append seek export adapter v6");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0341_gate_surfaces_append_see::evaluate(&fixture[..end]).expect("RR-0341: stable prefix");
        assert!(partial.consumed <= end, "RR-0341: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0341: full prefix should match bulk checksum");
        }
    }
}
