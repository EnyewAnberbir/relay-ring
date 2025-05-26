//! Integration test for `RR-0763` (stability).
//! Extended: Journal OTLP bridge wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0763_journal_otlp_bridge_wire_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06];
    let full = relayring::capabilities::rr_0763_journal_otlp_bridge_wire_extended::evaluate(fixture).expect("RR-0763: bulk Extended: Journal OTLP bridge wire planner v13");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0763_journal_otlp_bridge_wire_extended::evaluate(&fixture[..end]).expect("RR-0763: stable prefix");
        assert!(partial.consumed <= end, "RR-0763: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0763: full prefix should match bulk checksum");
        }
    }
}
