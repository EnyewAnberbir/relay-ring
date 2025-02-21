//! Integration test for `RR-0105` (stability).
//! Ring scan lattice modules validate resolver v45 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0105_ring_scan_lattice_module_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6c, 0x6e];
    let full = relayring::capabilities::rr_0105_ring_scan_lattice_module::evaluate(fixture).expect("RR-0105: bulk Ring scan lattice modules validate resolver v45");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0105_ring_scan_lattice_module::evaluate(&fixture[..end]).expect("RR-0105: stable prefix");
        assert!(partial.consumed <= end, "RR-0105: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0105: full prefix should match bulk checksum");
        }
    }
}
