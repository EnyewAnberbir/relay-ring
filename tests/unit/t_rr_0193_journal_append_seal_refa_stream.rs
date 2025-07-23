//! Integration test for `RR-0193` (stream).
//! Journal append seal refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0193_journal_append_seal_refa_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc4, 0xc6];
    let direct = relayring::capabilities::rr_0193_journal_append_seal_refa::evaluate(fixture).expect("RR-0193: direct Journal append seal refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0193_journal_append_seal_refa::evaluate(&copied).expect("RR-0193: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0193: stream path must consume input");
}
