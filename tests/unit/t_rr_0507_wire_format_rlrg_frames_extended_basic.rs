//! Integration test for `RR-0507` (basic).
//! Extended: Wire format RLRG frames integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0507_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x04];
    let first = relayring::capabilities::rr_0507_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0507: Extended: Wire format RLRG frames integrate validator v7");
    let second = relayring::capabilities::rr_0507_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0507: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0507: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0507: stats visits every byte");
}
