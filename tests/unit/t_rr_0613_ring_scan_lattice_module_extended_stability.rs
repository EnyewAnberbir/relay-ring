//! Integration test for `RR-0613` (stability).
//! Extended: Ring scan lattice modules wire planner v53 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0613_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6c, 0x6e];
    let full = relayring::capabilities::rr_0613_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0613: bulk Extended: Ring scan lattice modules wire planner v53");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0613_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0613: stable prefix");
        assert!(partial.consumed <= end, "RR-0613: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0613: full prefix should match bulk checksum");
        }
    }
}
