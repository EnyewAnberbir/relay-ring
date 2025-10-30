//! Integration test for `RR-0883` (stream).
//! Extended: Gate surfaces seal index refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0883_gate_surfaces_seal_index_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x7e];
    let direct = relayring::capabilities::rr_0883_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0883: direct Extended: Gate surfaces seal index refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0883_gate_surfaces_seal_index_extended::evaluate(&copied).expect("RR-0883: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0883: stream path must consume input");
}
