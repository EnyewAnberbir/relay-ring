//! Integration test for `RR-0679` (basic).
//! Extended: Journal append seal optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0679_journal_append_seal_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xae, 0xb0];
    let first = relayring::capabilities::rr_0679_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0679: Extended: Journal append seal optimize registry v4");
    let second = relayring::capabilities::rr_0679_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0679: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0679: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0679: stats visits every byte");
}
