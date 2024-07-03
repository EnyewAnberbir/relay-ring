//! Integration test for `RR-0401` (empty).
//! Gate compact checksum export export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0401_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0401_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0401: empty input must fail for Gate compact checksum export export adapter v6");
}
