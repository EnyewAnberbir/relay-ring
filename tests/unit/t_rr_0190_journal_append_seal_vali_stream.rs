//! Integration test for `RR-0190` (stream).
//! Journal append seal validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0190_journal_append_seal_vali_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc1, 0xc3];
    let direct = relayring::capabilities::rr_0190_journal_append_seal_vali::evaluate(fixture).expect("RR-0190: direct Journal append seal validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0190_journal_append_seal_vali::evaluate(&copied).expect("RR-0190: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0190: stream path must consume input");
}
