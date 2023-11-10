//! Integration test for `RR-0691` (basic).
//! Extended: Journal append seal export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0691_journal_append_seal_expo_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xba, 0xbc];
    let first = relayring::capabilities::rr_0691_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0691: Extended: Journal append seal export adapter v16");
    let second = relayring::capabilities::rr_0691_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0691: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0691: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0691: scanner should emit domain hints");
}
