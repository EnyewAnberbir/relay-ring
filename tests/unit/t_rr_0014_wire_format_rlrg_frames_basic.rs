//! Integration test for `RR-0014` (basic).
//! Wire format RLRG frames optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0014_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x13];
    let first = relayring::capabilities::rr_0014_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0014: Wire format RLRG frames optimize registry v14");
    let second = relayring::capabilities::rr_0014_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0014: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0014: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0014: scanner should emit domain hints");
}
