//! Integration test for `RR-0192` (stream).
//! Journal append seal integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0192_journal_append_seal_inte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0xc5];
    let direct = relayring::capabilities::rr_0192_journal_append_seal_inte::evaluate(fixture).expect("RR-0192: direct Journal append seal integrate validator v17");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0192_journal_append_seal_inte::evaluate(&copied).expect("RR-0192: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0192: stream path must consume input");
}
