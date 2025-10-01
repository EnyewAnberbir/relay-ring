//! Integration test for `RR-0685` (stream).
//! Extended: Journal append seal implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0685_journal_append_seal_impl_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0xb6];
    let direct = relayring::capabilities::rr_0685_journal_append_seal_impl_extended::evaluate(fixture).expect("RR-0685: direct Extended: Journal append seal implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0685_journal_append_seal_impl_extended::evaluate(&copied).expect("RR-0685: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0685: stream path must consume input");
}
