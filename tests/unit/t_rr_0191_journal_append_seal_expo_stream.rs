//! Integration test for `RR-0191` (stream).
//! Journal append seal export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0191_journal_append_seal_expo_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xc4];
    let direct = relayring::capabilities::rr_0191_journal_append_seal_expo::evaluate(fixture).expect("RR-0191: direct Journal append seal export adapter v16");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0191_journal_append_seal_expo::evaluate(&copied).expect("RR-0191: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0191: stream path must consume input");
}
