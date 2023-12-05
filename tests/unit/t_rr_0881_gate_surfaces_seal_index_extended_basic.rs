//! Integration test for `RR-0881` (basic).
//! Extended: Gate surfaces seal index export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0881_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7a, 0x7c];
    let first = relayring::capabilities::rr_0881_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0881: Extended: Gate surfaces seal index export adapter v16");
    let second = relayring::capabilities::rr_0881_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0881: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0881: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0881: stats visits every byte");
}
