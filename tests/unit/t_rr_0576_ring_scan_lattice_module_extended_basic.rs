//! Integration test for `RR-0576` (basic).
//! Extended: Ring scan lattice modules export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0576_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47, 0x49];
    let first = relayring::capabilities::rr_0576_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0576: Extended: Ring scan lattice modules export adapter v16");
    let second = relayring::capabilities::rr_0576_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0576: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0576: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0576: scanner should emit domain hints");
}
