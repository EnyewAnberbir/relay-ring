//! Integration test for `RR-0189` (basic).
//! Journal append seal optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0189_journal_append_seal_opti_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0xc2];
    let first = relayring::capabilities::rr_0189_journal_append_seal_opti::evaluate(fixture).expect("RR-0189: Journal append seal optimize registry v14");
    let second = relayring::capabilities::rr_0189_journal_append_seal_opti::evaluate(fixture).expect("RR-0189: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0189: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0189: scanner should emit domain hints");
}
