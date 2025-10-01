//! Integration test for `RR-0679` (stream).
//! Extended: Journal append seal optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0679_journal_append_seal_opti_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xae, 0xb0];
    let direct = relayring::capabilities::rr_0679_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0679: direct Extended: Journal append seal optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0679_journal_append_seal_opti_extended::evaluate(&copied).expect("RR-0679: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0679: stream path must consume input");
}
