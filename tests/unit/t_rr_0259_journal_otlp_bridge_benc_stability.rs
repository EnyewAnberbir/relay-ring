//! Integration test for `RR-0259` (stability).
//! Journal OTLP bridge benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0259_journal_otlp_bridge_benc_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0a];
    let full = relayring::capabilities::rr_0259_journal_otlp_bridge_benc::evaluate(fixture).expect("RR-0259: bulk Journal OTLP bridge benchmark reporter v9");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0259_journal_otlp_bridge_benc::evaluate(&fixture[..end]).expect("RR-0259: stable prefix");
        assert!(partial.consumed <= end, "RR-0259: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0259: full prefix should match bulk checksum");
        }
    }
}
