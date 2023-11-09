//! Integration test for `RR-0681` (basic).
//! Extended: Journal append seal export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0681_journal_append_seal_expo_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb0, 0xb2];
    let first = relayring::capabilities::rr_0681_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0681: Extended: Journal append seal export adapter v6");
    let second = relayring::capabilities::rr_0681_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0681: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0681: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0681: window consumes the whole buffer");
}
