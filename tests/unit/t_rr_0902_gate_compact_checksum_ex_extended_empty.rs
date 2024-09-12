//! Integration test for `RR-0902` (empty).
//! Extended: Gate compact checksum export integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0902_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0902_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0902: empty input must fail for Extended: Gate compact checksum export integrate validator v7");
}
