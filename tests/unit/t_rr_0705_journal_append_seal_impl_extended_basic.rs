//! Integration test for `RR-0705` (basic).
//! Extended: Journal append seal implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0705_journal_append_seal_impl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0xca];
    let first = relayring::capabilities::rr_0705_journal_append_seal_impl_extended::evaluate(fixture).expect("RR-0705: Extended: Journal append seal implement pipeline v30");
    let second = relayring::capabilities::rr_0705_journal_append_seal_impl_extended::evaluate(fixture).expect("RR-0705: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0705: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0705: window consumes the whole buffer");
}
