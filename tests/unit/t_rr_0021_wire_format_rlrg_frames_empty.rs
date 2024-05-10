//! Integration test for `RR-0021` (empty).
//! Wire format RLRG frames extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0021_wire_format_rlrg_frames_empty() {
    assert!(relayring::capabilities::rr_0021_wire_format_rlrg_frames::evaluate(&[]).is_err(), "RR-0021: empty input must fail for Wire format RLRG frames extend codec v21");
}
