//! Integration test for `RR-0891` (stability).
//! Extended: Gate surfaces seal index export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0891_gate_surfaces_seal_index_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x84, 0x86];
    let full = relayring::capabilities::rr_0891_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0891: bulk Extended: Gate surfaces seal index export adapter v26");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0891_gate_surfaces_seal_index_extended::evaluate(&fixture[..end]).expect("RR-0891: stable prefix");
        assert!(partial.consumed <= end, "RR-0891: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0891: full prefix should match bulk checksum");
        }
    }
}
