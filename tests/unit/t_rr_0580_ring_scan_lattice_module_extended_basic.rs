//! Integration test for `RR-0580` (basic).
//! Extended: Ring scan lattice modules implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0580_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4b, 0x4d];
    let first = relayring::capabilities::rr_0580_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0580: Extended: Ring scan lattice modules implement pipeline v20");
    let second = relayring::capabilities::rr_0580_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0580: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0580: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0580: scanner should emit domain hints");
}
