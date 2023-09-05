//! Integration test for `RR-0198` (basic).
//! Journal append seal wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0198_journal_append_seal_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc9, 0xcb];
    let first = relayring::capabilities::rr_0198_journal_append_seal_wire::evaluate(fixture).expect("RR-0198: Journal append seal wire planner v23");
    let second = relayring::capabilities::rr_0198_journal_append_seal_wire::evaluate(fixture).expect("RR-0198: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0198: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0198: scanner should emit domain hints");
}
