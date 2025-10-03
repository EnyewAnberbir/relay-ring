//! Integration test for `RR-0696` (stream).
//! Extended: Journal append seal extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0696_journal_append_seal_exte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbf, 0xc1];
    let direct = relayring::capabilities::rr_0696_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0696: direct Extended: Journal append seal extend codec v21");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0696_journal_append_seal_exte_extended::evaluate(&copied).expect("RR-0696: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0696: stream path must consume input");
}
