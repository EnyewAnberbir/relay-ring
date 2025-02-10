//! Integration test for `RR-0025` (stability).
//! Wire format RLRG frames validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0025_wire_format_rlrg_frames_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x1e];
    let full = relayring::capabilities::rr_0025_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0025: bulk Wire format RLRG frames validate resolver v25");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0025_wire_format_rlrg_frames::evaluate(&fixture[..end]).expect("RR-0025: stable prefix");
        assert!(partial.consumed <= end, "RR-0025: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0025: full prefix should match bulk checksum");
        }
    }
}
