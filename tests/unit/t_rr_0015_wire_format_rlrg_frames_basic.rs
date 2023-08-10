//! Integration test for `RR-0015` (basic).
//! Wire format RLRG frames validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0015_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x14];
    let first = relayring::capabilities::rr_0015_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0015: Wire format RLRG frames validate resolver v15");
    let second = relayring::capabilities::rr_0015_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0015: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0015: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0015: stats visits every byte");
}
