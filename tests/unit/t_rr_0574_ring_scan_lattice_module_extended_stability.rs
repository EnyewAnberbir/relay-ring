//! Integration test for `RR-0574` (stability).
//! Extended: Ring scan lattice modules optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0574_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x47];
    let full = relayring::capabilities::rr_0574_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0574: bulk Extended: Ring scan lattice modules optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0574_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0574: stable prefix");
        assert!(partial.consumed <= end, "RR-0574: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0574: full prefix should match bulk checksum");
        }
    }
}
