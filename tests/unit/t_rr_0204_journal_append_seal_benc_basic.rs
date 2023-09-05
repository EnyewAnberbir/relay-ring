//! Integration test for `RR-0204` (basic).
//! Journal append seal benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0204_journal_append_seal_benc_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcf, 0xd1];
    let first = relayring::capabilities::rr_0204_journal_append_seal_benc::evaluate(fixture).expect("RR-0204: Journal append seal benchmark reporter v29");
    let second = relayring::capabilities::rr_0204_journal_append_seal_benc::evaluate(fixture).expect("RR-0204: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0204: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0204: stats visits every byte");
}
