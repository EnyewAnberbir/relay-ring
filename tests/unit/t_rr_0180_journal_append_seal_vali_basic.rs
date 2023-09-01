//! Integration test for `RR-0180` (basic).
//! Journal append seal validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0180_journal_append_seal_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0xb9];
    let first = relayring::capabilities::rr_0180_journal_append_seal_vali::evaluate(fixture).expect("RR-0180: Journal append seal validate resolver v5");
    let second = relayring::capabilities::rr_0180_journal_append_seal_vali::evaluate(fixture).expect("RR-0180: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0180: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0180: stats visits every byte");
}
