//! Integration test for `RR-0213` (basic).
//! Journal append seal refactor mutator v38 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0213_journal_append_seal_refa_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0xda];
    let first = relayring::capabilities::rr_0213_journal_append_seal_refa::evaluate(fixture).expect("RR-0213: Journal append seal refactor mutator v38");
    let second = relayring::capabilities::rr_0213_journal_append_seal_refa::evaluate(fixture).expect("RR-0213: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0213: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0213: window consumes the whole buffer");
}
