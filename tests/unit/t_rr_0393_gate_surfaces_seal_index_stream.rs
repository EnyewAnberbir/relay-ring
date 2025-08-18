//! Integration test for `RR-0393` (stream).
//! Gate surfaces seal index refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0393_gate_surfaces_seal_index_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8e, 0x90];
    let direct = relayring::capabilities::rr_0393_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0393: direct Gate surfaces seal index refactor mutator v28");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0393_gate_surfaces_seal_index::evaluate(&copied).expect("RR-0393: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0393: stream path must consume input");
}
