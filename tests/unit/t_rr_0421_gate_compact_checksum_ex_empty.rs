//! Integration test for `RR-0421` (empty).
//! Gate compact checksum export export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0421_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0421_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0421: empty input must fail for Gate compact checksum export export adapter v26");
}
