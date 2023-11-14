//! Integration test for `RR-0709` (basic).
//! Extended: Journal append seal optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0709_journal_append_seal_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0xce];
    let first = relayring::capabilities::rr_0709_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0709: Extended: Journal append seal optimize registry v34");
    let second = relayring::capabilities::rr_0709_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0709: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0709: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0709: window consumes the whole buffer");
}
