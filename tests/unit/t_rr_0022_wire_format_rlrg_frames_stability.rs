//! Integration test for `RR-0022` (stability).
//! Wire format RLRG frames harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0022_wire_format_rlrg_frames_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1b];
    let full = relayring::capabilities::rr_0022_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0022: bulk Wire format RLRG frames harden index v22");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0022_wire_format_rlrg_frames::evaluate(&fixture[..end]).expect("RR-0022: stable prefix");
        assert!(partial.consumed <= end, "RR-0022: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0022: full prefix should match bulk checksum");
        }
    }
}
