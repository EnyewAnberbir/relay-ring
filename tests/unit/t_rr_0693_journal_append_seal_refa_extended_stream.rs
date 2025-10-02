//! Integration test for `RR-0693` (stream).
//! Extended: Journal append seal refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0693_journal_append_seal_refa_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbc, 0xbe];
    let direct = relayring::capabilities::rr_0693_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0693: direct Extended: Journal append seal refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0693_journal_append_seal_refa_extended::evaluate(&copied).expect("RR-0693: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0693: stream path must consume input");
}
