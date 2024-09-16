//! Integration test for `RR-0924` (empty).
//! Extended: Gate compact checksum export benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0924_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0924_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0924: empty input must fail for Extended: Gate compact checksum export benchmark reporter v29");
}
