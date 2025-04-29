//! Integration test for `RR-0585` (stability).
//! Extended: Ring scan lattice modules validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0585_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x52];
    let full = relayring::capabilities::rr_0585_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0585: bulk Extended: Ring scan lattice modules validate resolver v25");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0585_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0585: stable prefix");
        assert!(partial.consumed <= end, "RR-0585: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0585: full prefix should match bulk checksum");
        }
    }
}
