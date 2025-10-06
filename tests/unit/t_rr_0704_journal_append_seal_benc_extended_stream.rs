//! Integration test for `RR-0704` (stream).
//! Extended: Journal append seal benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0704_journal_append_seal_benc_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc7, 0xc9];
    let direct = relayring::capabilities::rr_0704_journal_append_seal_benc_extended::evaluate(fixture).expect("RR-0704: direct Extended: Journal append seal benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0704_journal_append_seal_benc_extended::evaluate(&copied).expect("RR-0704: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0704: stream path must consume input");
}
