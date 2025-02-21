//! Integration test for `RR-0109` (stability).
//! Ring scan lattice modules benchmark reporter v49 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0109_ring_scan_lattice_module_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x70, 0x72];
    let full = relayring::capabilities::rr_0109_ring_scan_lattice_module::evaluate(fixture).expect("RR-0109: bulk Ring scan lattice modules benchmark reporter v49");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0109_ring_scan_lattice_module::evaluate(&fixture[..end]).expect("RR-0109: stable prefix");
        assert!(partial.consumed <= end, "RR-0109: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0109: full prefix should match bulk checksum");
        }
    }
}
