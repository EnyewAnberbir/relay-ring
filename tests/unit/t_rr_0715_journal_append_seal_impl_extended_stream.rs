//! Integration test for `RR-0715` (stream).
//! Extended: Journal append seal implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0715_journal_append_seal_impl_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd2, 0xd4];
    let direct = relayring::capabilities::rr_0715_journal_append_seal_impl_extended::evaluate(fixture).expect("RR-0715: direct Extended: Journal append seal implement pipeline v40");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0715_journal_append_seal_impl_extended::evaluate(&copied).expect("RR-0715: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0715: stream path must consume input");
}
