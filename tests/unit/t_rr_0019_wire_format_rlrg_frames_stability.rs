//! Integration test for `RR-0019` (stability).
//! Wire format RLRG frames benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0019_wire_format_rlrg_frames_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x18];
    let full = relayring::capabilities::rr_0019_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0019: bulk Wire format RLRG frames benchmark reporter v19");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0019_wire_format_rlrg_frames::evaluate(&fixture[..end]).expect("RR-0019: stable prefix");
        assert!(partial.consumed <= end, "RR-0019: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0019: full prefix should match bulk checksum");
        }
    }
}
