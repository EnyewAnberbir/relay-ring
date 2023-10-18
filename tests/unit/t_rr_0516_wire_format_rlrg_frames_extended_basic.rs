//! Integration test for `RR-0516` (basic).
//! Extended: Wire format RLRG frames export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0516_wire_format_rlrg_frames_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x0d];
    let first = relayring::capabilities::rr_0516_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0516: Extended: Wire format RLRG frames export adapter v16");
    let second = relayring::capabilities::rr_0516_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0516: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0516: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0516: stats visits every byte");
}
