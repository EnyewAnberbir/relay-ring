//! Integration test for `RR-0016` (stability).
//! Wire format RLRG frames export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0016_wire_format_rlrg_frames_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x15];
    let full = relayring::capabilities::rr_0016_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0016: bulk Wire format RLRG frames export adapter v16");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0016_wire_format_rlrg_frames::evaluate(&fixture[..end]).expect("RR-0016: stable prefix");
        assert!(partial.consumed <= end, "RR-0016: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0016: full prefix should match bulk checksum");
        }
    }
}
