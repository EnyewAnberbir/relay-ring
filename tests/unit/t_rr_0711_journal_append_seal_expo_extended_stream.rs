//! Integration test for `RR-0711` (stream).
//! Extended: Journal append seal export adapter v36 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0711_journal_append_seal_expo_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xce, 0xd0];
    let direct = relayring::capabilities::rr_0711_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0711: direct Extended: Journal append seal export adapter v36");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0711_journal_append_seal_expo_extended::evaluate(&copied).expect("RR-0711: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0711: stream path must consume input");
}
