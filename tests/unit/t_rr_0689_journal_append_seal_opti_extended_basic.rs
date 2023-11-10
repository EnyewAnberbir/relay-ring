//! Integration test for `RR-0689` (basic).
//! Extended: Journal append seal optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0689_journal_append_seal_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb8, 0xba];
    let first = relayring::capabilities::rr_0689_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0689: Extended: Journal append seal optimize registry v14");
    let second = relayring::capabilities::rr_0689_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0689: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0689: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0689: window consumes the whole buffer");
}
