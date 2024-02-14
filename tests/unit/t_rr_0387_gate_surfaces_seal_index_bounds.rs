//! Integration test for `RR-0387` (bounds).
//! Gate surfaces seal index harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0387_gate_surfaces_seal_index_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0387_gate_surfaces_seal_index::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0387_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0387 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
