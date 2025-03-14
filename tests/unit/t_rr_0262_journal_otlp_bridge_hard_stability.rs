//! Integration test for `RR-0262` (stability).
//! Journal OTLP bridge harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0262_journal_otlp_bridge_hard_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x0d];
    let full = relayring::capabilities::rr_0262_journal_otlp_bridge_hard::evaluate(fixture).expect("RR-0262: bulk Journal OTLP bridge harden index v12");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0262_journal_otlp_bridge_hard::evaluate(&fixture[..end]).expect("RR-0262: stable prefix");
        assert!(partial.consumed <= end, "RR-0262: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0262: full prefix should match bulk checksum");
        }
    }
}
