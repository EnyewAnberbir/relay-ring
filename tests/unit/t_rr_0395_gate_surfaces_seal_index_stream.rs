//! Integration test for `RR-0395` (stream).
//! Gate surfaces seal index implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0395_gate_surfaces_seal_index_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x92];
    let direct = relayring::capabilities::rr_0395_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0395: direct Gate surfaces seal index implement pipeline v30");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0395_gate_surfaces_seal_index::evaluate(&copied).expect("RR-0395: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0395: stream path must consume input");
}
