//! Integration test for `RR-0506` (stream).
//! Extended: Wire format RLRG frames export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0506_wire_format_rlrg_frames_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03];
    let direct = relayring::capabilities::rr_0506_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0506: direct Extended: Wire format RLRG frames export adapter v6");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0506_wire_format_rlrg_frames_extended::evaluate(&copied).expect("RR-0506: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0506: stream path must consume input");
}
