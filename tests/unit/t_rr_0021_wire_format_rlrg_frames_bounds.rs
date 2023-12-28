//! Integration test for `RR-0021` (bounds).
//! Wire format RLRG frames extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0021_wire_format_rlrg_frames_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0021_wire_format_rlrg_frames::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0021_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0021 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
