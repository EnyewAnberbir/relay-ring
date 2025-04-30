//! Integration test for `RR-0589` (stability).
//! Extended: Ring scan lattice modules benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0589_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x56];
    let full = relayring::capabilities::rr_0589_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0589: bulk Extended: Ring scan lattice modules benchmark reporter v29");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0589_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0589: stable prefix");
        assert!(partial.consumed <= end, "RR-0589: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0589: full prefix should match bulk checksum");
        }
    }
}
