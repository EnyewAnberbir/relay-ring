//! Integration test for `RR-0251` (stability).
//! Journal OTLP bridge extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0251_journal_otlp_bridge_exte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe, 0x02];
    let full = relayring::capabilities::rr_0251_journal_otlp_bridge_exte::evaluate(fixture).expect("RR-0251: bulk Journal OTLP bridge extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0251_journal_otlp_bridge_exte::evaluate(&fixture[..end]).expect("RR-0251: stable prefix");
        assert!(partial.consumed <= end, "RR-0251: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0251: full prefix should match bulk checksum");
        }
    }
}
