//! Integration test for `RR-0190` (basic).
//! Journal append seal validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0190_journal_append_seal_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc1, 0xc3];
    let first = relayring::capabilities::rr_0190_journal_append_seal_vali::evaluate(fixture).expect("RR-0190: Journal append seal validate resolver v15");
    let second = relayring::capabilities::rr_0190_journal_append_seal_vali::evaluate(fixture).expect("RR-0190: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0190: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0190: stats visits every byte");
}
