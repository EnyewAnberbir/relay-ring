//! Integration test for `RR-0010` (stream).
//! Wire format RLRG frames implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0010_wire_format_rlrg_frames_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x0f];
    let direct = relayring::capabilities::rr_0010_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0010: direct Wire format RLRG frames implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0010_wire_format_rlrg_frames::evaluate(&copied).expect("RR-0010: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0010: stream path must consume input");
}
