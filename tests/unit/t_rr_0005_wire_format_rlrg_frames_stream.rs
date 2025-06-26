//! Integration test for `RR-0005` (stream).
//! Wire format RLRG frames validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0005_wire_format_rlrg_frames_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0a];
    let direct = relayring::capabilities::rr_0005_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0005: direct Wire format RLRG frames validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0005_wire_format_rlrg_frames::evaluate(&copied).expect("RR-0005: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0005: stream path must consume input");
}
