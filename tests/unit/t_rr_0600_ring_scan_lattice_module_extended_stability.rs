//! Integration test for `RR-0600` (stability).
//! Extended: Ring scan lattice modules implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0600_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5f, 0x61];
    let full = relayring::capabilities::rr_0600_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0600: bulk Extended: Ring scan lattice modules implement pipeline v40");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0600_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0600: stable prefix");
        assert!(partial.consumed <= end, "RR-0600: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0600: full prefix should match bulk checksum");
        }
    }
}
