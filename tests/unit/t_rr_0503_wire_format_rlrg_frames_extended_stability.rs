//! Integration test for `RR-0503` (stability).
//! Extended: Wire format RLRG frames wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0503_wire_format_rlrg_frames_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0xfe];
    let full = relayring::capabilities::rr_0503_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0503: bulk Extended: Wire format RLRG frames wire planner v3");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0503_wire_format_rlrg_frames_extended::evaluate(&fixture[..end]).expect("RR-0503: stable prefix");
        assert!(partial.consumed <= end, "RR-0503: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0503: full prefix should match bulk checksum");
        }
    }
}
