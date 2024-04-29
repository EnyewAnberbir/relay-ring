//! Integration test for `RR-0924` (bounds).
//! Extended: Gate compact checksum export benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0924_gate_compact_checksum_ex_extended_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0924_gate_compact_checksum_ex_extended::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0924_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0924 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
