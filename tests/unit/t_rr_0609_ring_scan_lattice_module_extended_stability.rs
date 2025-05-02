//! Integration test for `RR-0609` (stability).
//! Extended: Ring scan lattice modules benchmark reporter v49 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0609_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x6a];
    let full = relayring::capabilities::rr_0609_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0609: bulk Extended: Ring scan lattice modules benchmark reporter v49");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0609_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0609: stable prefix");
        assert!(partial.consumed <= end, "RR-0609: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0609: full prefix should match bulk checksum");
        }
    }
}
