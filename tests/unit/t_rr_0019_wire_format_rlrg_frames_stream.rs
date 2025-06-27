//! Integration test for `RR-0019` (stream).
//! Wire format RLRG frames benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0019_wire_format_rlrg_frames_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x18];
    let direct = relayring::capabilities::rr_0019_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0019: direct Wire format RLRG frames benchmark reporter v19");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0019_wire_format_rlrg_frames::evaluate(&copied).expect("RR-0019: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0019: stream path must consume input");
}
