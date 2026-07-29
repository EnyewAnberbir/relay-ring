//! Behavioral checks that reject shallow "skip crash" / "clear early" patches.
//! These assert observable journal workflow stats, not merely crash absence.

use relayring::journal_workflow::{build_session, process_journal_bytes};
use relayring::wire::decode;

fn encode_rlrg(payloads: &[&[u8]]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"RLRG");
    out.extend_from_slice(&[0, 0, 0, 0]);
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&(payloads.len() as u32).to_le_bytes());
    for (i, p) in payloads.iter().enumerate() {
        out.extend_from_slice(&(i as u64).to_le_bytes());
        out.extend_from_slice(&(1_700_000_000u64 + i as u64).to_le_bytes());
        out.extend_from_slice(&(p.len() as u32).to_le_bytes());
        out.extend_from_slice(p);
    }
    out
}

#[test]
fn empty_journal_reports_zero_records() {
    let data = encode_rlrg(&[]);
    let stats = process_journal_bytes(&data).expect("empty ok");
    assert_eq!(stats.records, 0);
    assert_eq!(stats.append_marks, 0);
}

#[test]
fn append_and_seal_marks_accumulate() {
    let a = vec![0x11u8; 40];
    let s = vec![0x22u8; 44];
    let data = encode_rlrg(&[a.as_slice(), s.as_slice()]);
    let frame = decode::decode(&data).unwrap();
    let session = build_session(&frame);
    assert!(session.append_marks >= 1);
    assert!(session.seal_rounds >= 1);
    assert!(session.payload_bytes >= 80);
}

#[test]
fn workflow_digest_stable_for_same_bytes() {
    let p0 = vec![0x70u8; 48];
    let p1 = vec![0x11u8; 36];
    let data = encode_rlrg(&[p0.as_slice(), p1.as_slice()]);
    let a = process_journal_bytes(&data).unwrap();
    let b = process_journal_bytes(&data).unwrap();
    assert_eq!(a.digest, b.digest);
    assert_eq!(a.records, 2);
}

#[test]
fn short_input_rejected_before_view_establish() {
    let err = process_journal_bytes(b"RLRG").unwrap_err();
    assert!(err.contains("rlrg") || err.contains("journal") || !err.is_empty());
}
