//! Integration test for `RR-0581` (stability).
//! Extended: Ring scan lattice modules extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0581_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x4e];
    let full = relayring::capabilities::rr_0581_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0581: bulk Extended: Ring scan lattice modules extend codec v21");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0581_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0581: stable prefix");
        assert!(partial.consumed <= end, "RR-0581: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0581: full prefix should match bulk checksum");
        }
    }
}
