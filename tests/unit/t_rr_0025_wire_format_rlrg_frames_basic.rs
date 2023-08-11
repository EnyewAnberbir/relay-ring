//! Integration test for `RR-0025` (basic).
//! Wire format RLRG frames validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0025_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x1e];
    let first = relayring::capabilities::rr_0025_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0025: Wire format RLRG frames validate resolver v25");
    let second = relayring::capabilities::rr_0025_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0025: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0025: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0025: stats visits every byte");
}
