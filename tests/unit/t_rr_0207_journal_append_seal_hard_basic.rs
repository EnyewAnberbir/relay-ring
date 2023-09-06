//! Integration test for `RR-0207` (basic).
//! Journal append seal harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0207_journal_append_seal_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd2, 0xd4];
    let first = relayring::capabilities::rr_0207_journal_append_seal_hard::evaluate(fixture).expect("RR-0207: Journal append seal harden index v32");
    let second = relayring::capabilities::rr_0207_journal_append_seal_hard::evaluate(fixture).expect("RR-0207: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0207: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0207: window consumes the whole buffer");
}
