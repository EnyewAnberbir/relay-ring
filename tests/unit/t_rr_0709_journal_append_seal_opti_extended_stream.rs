//! Integration test for `RR-0709` (stream).
//! Extended: Journal append seal optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0709_journal_append_seal_opti_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0xce];
    let direct = relayring::capabilities::rr_0709_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0709: direct Extended: Journal append seal optimize registry v34");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0709_journal_append_seal_opti_extended::evaluate(&copied).expect("RR-0709: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0709: stream path must consume input");
}
