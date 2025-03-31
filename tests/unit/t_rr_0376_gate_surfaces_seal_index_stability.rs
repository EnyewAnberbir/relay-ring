//! Integration test for `RR-0376` (stability).
//! Gate surfaces seal index extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0376_gate_surfaces_seal_index_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7d, 0x7f];
    let full = relayring::capabilities::rr_0376_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0376: bulk Gate surfaces seal index extend codec v11");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0376_gate_surfaces_seal_index::evaluate(&fixture[..end]).expect("RR-0376: stable prefix");
        assert!(partial.consumed <= end, "RR-0376: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0376: full prefix should match bulk checksum");
        }
    }
}
