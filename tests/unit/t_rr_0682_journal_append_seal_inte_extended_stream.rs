//! Integration test for `RR-0682` (stream).
//! Extended: Journal append seal integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0682_journal_append_seal_inte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb1, 0xb3];
    let direct = relayring::capabilities::rr_0682_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0682: direct Extended: Journal append seal integrate validator v7");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0682_journal_append_seal_inte_extended::evaluate(&copied).expect("RR-0682: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0682: stream path must consume input");
}
