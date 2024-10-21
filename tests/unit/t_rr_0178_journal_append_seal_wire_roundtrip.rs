//! Integration test for `RR-0178` (roundtrip).
//! Journal append seal wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0178_journal_append_seal_wire_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0xb7];
    let a = relayring::capabilities::rr_0178_journal_append_seal_wire::evaluate(fixture).expect("RR-0178 first pass");
    let b = relayring::capabilities::rr_0178_journal_append_seal_wire::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
