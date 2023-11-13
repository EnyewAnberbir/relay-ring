//! Integration test for `RR-0699` (basic).
//! Extended: Journal append seal optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0699_journal_append_seal_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xc4];
    let first = relayring::capabilities::rr_0699_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0699: Extended: Journal append seal optimize registry v24");
    let second = relayring::capabilities::rr_0699_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0699: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0699: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0699: stats visits every byte");
}
