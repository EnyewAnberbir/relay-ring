//! Integration test for `RR-0382` (stability).
//! Gate surfaces seal index integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0382_gate_surfaces_seal_index_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83, 0x85];
    let full = relayring::capabilities::rr_0382_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0382: bulk Gate surfaces seal index integrate validator v17");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0382_gate_surfaces_seal_index::evaluate(&fixture[..end]).expect("RR-0382: stable prefix");
        assert!(partial.consumed <= end, "RR-0382: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0382: full prefix should match bulk checksum");
        }
    }
}
