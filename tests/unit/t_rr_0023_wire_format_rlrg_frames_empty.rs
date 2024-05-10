//! Integration test for `RR-0023` (empty).
//! Wire format RLRG frames wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0023_wire_format_rlrg_frames_empty() {
    assert!(relayring::capabilities::rr_0023_wire_format_rlrg_frames::evaluate(&[]).is_err(), "RR-0023: empty input must fail for Wire format RLRG frames wire planner v23");
}
