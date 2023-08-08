//! Integration test for `RR-0001` (basic).
//! Wire format RLRG frames extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0001_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06];
    let first = relayring::capabilities::rr_0001_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0001: Wire format RLRG frames extend codec v1");
    let second = relayring::capabilities::rr_0001_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0001: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0001: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0001: stats visits every byte");
}
