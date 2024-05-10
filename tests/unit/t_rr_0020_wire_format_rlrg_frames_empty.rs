//! Integration test for `RR-0020` (empty).
//! Wire format RLRG frames implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0020_wire_format_rlrg_frames_empty() {
    assert!(relayring::capabilities::rr_0020_wire_format_rlrg_frames::evaluate(&[]).is_err(), "RR-0020: empty input must fail for Wire format RLRG frames implement pipeline v20");
}
