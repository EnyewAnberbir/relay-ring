//! Integration test for `RR-0595` (stability).
//! Extended: Ring scan lattice modules validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0595_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5a, 0x5c];
    let full = relayring::capabilities::rr_0595_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0595: bulk Extended: Ring scan lattice modules validate resolver v35");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0595_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0595: stable prefix");
        assert!(partial.consumed <= end, "RR-0595: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0595: full prefix should match bulk checksum");
        }
    }
}
