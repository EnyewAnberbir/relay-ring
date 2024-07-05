//! Integration test for `RR-0423` (empty).
//! Gate compact checksum export refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0423_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0423_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0423: empty input must fail for Gate compact checksum export refactor mutator v28");
}
