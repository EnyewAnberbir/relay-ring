//! Integration test for `RR-0187` (basic).
//! Journal append seal harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0187_journal_append_seal_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbe, 0xc0];
    let first = relayring::capabilities::rr_0187_journal_append_seal_hard::evaluate(fixture).expect("RR-0187: Journal append seal harden index v12");
    let second = relayring::capabilities::rr_0187_journal_append_seal_hard::evaluate(fixture).expect("RR-0187: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0187: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0187: stats visits every byte");
}
