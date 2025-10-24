//! Integration test for `RR-0837` (stream).
//! Extended: Gate surfaces append seek harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0837_gate_surfaces_append_see_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x50];
    let direct = relayring::capabilities::rr_0837_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0837: direct Extended: Gate surfaces append seek harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0837_gate_surfaces_append_see_extended::evaluate(&copied).expect("RR-0837: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0837: stream path must consume input");
}
