//! Integration test for `RR-0004` (empty).
//! Wire format RLRG frames optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0004_wire_format_rlrg_frames_empty() {
    assert!(relayring::capabilities::rr_0004_wire_format_rlrg_frames::evaluate(&[]).is_err(), "RR-0004: empty input must fail for Wire format RLRG frames optimize registry v4");
}
