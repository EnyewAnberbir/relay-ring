//! Integration test for `RR-0511` (stability).
//! Extended: Wire format RLRG frames extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0511_wire_format_rlrg_frames_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x08];
    let full = relayring::capabilities::rr_0511_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0511: bulk Extended: Wire format RLRG frames extend codec v11");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0511_wire_format_rlrg_frames_extended::evaluate(&fixture[..end]).expect("RR-0511: stable prefix");
        assert!(partial.consumed <= end, "RR-0511: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0511: full prefix should match bulk checksum");
        }
    }
}
