//! Integration test for `RR-0203` (basic).
//! Journal append seal refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0203_journal_append_seal_refa_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xce, 0xd0];
    let first = relayring::capabilities::rr_0203_journal_append_seal_refa::evaluate(fixture).expect("RR-0203: Journal append seal refactor mutator v28");
    let second = relayring::capabilities::rr_0203_journal_append_seal_refa::evaluate(fixture).expect("RR-0203: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0203: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0203: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
