//! Integration test for `RR-0910` (stream).
//! Extended: Gate compact checksum export validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0910_gate_compact_checksum_ex_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x97, 0x99];
    let direct = relayring::capabilities::rr_0910_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0910: direct Extended: Gate compact checksum export validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0910_gate_compact_checksum_ex_extended::evaluate(&copied).expect("RR-0910: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0910: stream path must consume input");
}
