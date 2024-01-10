//! Integration test for `RR-0117` (bounds).
//! Ring scan lattice modules integrate validator v57 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0117_ring_scan_lattice_module_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0117_ring_scan_lattice_module::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0117_ring_scan_lattice_module::evaluate(fixture).expect("RR-0117 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
