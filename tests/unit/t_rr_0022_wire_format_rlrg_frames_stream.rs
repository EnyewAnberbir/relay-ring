//! Integration test for `RR-0022` (stream).
//! Wire format RLRG frames harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0022_wire_format_rlrg_frames_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1b];
    let direct = relayring::capabilities::rr_0022_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0022: direct Wire format RLRG frames harden index v22");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0022_wire_format_rlrg_frames::evaluate(&copied).expect("RR-0022: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0022: stream path must consume input");
}
