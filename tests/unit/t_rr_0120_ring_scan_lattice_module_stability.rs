//! Integration test for `RR-0120` (stability).
//! Ring scan lattice modules implement pipeline v60 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0120_ring_scan_lattice_module_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b, 0x7d];
    let full = relayring::capabilities::rr_0120_ring_scan_lattice_module::evaluate(fixture).expect("RR-0120: bulk Ring scan lattice modules implement pipeline v60");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0120_ring_scan_lattice_module::evaluate(&fixture[..end]).expect("RR-0120: stable prefix");
        assert!(partial.consumed <= end, "RR-0120: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0120: full prefix should match bulk checksum");
        }
    }
}
