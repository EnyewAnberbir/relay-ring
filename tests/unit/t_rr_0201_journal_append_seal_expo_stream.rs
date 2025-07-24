//! Integration test for `RR-0201` (stream).
//! Journal append seal export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0201_journal_append_seal_expo_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0xce];
    let direct = relayring::capabilities::rr_0201_journal_append_seal_expo::evaluate(fixture).expect("RR-0201: direct Journal append seal export adapter v26");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0201_journal_append_seal_expo::evaluate(&copied).expect("RR-0201: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0201: stream path must consume input");
}
