//! Integration test for `RR-0677` (basic).
//! Extended: Journal append seal harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0677_journal_append_seal_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xac, 0xae];
    let first = relayring::capabilities::rr_0677_journal_append_seal_hard_extended::evaluate(fixture).expect("RR-0677: Extended: Journal append seal harden index v2");
    let second = relayring::capabilities::rr_0677_journal_append_seal_hard_extended::evaluate(fixture).expect("RR-0677: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0677: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0677: window consumes the whole buffer");
}
