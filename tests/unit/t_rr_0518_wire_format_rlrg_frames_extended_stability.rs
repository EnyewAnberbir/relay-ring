//! Integration test for `RR-0518` (stability).
//! Extended: Wire format RLRG frames refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0518_wire_format_rlrg_frames_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x0f];
    let full = relayring::capabilities::rr_0518_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0518: bulk Extended: Wire format RLRG frames refactor mutator v18");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0518_wire_format_rlrg_frames_extended::evaluate(&fixture[..end]).expect("RR-0518: stable prefix");
        assert!(partial.consumed <= end, "RR-0518: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0518: full prefix should match bulk checksum");
        }
    }
}
