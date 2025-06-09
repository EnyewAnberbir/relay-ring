//! Integration test for `RR-0870` (stability).
//! Extended: Gate surfaces seal index validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0870_gate_surfaces_seal_index_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6f, 0x71];
    let full = relayring::capabilities::rr_0870_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0870: bulk Extended: Gate surfaces seal index validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0870_gate_surfaces_seal_index_extended::evaluate(&fixture[..end]).expect("RR-0870: stable prefix");
        assert!(partial.consumed <= end, "RR-0870: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0870: full prefix should match bulk checksum");
        }
    }
}
