//! Integration test for `RR-0352` (stability).
//! Gate surfaces append seek integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0352_gate_surfaces_append_see_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x65, 0x67];
    let full = relayring::capabilities::rr_0352_gate_surfaces_append_see::evaluate(fixture).expect("RR-0352: bulk Gate surfaces append seek integrate validator v17");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0352_gate_surfaces_append_see::evaluate(&fixture[..end]).expect("RR-0352: stable prefix");
        assert!(partial.consumed <= end, "RR-0352: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0352: full prefix should match bulk checksum");
        }
    }
}
