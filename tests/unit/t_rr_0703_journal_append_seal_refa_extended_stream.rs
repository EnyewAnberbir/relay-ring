//! Integration test for `RR-0703` (stream).
//! Extended: Journal append seal refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0703_journal_append_seal_refa_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc6, 0xc8];
    let direct = relayring::capabilities::rr_0703_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0703: direct Extended: Journal append seal refactor mutator v28");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0703_journal_append_seal_refa_extended::evaluate(&copied).expect("RR-0703: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0703: stream path must consume input");
}
