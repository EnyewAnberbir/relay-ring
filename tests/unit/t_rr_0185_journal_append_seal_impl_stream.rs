//! Integration test for `RR-0185` (stream).
//! Journal append seal implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0185_journal_append_seal_impl_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbc, 0xbe];
    let direct = relayring::capabilities::rr_0185_journal_append_seal_impl::evaluate(fixture).expect("RR-0185: direct Journal append seal implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0185_journal_append_seal_impl::evaluate(&copied).expect("RR-0185: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0185: stream path must consume input");
}
