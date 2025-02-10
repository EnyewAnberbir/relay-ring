//! Integration test for `RR-0018` (stability).
//! Wire format RLRG frames refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0018_wire_format_rlrg_frames_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x17];
    let full = relayring::capabilities::rr_0018_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0018: bulk Wire format RLRG frames refactor mutator v18");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0018_wire_format_rlrg_frames::evaluate(&fixture[..end]).expect("RR-0018: stable prefix");
        assert!(partial.consumed <= end, "RR-0018: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0018: full prefix should match bulk checksum");
        }
    }
}
