//! Integration test for `RR-0179` (stream).
//! Journal append seal optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0179_journal_append_seal_opti_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb6, 0xb8];
    let direct = relayring::capabilities::rr_0179_journal_append_seal_opti::evaluate(fixture).expect("RR-0179: direct Journal append seal optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0179_journal_append_seal_opti::evaluate(&copied).expect("RR-0179: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0179: stream path must consume input");
}
