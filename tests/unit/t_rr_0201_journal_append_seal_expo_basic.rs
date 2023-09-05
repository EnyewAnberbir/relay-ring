//! Integration test for `RR-0201` (basic).
//! Journal append seal export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0201_journal_append_seal_expo_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0xce];
    let first = relayring::capabilities::rr_0201_journal_append_seal_expo::evaluate(fixture).expect("RR-0201: Journal append seal export adapter v26");
    let second = relayring::capabilities::rr_0201_journal_append_seal_expo::evaluate(fixture).expect("RR-0201: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0201: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0201: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
