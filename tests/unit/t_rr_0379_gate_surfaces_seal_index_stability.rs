//! Integration test for `RR-0379` (stability).
//! Gate surfaces seal index optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0379_gate_surfaces_seal_index_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x82];
    let full = relayring::capabilities::rr_0379_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0379: bulk Gate surfaces seal index optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0379_gate_surfaces_seal_index::evaluate(&fixture[..end]).expect("RR-0379: stable prefix");
        assert!(partial.consumed <= end, "RR-0379: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0379: full prefix should match bulk checksum");
        }
    }
}
