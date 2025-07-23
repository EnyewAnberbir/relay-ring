//! Integration test for `RR-0194` (stream).
//! Journal append seal benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0194_journal_append_seal_benc_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc5, 0xc7];
    let direct = relayring::capabilities::rr_0194_journal_append_seal_benc::evaluate(fixture).expect("RR-0194: direct Journal append seal benchmark reporter v19");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0194_journal_append_seal_benc::evaluate(&copied).expect("RR-0194: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0194: stream path must consume input");
}
