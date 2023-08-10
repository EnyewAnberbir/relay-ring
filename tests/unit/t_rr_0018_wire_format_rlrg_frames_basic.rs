//! Integration test for `RR-0018` (basic).
//! Wire format RLRG frames refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0018_wire_format_rlrg_frames_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x17];
    let first = relayring::capabilities::rr_0018_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0018: Wire format RLRG frames refactor mutator v18");
    let second = relayring::capabilities::rr_0018_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0018: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0018: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0018: stats visits every byte");
}
