//! Integration test for `RR-0005` (basic).
//! Wire format RLRG frames validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0005_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0a];
    let first = relayring::capabilities::rr_0005_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0005: Wire format RLRG frames validate resolver v5");
    let second = relayring::capabilities::rr_0005_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0005: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0005: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0005: stats visits every byte");
}
