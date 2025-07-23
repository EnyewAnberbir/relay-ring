//! Integration test for `RR-0195` (stream).
//! Journal append seal implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0195_journal_append_seal_impl_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc6, 0xc8];
    let direct = relayring::capabilities::rr_0195_journal_append_seal_impl::evaluate(fixture).expect("RR-0195: direct Journal append seal implement pipeline v20");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0195_journal_append_seal_impl::evaluate(&copied).expect("RR-0195: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0195: stream path must consume input");
}
