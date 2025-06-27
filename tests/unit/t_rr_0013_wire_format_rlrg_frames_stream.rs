//! Integration test for `RR-0013` (stream).
//! Wire format RLRG frames wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0013_wire_format_rlrg_frames_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x12];
    let direct = relayring::capabilities::rr_0013_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0013: direct Wire format RLRG frames wire planner v13");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0013_wire_format_rlrg_frames::evaluate(&copied).expect("RR-0013: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0013: stream path must consume input");
}
