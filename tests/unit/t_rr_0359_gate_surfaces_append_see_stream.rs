//! Integration test for `RR-0359` (stream).
//! Gate surfaces append seek optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0359_gate_surfaces_append_see_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6c, 0x6e];
    let direct = relayring::capabilities::rr_0359_gate_surfaces_append_see::evaluate(fixture).expect("RR-0359: direct Gate surfaces append seek optimize registry v24");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0359_gate_surfaces_append_see::evaluate(&copied).expect("RR-0359: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0359: stream path must consume input");
}
