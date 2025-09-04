//! Integration test for `RR-0513` (stream).
//! Extended: Wire format RLRG frames wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0513_wire_format_rlrg_frames_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0a];
    let direct = relayring::capabilities::rr_0513_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0513: direct Extended: Wire format RLRG frames wire planner v13");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0513_wire_format_rlrg_frames_extended::evaluate(&copied).expect("RR-0513: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0513: stream path must consume input");
}
