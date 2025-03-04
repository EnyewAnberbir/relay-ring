//! Integration test for `RR-0186` (stability).
//! Journal append seal extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0186_journal_append_seal_exte_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbd, 0xbf];
    let full = relayring::capabilities::rr_0186_journal_append_seal_exte::evaluate(fixture).expect("RR-0186: bulk Journal append seal extend codec v11");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0186_journal_append_seal_exte::evaluate(&fixture[..end]).expect("RR-0186: stable prefix");
        assert!(partial.consumed <= end, "RR-0186: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0186: full prefix should match bulk checksum");
        }
    }
}
