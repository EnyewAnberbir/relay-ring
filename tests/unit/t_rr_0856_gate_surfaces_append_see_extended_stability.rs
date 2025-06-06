//! Integration test for `RR-0856` (stability).
//! Extended: Gate surfaces append seek extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0856_gate_surfaces_append_see_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x63];
    let full = relayring::capabilities::rr_0856_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0856: bulk Extended: Gate surfaces append seek extend codec v21");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0856_gate_surfaces_append_see_extended::evaluate(&fixture[..end]).expect("RR-0856: stable prefix");
        assert!(partial.consumed <= end, "RR-0856: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0856: full prefix should match bulk checksum");
        }
    }
}
