//! Integration test for `RR-0254` (stability).
//! Journal OTLP bridge optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0254_journal_otlp_bridge_opti_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x05];
    let full = relayring::capabilities::rr_0254_journal_otlp_bridge_opti::evaluate(fixture).expect("RR-0254: bulk Journal OTLP bridge optimize registry v4");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0254_journal_otlp_bridge_opti::evaluate(&fixture[..end]).expect("RR-0254: stable prefix");
        assert!(partial.consumed <= end, "RR-0254: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0254: full prefix should match bulk checksum");
        }
    }
}
