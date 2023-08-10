//! Integration test for `RR-0016` (basic).
//! Wire format RLRG frames export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0016_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x15];
    let first = relayring::capabilities::rr_0016_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0016: Wire format RLRG frames export adapter v16");
    let second = relayring::capabilities::rr_0016_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0016: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0016: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0016: scanner should emit domain hints");
}
