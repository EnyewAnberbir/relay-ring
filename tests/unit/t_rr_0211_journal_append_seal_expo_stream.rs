//! Integration test for `RR-0211` (stream).
//! Journal append seal export adapter v36 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0211_journal_append_seal_expo_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd6, 0xd8];
    let direct = relayring::capabilities::rr_0211_journal_append_seal_expo::evaluate(fixture).expect("RR-0211: direct Journal append seal export adapter v36");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0211_journal_append_seal_expo::evaluate(&copied).expect("RR-0211: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0211: stream path must consume input");
}
