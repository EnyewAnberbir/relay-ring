//! Integration test for `RR-0017` (empty).
//! Wire format RLRG frames integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0017_wire_format_rlrg_frames_empty() {
    assert!(relayring::capabilities::rr_0017_wire_format_rlrg_frames::evaluate(&[]).is_err(), "RR-0017: empty input must fail for Wire format RLRG frames integrate validator v17");
}
