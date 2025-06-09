//! Integration test for `RR-0864` (stability).
//! Extended: Gate surfaces append seek benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0864_gate_surfaces_append_see_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0x6b];
    let full = relayring::capabilities::rr_0864_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0864: bulk Extended: Gate surfaces append seek benchmark reporter v29");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0864_gate_surfaces_append_see_extended::evaluate(&fixture[..end]).expect("RR-0864: stable prefix");
        assert!(partial.consumed <= end, "RR-0864: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0864: full prefix should match bulk checksum");
        }
    }
}
