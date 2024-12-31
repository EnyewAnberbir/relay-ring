//! Integration test for `RR-0711` (roundtrip).
//! Extended: Journal append seal export adapter v36 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0711_journal_append_seal_expo_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xce, 0xd0];
    let a = relayring::capabilities::rr_0711_journal_append_seal_expo_extended::evaluate(fixture).expect("RR-0711 first pass");
    let b = relayring::capabilities::rr_0711_journal_append_seal_expo_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
