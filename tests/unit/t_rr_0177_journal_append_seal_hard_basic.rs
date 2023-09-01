//! Integration test for `RR-0177` (basic).
//! Journal append seal harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0177_journal_append_seal_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0xb6];
    let first = relayring::capabilities::rr_0177_journal_append_seal_hard::evaluate(fixture).expect("RR-0177: Journal append seal harden index v2");
    let second = relayring::capabilities::rr_0177_journal_append_seal_hard::evaluate(fixture).expect("RR-0177: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0177: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0177: window consumes the whole buffer");
}
