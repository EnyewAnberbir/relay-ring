//! Integration test for `RR-0896` (empty).
//! Extended: Gate compact checksum export extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0896_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0896_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0896: empty input must fail for Extended: Gate compact checksum export extend codec v1");
}
