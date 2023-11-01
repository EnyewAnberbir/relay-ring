//! Integration test for `RR-0626` (basic).
//! Extended: Ring scan lattice modules export adapter v66 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0626_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x79, 0x7b];
    let first = relayring::capabilities::rr_0626_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0626: Extended: Ring scan lattice modules export adapter v66");
    let second = relayring::capabilities::rr_0626_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0626: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0626: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0626: window consumes the whole buffer");
}
