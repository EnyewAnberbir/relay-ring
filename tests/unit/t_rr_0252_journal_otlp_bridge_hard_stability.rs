//! Integration test for `RR-0252` (stability).
//! Journal OTLP bridge harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0252_journal_otlp_bridge_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03];
    let full = relayring::capabilities::rr_0252_journal_otlp_bridge_hard::evaluate(fixture).expect("RR-0252: bulk Journal OTLP bridge harden index v2");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0252_journal_otlp_bridge_hard::evaluate(&fixture[..end]).expect("RR-0252: stable prefix");
        assert!(partial.consumed <= end, "RR-0252: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0252: full prefix should match bulk checksum");
        }
    }
}
