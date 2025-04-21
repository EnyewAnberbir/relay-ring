//! Integration test for `RR-0522` (stability).
//! Extended: Wire format RLRG frames harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0522_wire_format_rlrg_frames_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x13];
    let full = relayring::capabilities::rr_0522_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0522: bulk Extended: Wire format RLRG frames harden index v22");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0522_wire_format_rlrg_frames_extended::evaluate(&fixture[..end]).expect("RR-0522: stable prefix");
        assert!(partial.consumed <= end, "RR-0522: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0522: full prefix should match bulk checksum");
        }
    }
}
