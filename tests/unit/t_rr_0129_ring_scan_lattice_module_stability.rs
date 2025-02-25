//! Integration test for `RR-0129` (stability).
//! Ring scan lattice modules benchmark reporter v69 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0129_ring_scan_lattice_module_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x84, 0x86];
    let full = relayring::capabilities::rr_0129_ring_scan_lattice_module::evaluate(fixture).expect("RR-0129: bulk Ring scan lattice modules benchmark reporter v69");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0129_ring_scan_lattice_module::evaluate(&fixture[..end]).expect("RR-0129: stable prefix");
        assert!(partial.consumed <= end, "RR-0129: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0129: full prefix should match bulk checksum");
        }
    }
}
