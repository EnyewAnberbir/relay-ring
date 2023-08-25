//! Integration test for `RR-0131` (basic).
//! Ring scan lattice modules extend codec v71 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0131_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x86, 0x88];
    let first = relayring::capabilities::rr_0131_ring_scan_lattice_module::evaluate(fixture).expect("RR-0131: Ring scan lattice modules extend codec v71");
    let second = relayring::capabilities::rr_0131_ring_scan_lattice_module::evaluate(fixture).expect("RR-0131: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0131: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0131: stats visits every byte");
}
