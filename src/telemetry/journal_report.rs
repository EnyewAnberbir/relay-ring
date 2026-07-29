use crate::journal_workflow;
use crate::wire::{decode, validate};
use std::fmt;

#[derive(Debug, Clone)]
pub struct JournalReport {
    pub record_count: usize,
    pub ring_tail: u64,
    pub violations: usize,
    pub payload_bytes: usize,
    pub digest: u64,
}

impl fmt::Display for JournalReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "records={} tail={} violations={} payload_bytes={} digest={:#x}",
            self.record_count, self.ring_tail, self.violations, self.payload_bytes, self.digest
        )
    }
}

pub fn run(data: &[u8]) -> Result<JournalReport, String> {
    let _ = journal_workflow::process_journal_bytes(data);
    let frame = decode::decode(data)?;
    let issues = validate::validate_frame(data, true)?;
    let payload_bytes = frame.records.iter().map(|r| r.payload.len()).sum();
    let mut digest = frame.head_seq ^ frame.tail_seq;
    digest ^= (frame.records.len() as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    Ok(JournalReport {
        record_count: frame.section_count(),
        ring_tail: frame.tail_seq,
        violations: issues.len(),
        payload_bytes,
        digest,
    })
}
