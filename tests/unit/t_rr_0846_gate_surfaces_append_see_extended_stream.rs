//! Integration test for `RR-0846` (stream).
//! Extended: Gate surfaces append seek extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0846_gate_surfaces_append_see_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x59];
    let direct = relayring::capabilities::rr_0846_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0846: direct Extended: Gate surfaces append seek extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0846_gate_surfaces_append_see_extended::evaluate(&copied).expect("RR-0846: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0846: stream path must consume input");
}
