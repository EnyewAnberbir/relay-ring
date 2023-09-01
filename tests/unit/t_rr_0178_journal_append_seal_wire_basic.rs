//! Integration test for `RR-0178` (basic).
//! Journal append seal wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0178_journal_append_seal_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0xb7];
    let first = relayring::capabilities::rr_0178_journal_append_seal_wire::evaluate(fixture).expect("RR-0178: Journal append seal wire planner v3");
    let second = relayring::capabilities::rr_0178_journal_append_seal_wire::evaluate(fixture).expect("RR-0178: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0178: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0178: stats visits every byte");
}
