//! Integration test for `RR-0520` (stream).
//! Extended: Wire format RLRG frames implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0520_wire_format_rlrg_frames_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x11];
    let direct = relayring::capabilities::rr_0520_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0520: direct Extended: Wire format RLRG frames implement pipeline v20");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0520_wire_format_rlrg_frames_extended::evaluate(&copied).expect("RR-0520: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0520: stream path must consume input");
}
