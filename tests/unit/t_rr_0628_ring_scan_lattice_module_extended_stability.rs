//! Integration test for `RR-0628` (stability).
//! Extended: Ring scan lattice modules refactor mutator v68 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0628_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b, 0x7d];
    let full = relayring::capabilities::rr_0628_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0628: bulk Extended: Ring scan lattice modules refactor mutator v68");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0628_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0628: stable prefix");
        assert!(partial.consumed <= end, "RR-0628: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0628: full prefix should match bulk checksum");
        }
    }
}
