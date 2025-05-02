//! Integration test for `RR-0611` (stability).
//! Extended: Ring scan lattice modules extend codec v51 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0611_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6a, 0x6c];
    let full = relayring::capabilities::rr_0611_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0611: bulk Extended: Ring scan lattice modules extend codec v51");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0611_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0611: stable prefix");
        assert!(partial.consumed <= end, "RR-0611: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0611: full prefix should match bulk checksum");
        }
    }
}
