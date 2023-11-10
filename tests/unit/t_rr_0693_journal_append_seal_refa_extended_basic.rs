//! Integration test for `RR-0693` (basic).
//! Extended: Journal append seal refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0693_journal_append_seal_refa_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbc, 0xbe];
    let first = relayring::capabilities::rr_0693_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0693: Extended: Journal append seal refactor mutator v18");
    let second = relayring::capabilities::rr_0693_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0693: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0693: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0693: window consumes the whole buffer");
}
