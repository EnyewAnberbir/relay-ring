//! Integration test for `RR-0629` (stability).
//! Extended: Ring scan lattice modules benchmark reporter v69 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0629_ring_scan_lattice_module_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x7e];
    let full = relayring::capabilities::rr_0629_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0629: bulk Extended: Ring scan lattice modules benchmark reporter v69");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0629_ring_scan_lattice_module_extended::evaluate(&fixture[..end]).expect("RR-0629: stable prefix");
        assert!(partial.consumed <= end, "RR-0629: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0629: full prefix should match bulk checksum");
        }
    }
}
