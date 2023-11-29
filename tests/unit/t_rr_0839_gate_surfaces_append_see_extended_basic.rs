//! Integration test for `RR-0839` (basic).
//! Extended: Gate surfaces append seek optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0839_gate_surfaces_append_see_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x52];
    let first = relayring::capabilities::rr_0839_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0839: Extended: Gate surfaces append seek optimize registry v4");
    let second = relayring::capabilities::rr_0839_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0839: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0839: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0839: scanner should emit domain hints");
}
