//! Integration test for `RR-0710` (basic).
//! Extended: Journal append seal validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0710_journal_append_seal_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0xcf];
    let first = relayring::capabilities::rr_0710_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0710: Extended: Journal append seal validate resolver v35");
    let second = relayring::capabilities::rr_0710_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0710: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0710: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0710: scanner should emit domain hints");
}
