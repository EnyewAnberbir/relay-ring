//! Integration test for `RR-0012` (stability).
//! Wire format RLRG frames harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0012_wire_format_rlrg_frames_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x11];
    let full = relayring::capabilities::rr_0012_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0012: bulk Wire format RLRG frames harden index v12");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0012_wire_format_rlrg_frames::evaluate(&fixture[..end]).expect("RR-0012: stable prefix");
        assert!(partial.consumed <= end, "RR-0012: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0012: full prefix should match bulk checksum");
        }
    }
}
