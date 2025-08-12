//! Integration test for `RR-0346` (stream).
//! Gate surfaces append seek extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0346_gate_surfaces_append_see_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5f, 0x61];
    let direct = relayring::capabilities::rr_0346_gate_surfaces_append_see::evaluate(fixture).expect("RR-0346: direct Gate surfaces append seek extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0346_gate_surfaces_append_see::evaluate(&copied).expect("RR-0346: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0346: stream path must consume input");
}
