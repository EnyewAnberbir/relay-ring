//! Integration test for `RR-0014` (stability).
//! Wire format RLRG frames optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0014_wire_format_rlrg_frames_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x13];
    let full = relayring::capabilities::rr_0014_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0014: bulk Wire format RLRG frames optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0014_wire_format_rlrg_frames::evaluate(&fixture[..end]).expect("RR-0014: stable prefix");
        assert!(partial.consumed <= end, "RR-0014: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0014: full prefix should match bulk checksum");
        }
    }
}
