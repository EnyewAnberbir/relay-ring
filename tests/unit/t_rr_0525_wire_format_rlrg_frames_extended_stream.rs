//! Integration test for `RR-0525` (stream).
//! Extended: Wire format RLRG frames validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0525_wire_format_rlrg_frames_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x16];
    let direct = relayring::capabilities::rr_0525_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0525: direct Extended: Wire format RLRG frames validate resolver v25");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0525_wire_format_rlrg_frames_extended::evaluate(&copied).expect("RR-0525: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0525: stream path must consume input");
}
