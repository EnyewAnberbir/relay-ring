//! Integration test for `RR-0691` (stream).
//! Extended: Journal append seal export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0691_journal_append_seal_expo_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xba, 0xbc];
    let direct = relayring::capabilities::rr_0691_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0691: direct Extended: Journal append seal export adapter v16");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0691_journal_append_seal_expo_extended::evaluate(&copied).expect("RR-0691: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0691: stream path must consume input");
}
