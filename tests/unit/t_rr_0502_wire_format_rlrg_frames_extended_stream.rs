//! Integration test for `RR-0502` (stream).
//! Extended: Wire format RLRG frames harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0502_wire_format_rlrg_frames_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0xfd];
    let direct = relayring::capabilities::rr_0502_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0502: direct Extended: Wire format RLRG frames harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0502_wire_format_rlrg_frames_extended::evaluate(&copied).expect("RR-0502: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0502: stream path must consume input");
}
