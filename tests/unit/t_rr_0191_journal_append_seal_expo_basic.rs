//! Integration test for `RR-0191` (basic).
//! Journal append seal export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0191_journal_append_seal_expo_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xc4];
    let first = relayring::capabilities::rr_0191_journal_append_seal_expo::evaluate(fixture).expect("RR-0191: Journal append seal export adapter v16");
    let second = relayring::capabilities::rr_0191_journal_append_seal_expo::evaluate(fixture).expect("RR-0191: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0191: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0191: scanner should emit domain hints");
}
