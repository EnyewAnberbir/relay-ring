//! Integration test for `RR-0901` (empty).
//! Extended: Gate compact checksum export export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0901_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0901_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0901: empty input must fail for Extended: Gate compact checksum export export adapter v6");
}
