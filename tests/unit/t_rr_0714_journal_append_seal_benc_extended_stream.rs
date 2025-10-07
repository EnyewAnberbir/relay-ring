//! Integration test for `RR-0714` (stream).
//! Extended: Journal append seal benchmark reporter v39 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0714_journal_append_seal_benc_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd1, 0xd3];
    let direct = relayring::capabilities::rr_0714_journal_append_seal_benc_extended::evaluate(fixture).expect("RR-0714: direct Extended: Journal append seal benchmark reporter v39");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0714_journal_append_seal_benc_extended::evaluate(&copied).expect("RR-0714: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0714: stream path must consume input");
}
