//! Integration test for `RR-0336` (empty).
//! Gate surfaces append seek extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0336_gate_surfaces_append_see_empty() {
    assert!(relayring::capabilities::rr_0336_gate_surfaces_append_see::evaluate(&[]).is_err(), "RR-0336: empty input must fail for Gate surfaces append seek extend codec v1");
}
