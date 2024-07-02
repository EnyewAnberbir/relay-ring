//! Integration test for `RR-0396` (empty).
//! Gate compact checksum export extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0396_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0396_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0396: empty input must fail for Gate compact checksum export extend codec v1");
}
