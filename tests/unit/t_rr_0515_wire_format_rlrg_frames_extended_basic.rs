//! Integration test for `RR-0515` (basic).
//! Extended: Wire format RLRG frames validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0515_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x0c];
    let first = relayring::capabilities::rr_0515_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0515: Extended: Wire format RLRG frames validate resolver v15");
    let second = relayring::capabilities::rr_0515_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0515: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0515: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0515: scanner should emit domain hints");
}
