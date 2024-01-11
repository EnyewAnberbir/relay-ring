//! Integration test for `RR-0130` (bounds).
//! Ring scan lattice modules implement pipeline v70 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0130_ring_scan_lattice_module_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0130_ring_scan_lattice_module::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0130_ring_scan_lattice_module::evaluate(fixture).expect("RR-0130 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
