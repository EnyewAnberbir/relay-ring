//! Integration test for `RR-0209` (basic).
//! Journal append seal optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0209_journal_append_seal_opti_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd4, 0xd6];
    let first = relayring::capabilities::rr_0209_journal_append_seal_opti::evaluate(fixture).expect("RR-0209: Journal append seal optimize registry v34");
    let second = relayring::capabilities::rr_0209_journal_append_seal_opti::evaluate(fixture).expect("RR-0209: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0209: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0209: scanner should emit domain hints");
}
