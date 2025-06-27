//! Integration test for `RR-0011` (stream).
//! Wire format RLRG frames extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0011_wire_format_rlrg_frames_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x10];
    let direct = relayring::capabilities::rr_0011_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0011: direct Wire format RLRG frames extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0011_wire_format_rlrg_frames::evaluate(&copied).expect("RR-0011: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0011: stream path must consume input");
}
