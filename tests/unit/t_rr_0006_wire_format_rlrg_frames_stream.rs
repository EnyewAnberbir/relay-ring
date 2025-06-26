//! Integration test for `RR-0006` (stream).
//! Wire format RLRG frames export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0006_wire_format_rlrg_frames_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0b];
    let direct = relayring::capabilities::rr_0006_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0006: direct Wire format RLRG frames export adapter v6");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0006_wire_format_rlrg_frames::evaluate(&copied).expect("RR-0006: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0006: stream path must consume input");
}
