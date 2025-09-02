//! Integration test for `RR-0501` (stream).
//! Extended: Wire format RLRG frames extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0501_wire_format_rlrg_frames_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0xfc];
    let direct = relayring::capabilities::rr_0501_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0501: direct Extended: Wire format RLRG frames extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0501_wire_format_rlrg_frames_extended::evaluate(&copied).expect("RR-0501: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0501: stream path must consume input");
}
