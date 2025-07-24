//! Integration test for `RR-0203` (stream).
//! Journal append seal refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0203_journal_append_seal_refa_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xce, 0xd0];
    let direct = relayring::capabilities::rr_0203_journal_append_seal_refa::evaluate(fixture).expect("RR-0203: direct Journal append seal refactor mutator v28");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0203_journal_append_seal_refa::evaluate(&copied).expect("RR-0203: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0203: stream path must consume input");
}
