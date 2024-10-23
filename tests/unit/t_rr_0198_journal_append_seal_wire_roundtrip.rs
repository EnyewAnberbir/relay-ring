//! Integration test for `RR-0198` (roundtrip).
//! Journal append seal wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0198_journal_append_seal_wire_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc9, 0xcb];
    let a = relayring::capabilities::rr_0198_journal_append_seal_wire::evaluate(fixture).expect("RR-0198 first pass");
    let b = relayring::capabilities::rr_0198_journal_append_seal_wire::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
