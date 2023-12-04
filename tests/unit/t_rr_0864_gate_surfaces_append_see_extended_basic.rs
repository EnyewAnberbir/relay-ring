//! Integration test for `RR-0864` (basic).
//! Extended: Gate surfaces append seek benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0864_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0x6b];
    let first = relayring::capabilities::rr_0864_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0864: Extended: Gate surfaces append seek benchmark reporter v29");
    let second = relayring::capabilities::rr_0864_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0864: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0864: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0864: stats visits every byte");
}
