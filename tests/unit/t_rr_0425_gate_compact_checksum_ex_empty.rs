//! Integration test for `RR-0425` (empty).
//! Gate compact checksum export implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0425_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0425_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0425: empty input must fail for Gate compact checksum export implement pipeline v30");
}
