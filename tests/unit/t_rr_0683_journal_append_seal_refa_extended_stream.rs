//! Integration test for `RR-0683` (stream).
//! Extended: Journal append seal refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0683_journal_append_seal_refa_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb2, 0xb4];
    let direct = relayring::capabilities::rr_0683_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0683: direct Extended: Journal append seal refactor mutator v8");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0683_journal_append_seal_refa_extended::evaluate(&copied).expect("RR-0683: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0683: stream path must consume input");
}
