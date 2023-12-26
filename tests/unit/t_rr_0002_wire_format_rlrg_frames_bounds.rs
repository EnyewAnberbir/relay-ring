//! Integration test for `RR-0002` (bounds).
//! Wire format RLRG frames harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0002_wire_format_rlrg_frames_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0002_wire_format_rlrg_frames::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0002_wire_format_rlrg_frames::evaluate(fixture).expect("RR-0002 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
