//! Integration test for `RR-0258` (bounds).
//! Journal OTLP bridge refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0258_journal_otlp_bridge_refa_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0258_journal_otlp_bridge_refa::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0258_journal_otlp_bridge_refa::evaluate(fixture).expect("RR-0258 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
