//! Integration test for `RR-0061` (stability).
//! Ring scan lattice modules extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0061_ring_scan_lattice_module_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x42];
    let full = relayring::capabilities::rr_0061_ring_scan_lattice_module::evaluate(fixture).expect("RR-0061: bulk Ring scan lattice modules extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0061_ring_scan_lattice_module::evaluate(&fixture[..end]).expect("RR-0061: stable prefix");
        assert!(partial.consumed <= end, "RR-0061: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0061: full prefix should match bulk checksum");
        }
    }
}
