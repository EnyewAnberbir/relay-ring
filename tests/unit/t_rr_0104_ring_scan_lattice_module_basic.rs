//! Integration test for `RR-0104` (basic).
//! Ring scan lattice modules optimize registry v44 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0104_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6b, 0x6d];
    let first = relayring::capabilities::rr_0104_ring_scan_lattice_module::evaluate(fixture).expect("RR-0104: Ring scan lattice modules optimize registry v44");
    let second = relayring::capabilities::rr_0104_ring_scan_lattice_module::evaluate(fixture).expect("RR-0104: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0104: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0104: scanner should emit domain hints");
}
