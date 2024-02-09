//! Integration test for `RR-0355` (bounds).
//! Gate surfaces append seek implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0355_gate_surfaces_append_see_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0355_gate_surfaces_append_see::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0355_gate_surfaces_append_see::evaluate(fixture).expect("RR-0355 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
