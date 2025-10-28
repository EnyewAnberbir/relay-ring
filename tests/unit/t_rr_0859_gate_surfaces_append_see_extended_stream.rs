//! Integration test for `RR-0859` (stream).
//! Extended: Gate surfaces append seek optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0859_gate_surfaces_append_see_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64, 0x66];
    let direct = relayring::capabilities::rr_0859_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0859: direct Extended: Gate surfaces append seek optimize registry v24");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0859_gate_surfaces_append_see_extended::evaluate(&copied).expect("RR-0859: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0859: stream path must consume input");
}
