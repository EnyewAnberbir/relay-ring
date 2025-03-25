//! Integration test for `RR-0336` (stability).
//! Gate surfaces append seek extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0336_gate_surfaces_append_see_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x55, 0x57];
    let full = relayring::capabilities::rr_0336_gate_surfaces_append_see::evaluate(fixture).expect("RR-0336: bulk Gate surfaces append seek extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0336_gate_surfaces_append_see::evaluate(&fixture[..end]).expect("RR-0336: stable prefix");
        assert!(partial.consumed <= end, "RR-0336: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0336: full prefix should match bulk checksum");
        }
    }
}
