//! Integration test for `RR-0424` (empty).
//! Gate compact checksum export benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0424_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0424_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0424: empty input must fail for Gate compact checksum export benchmark reporter v29");
}
