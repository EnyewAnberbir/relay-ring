//! Integration test for `RR-0712` (basic).
//! Extended: Journal append seal integrate validator v37 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0712_journal_append_seal_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcf, 0xd1];
    let first = relayring::capabilities::rr_0712_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0712: Extended: Journal append seal integrate validator v37");
    let second = relayring::capabilities::rr_0712_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0712: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0712: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0712: stats visits every byte");
}
