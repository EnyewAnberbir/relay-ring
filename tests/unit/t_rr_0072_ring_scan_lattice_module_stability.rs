//! Integration test for `RR-0072` (stability).
//! Ring scan lattice modules harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0072_ring_scan_lattice_module_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4b, 0x4d];
    let full = relayring::capabilities::rr_0072_ring_scan_lattice_module::evaluate(fixture).expect("RR-0072: bulk Ring scan lattice modules harden index v12");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0072_ring_scan_lattice_module::evaluate(&fixture[..end]).expect("RR-0072: stable prefix");
        assert!(partial.consumed <= end, "RR-0072: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0072: full prefix should match bulk checksum");
        }
    }
}
