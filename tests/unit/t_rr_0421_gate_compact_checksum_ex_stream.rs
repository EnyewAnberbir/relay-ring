//! Integration test for `RR-0421` (stream).
//! Gate compact checksum export export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0421_gate_compact_checksum_ex_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xac];
    let direct = relayring::capabilities::rr_0421_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0421: direct Gate compact checksum export export adapter v26");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0421_gate_compact_checksum_ex::evaluate(&copied).expect("RR-0421: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0421: stream path must consume input");
}
