//! Integration test for `RR-0502` (stability).
//! Extended: Wire format RLRG frames harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0502_wire_format_rlrg_frames_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0xfd];
    let full = relayring::capabilities::rr_0502_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0502: bulk Extended: Wire format RLRG frames harden index v2");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0502_wire_format_rlrg_frames_extended::evaluate(&fixture[..end]).expect("RR-0502: stable prefix");
        assert!(partial.consumed <= end, "RR-0502: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0502: full prefix should match bulk checksum");
        }
    }
}
