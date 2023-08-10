//! Integration test for `RR-0022` (basic).
//! Wire format RLRG frames harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0022_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1b];
    let first = relayring::capabilities::rr_0022_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0022: Wire format RLRG frames harden index v22");
    let second = relayring::capabilities::rr_0022_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0022: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0022: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0022: window consumes the whole buffer");
}
