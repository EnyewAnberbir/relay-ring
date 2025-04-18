//! Integration test for `RR-0505` (stability).
//! Extended: Wire format RLRG frames validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0505_wire_format_rlrg_frames_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe, 0x02];
    let full = relayring::capabilities::rr_0505_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0505: bulk Extended: Wire format RLRG frames validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0505_wire_format_rlrg_frames_extended::evaluate(&fixture[..end]).expect("RR-0505: stable prefix");
        assert!(partial.consumed <= end, "RR-0505: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0505: full prefix should match bulk checksum");
        }
    }
}
