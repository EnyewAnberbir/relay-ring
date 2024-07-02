//! Integration test for `RR-0399` (empty).
//! Gate compact checksum export optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0399_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0399_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0399: empty input must fail for Gate compact checksum export optimize registry v4");
}
