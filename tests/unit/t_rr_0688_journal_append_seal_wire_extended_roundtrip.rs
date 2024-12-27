//! Integration test for `RR-0688` (roundtrip).
//! Extended: Journal append seal wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0688_journal_append_seal_wire_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0xb9];
    let a = relayring::capabilities::rr_0688_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0688 first pass");
    let b = relayring::capabilities::rr_0688_journal_append_seal_wire_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
