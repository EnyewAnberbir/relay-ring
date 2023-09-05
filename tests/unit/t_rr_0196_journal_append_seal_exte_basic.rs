//! Integration test for `RR-0196` (basic).
//! Journal append seal extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0196_journal_append_seal_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc7, 0xc9];
    let first = relayring::capabilities::rr_0196_journal_append_seal_exte::evaluate(fixture).expect("RR-0196: Journal append seal extend codec v21");
    let second = relayring::capabilities::rr_0196_journal_append_seal_exte::evaluate(fixture).expect("RR-0196: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0196: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0196: window consumes the whole buffer");
}
