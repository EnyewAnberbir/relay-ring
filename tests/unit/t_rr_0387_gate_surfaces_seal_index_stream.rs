//! Integration test for `RR-0387` (stream).
//! Gate surfaces seal index harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0387_gate_surfaces_seal_index_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x8a];
    let direct = relayring::capabilities::rr_0387_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0387: direct Gate surfaces seal index harden index v22");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0387_gate_surfaces_seal_index::evaluate(&copied).expect("RR-0387: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0387: stream path must consume input");
}
