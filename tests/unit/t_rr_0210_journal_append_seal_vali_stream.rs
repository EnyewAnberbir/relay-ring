//! Integration test for `RR-0210` (stream).
//! Journal append seal validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0210_journal_append_seal_vali_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd5, 0xd7];
    let direct = relayring::capabilities::rr_0210_journal_append_seal_vali::evaluate(fixture).expect("RR-0210: direct Journal append seal validate resolver v35");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0210_journal_append_seal_vali::evaluate(&copied).expect("RR-0210: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0210: stream path must consume input");
}
