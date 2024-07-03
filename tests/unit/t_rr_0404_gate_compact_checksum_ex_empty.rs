//! Integration test for `RR-0404` (empty).
//! Gate compact checksum export benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0404_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0404_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0404: empty input must fail for Gate compact checksum export benchmark reporter v9");
}
