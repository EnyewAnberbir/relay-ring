//! Integration test for `RR-0517` (basic).
//! Extended: Wire format RLRG frames integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0517_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x0e];
    let first = relayring::capabilities::rr_0517_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0517: Extended: Wire format RLRG frames integrate validator v17");
    let second = relayring::capabilities::rr_0517_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0517: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0517: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0517: stats visits every byte");
}
