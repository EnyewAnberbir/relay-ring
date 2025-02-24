//! Integration test for `RR-0116` (stability).
//! Ring scan lattice modules export adapter v56 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0116_ring_scan_lattice_module_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x79];
    let full = relayring::capabilities::rr_0116_ring_scan_lattice_module::evaluate(fixture).expect("RR-0116: bulk Ring scan lattice modules export adapter v56");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0116_ring_scan_lattice_module::evaluate(&fixture[..end]).expect("RR-0116: stable prefix");
        assert!(partial.consumed <= end, "RR-0116: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0116: full prefix should match bulk checksum");
        }
    }
}
