//! Integration test for `RR-0903` (empty).
//! Extended: Gate compact checksum export refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0903_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0903_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0903: empty input must fail for Extended: Gate compact checksum export refactor mutator v8");
}
