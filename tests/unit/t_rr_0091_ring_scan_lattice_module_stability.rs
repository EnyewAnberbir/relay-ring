//! Integration test for `RR-0091` (stability).
//! Ring scan lattice modules extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0091_ring_scan_lattice_module_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5e, 0x60];
    let full = relayring::capabilities::rr_0091_ring_scan_lattice_module::evaluate(fixture).expect("RR-0091: bulk Ring scan lattice modules extend codec v31");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0091_ring_scan_lattice_module::evaluate(&fixture[..end]).expect("RR-0091: stable prefix");
        assert!(partial.consumed <= end, "RR-0091: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0091: full prefix should match bulk checksum");
        }
    }
}
