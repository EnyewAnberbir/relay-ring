//! Integration test for `RR-0214` (stream).
//! Journal append seal benchmark reporter v39 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0214_journal_append_seal_benc_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd9, 0xdb];
    let direct = relayring::capabilities::rr_0214_journal_append_seal_benc::evaluate(fixture).expect("RR-0214: direct Journal append seal benchmark reporter v39");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0214_journal_append_seal_benc::evaluate(&copied).expect("RR-0214: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0214: stream path must consume input");
}
