//! Integration test for `RR-0904` (empty).
//! Extended: Gate compact checksum export benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0904_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0904_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0904: empty input must fail for Extended: Gate compact checksum export benchmark reporter v9");
}
