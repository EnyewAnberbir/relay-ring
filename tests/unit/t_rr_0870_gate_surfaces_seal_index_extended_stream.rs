//! Integration test for `RR-0870` (stream).
//! Extended: Gate surfaces seal index validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0870_gate_surfaces_seal_index_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6f, 0x71];
    let direct = relayring::capabilities::rr_0870_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0870: direct Extended: Gate surfaces seal index validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0870_gate_surfaces_seal_index_extended::evaluate(&copied).expect("RR-0870: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0870: stream path must consume input");
}
