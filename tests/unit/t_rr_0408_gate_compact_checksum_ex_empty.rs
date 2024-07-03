//! Integration test for `RR-0408` (empty).
//! Gate compact checksum export wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0408_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0408_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0408: empty input must fail for Gate compact checksum export wire planner v13");
}
