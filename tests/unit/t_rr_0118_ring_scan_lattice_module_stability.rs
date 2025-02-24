//! Integration test for `RR-0118` (stability).
//! Ring scan lattice modules refactor mutator v58 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0118_ring_scan_lattice_module_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x79, 0x7b];
    let full = relayring::capabilities::rr_0118_ring_scan_lattice_module::evaluate(fixture).expect("RR-0118: bulk Ring scan lattice modules refactor mutator v58");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0118_ring_scan_lattice_module::evaluate(&fixture[..end]).expect("RR-0118: stable prefix");
        assert!(partial.consumed <= end, "RR-0118: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0118: full prefix should match bulk checksum");
        }
    }
}
