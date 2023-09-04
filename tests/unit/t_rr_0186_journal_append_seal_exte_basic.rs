//! Integration test for `RR-0186` (basic).
//! Journal append seal extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0186_journal_append_seal_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbd, 0xbf];
    let first = relayring::capabilities::rr_0186_journal_append_seal_exte::evaluate(fixture).expect("RR-0186: Journal append seal extend codec v11");
    let second = relayring::capabilities::rr_0186_journal_append_seal_exte::evaluate(fixture).expect("RR-0186: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0186: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0186: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
