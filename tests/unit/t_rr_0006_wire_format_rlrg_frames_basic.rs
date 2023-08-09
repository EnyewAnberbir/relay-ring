//! Integration test for `RR-0006` (basic).
//! Wire format RLRG frames export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0006_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0b];
    let first = relayring::capabilities::rr_0006_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0006: Wire format RLRG frames export adapter v6");
    let second = relayring::capabilities::rr_0006_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0006: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0006: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0006: scanner should emit domain hints");
}
