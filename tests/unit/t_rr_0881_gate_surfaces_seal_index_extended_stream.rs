//! Integration test for `RR-0881` (stream).
//! Extended: Gate surfaces seal index export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0881_gate_surfaces_seal_index_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7a, 0x7c];
    let direct = relayring::capabilities::rr_0881_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0881: direct Extended: Gate surfaces seal index export adapter v16");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0881_gate_surfaces_seal_index_extended::evaluate(&copied).expect("RR-0881: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0881: stream path must consume input");
}
