//! Integration test for `RR-0871` (stability).
//! Extended: Gate surfaces seal index export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0871_gate_surfaces_seal_index_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x70, 0x72];
    let full = relayring::capabilities::rr_0871_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0871: bulk Extended: Gate surfaces seal index export adapter v6");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0871_gate_surfaces_seal_index_extended::evaluate(&fixture[..end]).expect("RR-0871: stable prefix");
        assert!(partial.consumed <= end, "RR-0871: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0871: full prefix should match bulk checksum");
        }
    }
}
