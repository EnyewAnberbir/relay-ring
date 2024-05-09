//! Integration test for `RR-0008` (empty).
//! Wire format RLRG frames refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0008_wire_format_rlrg_frames_empty() {
    assert!(relayring::capabilities::rr_0008_wire_format_rlrg_frames::evaluate(&[]).is_err(), "RR-0008: empty input must fail for Wire format RLRG frames refactor mutator v8");
}
