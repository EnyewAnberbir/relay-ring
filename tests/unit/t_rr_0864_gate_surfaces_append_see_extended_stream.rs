//! Integration test for `RR-0864` (stream).
//! Extended: Gate surfaces append seek benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0864_gate_surfaces_append_see_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0x6b];
    let direct = relayring::capabilities::rr_0864_gate_surfaces_append_see_extended::evaluate(fixture).expect("RR-0864: direct Extended: Gate surfaces append seek benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0864_gate_surfaces_append_see_extended::evaluate(&copied).expect("RR-0864: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0864: stream path must consume input");
}
