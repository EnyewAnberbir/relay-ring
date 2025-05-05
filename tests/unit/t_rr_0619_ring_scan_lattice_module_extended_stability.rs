//! Integration test for `RR-0619` (stability).
//! Extended: Ring scan lattice modules benchmark reporter v59 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0619_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x72, 0x74];
    let full = relayring::capabilities::rr_0619_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0619: bulk Extended: Ring scan lattice modules benchmark reporter v59");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0619_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0619: stable prefix");
        assert!(partial.consumed <= end, "RR-0619: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0619: full prefix should match bulk checksum");
        }
    }
}
