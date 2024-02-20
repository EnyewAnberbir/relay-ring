//! Integration test for `RR-0413` (bounds).
//! Gate compact checksum export refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0413_gate_compact_checksum_ex_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0413_gate_compact_checksum_ex::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0413_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0413 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
