//! Integration test for `RR-0561` (stability).
//! Extended: Ring scan lattice modules extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0561_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let full = relayring::capabilities::rr_0561_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0561: bulk Extended: Ring scan lattice modules extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0561_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0561: stable prefix");
        assert!(partial.consumed <= end, "RR-0561: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0561: full prefix should match bulk checksum");
        }
    }
}
