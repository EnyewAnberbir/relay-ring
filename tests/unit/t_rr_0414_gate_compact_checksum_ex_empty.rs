//! Integration test for `RR-0414` (empty).
//! Gate compact checksum export benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0414_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0414_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0414: empty input must fail for Gate compact checksum export benchmark reporter v19");
}
