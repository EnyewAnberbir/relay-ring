//! Integration test for `RR-0186` (stream).
//! Journal append seal extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0186_journal_append_seal_exte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbd, 0xbf];
    let direct = relayring::capabilities::rr_0186_journal_append_seal_exte::evaluate(fixture).expect("RR-0186: direct Journal append seal extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0186_journal_append_seal_exte::evaluate(&copied).expect("RR-0186: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0186: stream path must consume input");
}
