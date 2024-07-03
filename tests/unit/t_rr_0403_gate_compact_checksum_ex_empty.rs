//! Integration test for `RR-0403` (empty).
//! Gate compact checksum export refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0403_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0403_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0403: empty input must fail for Gate compact checksum export refactor mutator v8");
}
