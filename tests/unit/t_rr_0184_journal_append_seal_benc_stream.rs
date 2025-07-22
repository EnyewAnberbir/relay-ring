//! Integration test for `RR-0184` (stream).
//! Journal append seal benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0184_journal_append_seal_benc_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbb, 0xbd];
    let direct = relayring::capabilities::rr_0184_journal_append_seal_benc::evaluate(fixture).expect("RR-0184: direct Journal append seal benchmark reporter v9");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0184_journal_append_seal_benc::evaluate(&copied).expect("RR-0184: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0184: stream path must consume input");
}
