//! Integration test for `RR-0111` (basic).
//! Ring scan lattice modules extend codec v51 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0111_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x72, 0x74];
    let first = relayring::capabilities::rr_0111_ring_scan_lattice_module::evaluate(fixture).expect("RR-0111: Ring scan lattice modules extend codec v51");
    let second = relayring::capabilities::rr_0111_ring_scan_lattice_module::evaluate(fixture).expect("RR-0111: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0111: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0111: stats visits every byte");
}
