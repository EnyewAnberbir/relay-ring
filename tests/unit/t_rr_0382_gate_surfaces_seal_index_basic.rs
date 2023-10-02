//! Integration test for `RR-0382` (basic).
//! Gate surfaces seal index integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0382_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83, 0x85];
    let first = relayring::capabilities::rr_0382_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0382: Gate surfaces seal index integrate validator v17");
    let second = relayring::capabilities::rr_0382_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0382: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0382: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0382: window consumes the whole buffer");
}
