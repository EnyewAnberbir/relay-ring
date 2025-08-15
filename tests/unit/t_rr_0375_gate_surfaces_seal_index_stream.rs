//! Integration test for `RR-0375` (stream).
//! Gate surfaces seal index implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0375_gate_surfaces_seal_index_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x7e];
    let direct = relayring::capabilities::rr_0375_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0375: direct Gate surfaces seal index implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0375_gate_surfaces_seal_index::evaluate(&copied).expect("RR-0375: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0375: stream path must consume input");
}
