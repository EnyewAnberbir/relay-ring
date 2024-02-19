//! Integration test for `RR-0407` (bounds).
//! Gate compact checksum export harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0407_gate_compact_checksum_ex_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0407_gate_compact_checksum_ex::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0407_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0407 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
