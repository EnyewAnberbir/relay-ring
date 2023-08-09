//! Integration test for `RR-0007` (basic).
//! Wire format RLRG frames integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0007_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x0c];
    let first = relayring::capabilities::rr_0007_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0007: Wire format RLRG frames integrate validator v7");
    let second = relayring::capabilities::rr_0007_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0007: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0007: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0007: scanner should emit domain hints");
}
