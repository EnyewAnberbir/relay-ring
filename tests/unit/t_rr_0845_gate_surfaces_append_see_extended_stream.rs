//! Integration test for `RR-0845` (stream).
//! Extended: Gate surfaces append seek implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0845_gate_surfaces_append_see_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x56, 0x58];
    let direct = relayring::capabilities::rr_0845_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0845: direct Extended: Gate surfaces append seek implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0845_gate_surfaces_append_see_extended::evaluate(&copied).expect("RR-0845: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0845: stream path must consume input");
}
