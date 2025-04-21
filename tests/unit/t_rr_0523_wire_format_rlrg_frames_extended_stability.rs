//! Integration test for `RR-0523` (stability).
//! Extended: Wire format RLRG frames wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0523_wire_format_rlrg_frames_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x14];
    let full = relayring::capabilities::rr_0523_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0523: bulk Extended: Wire format RLRG frames wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0523_wire_format_rlrg_frames_extended::evaluate(&fixture[..end]).expect("RR-0523: stable prefix");
        assert!(partial.consumed <= end, "RR-0523: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0523: full prefix should match bulk checksum");
        }
    }
}
