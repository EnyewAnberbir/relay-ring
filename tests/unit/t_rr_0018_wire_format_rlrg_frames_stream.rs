//! Integration test for `RR-0018` (stream).
//! Wire format RLRG frames refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0018_wire_format_rlrg_frames_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x17];
    let direct = relayring::capabilities::rr_0018_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0018: direct Wire format RLRG frames refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0018_wire_format_rlrg_frames::evaluate(&copied).expect("RR-0018: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0018: stream path must consume input");
}
