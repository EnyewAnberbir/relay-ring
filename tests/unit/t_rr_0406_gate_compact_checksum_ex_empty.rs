//! Integration test for `RR-0406` (empty).
//! Gate compact checksum export extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0406_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0406_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0406: empty input must fail for Gate compact checksum export extend codec v11");
}
