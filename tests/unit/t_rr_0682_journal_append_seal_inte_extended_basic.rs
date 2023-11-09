//! Integration test for `RR-0682` (basic).
//! Extended: Journal append seal integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0682_journal_append_seal_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb1, 0xb3];
    let first = relayring::capabilities::rr_0682_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0682: Extended: Journal append seal integrate validator v7");
    let second = relayring::capabilities::rr_0682_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0682: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0682: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0682: scanner should emit domain hints");
}
