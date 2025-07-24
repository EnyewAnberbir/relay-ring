//! Integration test for `RR-0209` (stream).
//! Journal append seal optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0209_journal_append_seal_opti_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd4, 0xd6];
    let direct = relayring::capabilities::rr_0209_journal_append_seal_opti::evaluate(fixture).expect("RR-0209: direct Journal append seal optimize registry v34");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0209_journal_append_seal_opti::evaluate(&copied).expect("RR-0209: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0209: stream path must consume input");
}
