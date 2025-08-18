//! Integration test for `RR-0384` (stream).
//! Gate surfaces seal index benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0384_gate_surfaces_seal_index_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x85, 0x87];
    let direct = relayring::capabilities::rr_0384_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0384: direct Gate surfaces seal index benchmark reporter v19");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0384_gate_surfaces_seal_index::evaluate(&copied).expect("RR-0384: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0384: stream path must consume input");
}
