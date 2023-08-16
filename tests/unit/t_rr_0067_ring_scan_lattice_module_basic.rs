//! Integration test for `RR-0067` (basic).
//! Ring scan lattice modules integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0067_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x48];
    let first = relayring::capabilities::rr_0067_ring_scan_lattice_module::evaluate(fixture).expect("RR-0067: Ring scan lattice modules integrate validator v7");
    let second = relayring::capabilities::rr_0067_ring_scan_lattice_module::evaluate(fixture).expect("RR-0067: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0067: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0067: scanner should emit domain hints");
}
