//! Integration test for `RR-0180` (stream).
//! Journal append seal validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0180_journal_append_seal_vali_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0xb9];
    let direct = relayring::capabilities::rr_0180_journal_append_seal_vali::evaluate(fixture).expect("RR-0180: direct Journal append seal validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0180_journal_append_seal_vali::evaluate(&copied).expect("RR-0180: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0180: stream path must consume input");
}
