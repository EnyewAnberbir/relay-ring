//! Integration test for `RR-0389` (stability).
//! Gate surfaces seal index optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0389_gate_surfaces_seal_index_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x8c];
    let full = relayring::capabilities::rr_0389_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0389: bulk Gate surfaces seal index optimize registry v24");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0389_gate_surfaces_seal_index::evaluate(&fixture[..end]).expect("RR-0389: stable prefix");
        assert!(partial.consumed <= end, "RR-0389: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0389: full prefix should match bulk checksum");
        }
    }
}
