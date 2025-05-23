//! Integration test for `RR-0753` (stability).
//! Extended: Journal OTLP bridge wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0753_journal_otlp_bridge_wire_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0xfa];
    let full = relayring::capabilities::rr_0753_journal_otlp_bridge_wire_extended::evaluate(fixture).expect("RR-0753: bulk Extended: Journal OTLP bridge wire planner v3");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0753_journal_otlp_bridge_wire_extended::evaluate(&fixture[..end]).expect("RR-0753: stable prefix");
        assert!(partial.consumed <= end, "RR-0753: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0753: full prefix should match bulk checksum");
        }
    }
}
