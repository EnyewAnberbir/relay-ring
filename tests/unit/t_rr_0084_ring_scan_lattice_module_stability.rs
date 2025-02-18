//! Integration test for `RR-0084` (stability).
//! Ring scan lattice modules optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0084_ring_scan_lattice_module_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x59];
    let full = relayring::capabilities::rr_0084_ring_scan_lattice_module::evaluate(fixture).expect("RR-0084: bulk Ring scan lattice modules optimize registry v24");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0084_ring_scan_lattice_module::evaluate(&fixture[..end]).expect("RR-0084: stable prefix");
        assert!(partial.consumed <= end, "RR-0084: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0084: full prefix should match bulk checksum");
        }
    }
}
