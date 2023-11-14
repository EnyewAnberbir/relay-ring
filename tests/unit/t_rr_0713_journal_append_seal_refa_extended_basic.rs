//! Integration test for `RR-0713` (basic).
//! Extended: Journal append seal refactor mutator v38 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0713_journal_append_seal_refa_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0, 0xd2];
    let first = relayring::capabilities::rr_0713_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0713: Extended: Journal append seal refactor mutator v38");
    let second = relayring::capabilities::rr_0713_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0713: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0713: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0713: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
