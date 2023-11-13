//! Integration test for `RR-0702` (basic).
//! Extended: Journal append seal integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0702_journal_append_seal_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc5, 0xc7];
    let first = relayring::capabilities::rr_0702_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0702: Extended: Journal append seal integrate validator v27");
    let second = relayring::capabilities::rr_0702_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0702: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0702: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0702: window consumes the whole buffer");
}
