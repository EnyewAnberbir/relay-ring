//! Integration test for `RR-0141` (basic).
//! Ring scan lattice modules extend codec v81 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0141_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x92];
    let first = relayring::capabilities::rr_0141_ring_scan_lattice_module::evaluate(fixture).expect("RR-0141: Ring scan lattice modules extend codec v81");
    let second = relayring::capabilities::rr_0141_ring_scan_lattice_module::evaluate(fixture).expect("RR-0141: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0141: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0141: window consumes the whole buffer");
}
