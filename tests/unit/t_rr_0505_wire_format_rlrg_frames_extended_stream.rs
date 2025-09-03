//! Integration test for `RR-0505` (stream).
//! Extended: Wire format RLRG frames validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0505_wire_format_rlrg_frames_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe, 0x02];
    let direct = relayring::capabilities::rr_0505_wire_format_rlrg_frames_extended::evaluate(fixture).expect("RR-0505: direct Extended: Wire format RLRG frames validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0505_wire_format_rlrg_frames_extended::evaluate(&copied).expect("RR-0505: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0505: stream path must consume input");
}
