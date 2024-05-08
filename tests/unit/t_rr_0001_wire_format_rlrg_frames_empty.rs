//! Integration test for `RR-0001` (empty).
//! Wire format RLRG frames extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0001_wire_format_rlrg_frames_empty() {
    assert!(relayring::capabilities::rr_0001_wire_format_rlrg_frames::evaluate(&[]).is_err(), "RR-0001: empty input must fail for Wire format RLRG frames extend codec v1");
}
