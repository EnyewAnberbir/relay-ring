//! Integration test for `RR-0179` (basic).
//! Journal append seal optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0179_journal_append_seal_opti_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb6, 0xb8];
    let first = relayring::capabilities::rr_0179_journal_append_seal_opti::evaluate(fixture).expect("RR-0179: Journal append seal optimize registry v4");
    let second = relayring::capabilities::rr_0179_journal_append_seal_opti::evaluate(fixture).expect("RR-0179: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0179: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0179: stats visits every byte");
}
