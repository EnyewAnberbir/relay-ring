//! Integration test for `RR-0215` (stream).
//! Journal append seal implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0215_journal_append_seal_impl_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xda, 0xdc];
    let direct = relayring::capabilities::rr_0215_journal_append_seal_impl::evaluate(fixture).expect("RR-0215: direct Journal append seal implement pipeline v40");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0215_journal_append_seal_impl::evaluate(&copied).expect("RR-0215: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0215: stream path must consume input");
}
