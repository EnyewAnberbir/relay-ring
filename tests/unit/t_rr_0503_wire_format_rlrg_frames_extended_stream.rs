//! Integration test for `RR-0503` (stream).
//! Extended: Wire format RLRG frames wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0503_wire_format_rlrg_frames_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0xfe];
    let direct = relayring::capabilities::rr_0503_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0503: direct Extended: Wire format RLRG frames wire planner v3");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0503_wire_format_rlrg_frames_extended::evaluate(&copied).expect("RR-0503: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0503: stream path must consume input");
}
