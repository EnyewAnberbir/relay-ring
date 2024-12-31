//! Integration test for `RR-0708` (roundtrip).
//! Extended: Journal append seal wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0708_journal_append_seal_wire_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcb, 0xcd];
    let a = relayring::capabilities::rr_0708_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0708 first pass");
    let b = relayring::capabilities::rr_0708_journal_append_seal_wire_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
