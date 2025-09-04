//! Integration test for `RR-0514` (stream).
//! Extended: Wire format RLRG frames optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0514_wire_format_rlrg_frames_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0b];
    let direct = relayring::capabilities::rr_0514_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0514: direct Extended: Wire format RLRG frames optimize registry v14");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0514_wire_format_rlrg_frames_extended::evaluate(&copied).expect("RR-0514: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0514: stream path must consume input");
}
