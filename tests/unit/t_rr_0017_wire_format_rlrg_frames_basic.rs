//! Integration test for `RR-0017` (basic).
//! Wire format RLRG frames integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0017_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x16];
    let first = relayring::capabilities::rr_0017_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0017: Wire format RLRG frames integrate validator v17");
    let second = relayring::capabilities::rr_0017_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0017: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0017: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0017: scanner should emit domain hints");
}
