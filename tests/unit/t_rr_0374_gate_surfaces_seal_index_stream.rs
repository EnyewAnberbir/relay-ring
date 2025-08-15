//! Integration test for `RR-0374` (stream).
//! Gate surfaces seal index benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0374_gate_surfaces_seal_index_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b, 0x7d];
    let direct = relayring::capabilities::rr_0374_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0374: direct Gate surfaces seal index benchmark reporter v9");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0374_gate_surfaces_seal_index::evaluate(&copied).expect("RR-0374: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0374: stream path must consume input");
}
