//! Integration test for `RR-0010` (basic).
//! Wire format RLRG frames implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0010_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x0f];
    let first = relayring::capabilities::rr_0010_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0010: Wire format RLRG frames implement pipeline v10");
    let second = relayring::capabilities::rr_0010_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0010: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0010: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0010: scanner should emit domain hints");
}
