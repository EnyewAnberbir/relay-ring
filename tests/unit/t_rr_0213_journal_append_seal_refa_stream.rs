//! Integration test for `RR-0213` (stream).
//! Journal append seal refactor mutator v38 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0213_journal_append_seal_refa_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0xda];
    let direct = relayring::capabilities::rr_0213_journal_append_seal_refa::evaluate(fixture).expect("RR-0213: direct Journal append seal refactor mutator v38");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0213_journal_append_seal_refa::evaluate(&copied).expect("RR-0213: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0213: stream path must consume input");
}
