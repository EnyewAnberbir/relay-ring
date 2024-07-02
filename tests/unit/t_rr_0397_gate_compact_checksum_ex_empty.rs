//! Integration test for `RR-0397` (empty).
//! Gate compact checksum export harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0397_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0397_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0397: empty input must fail for Gate compact checksum export harden index v2");
}
