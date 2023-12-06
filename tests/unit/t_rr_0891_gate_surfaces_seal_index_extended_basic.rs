//! Integration test for `RR-0891` (basic).
//! Extended: Gate surfaces seal index export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0891_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x84, 0x86];
    let first = relayring::capabilities::rr_0891_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0891: Extended: Gate surfaces seal index export adapter v26");
    let second = relayring::capabilities::rr_0891_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0891: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0891: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0891: stats visits every byte");
}
