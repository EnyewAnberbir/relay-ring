#[derive(Debug, Clone)]
pub struct JournalRecord {
    pub seq: u64,
    pub timestamp: u64,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RelayringFrame {
    pub head_seq: u64,
    pub tail_seq: u64,
    pub records: Vec<JournalRecord>,
}

impl RelayringFrame {
    pub fn version(&self) -> u8 { 1 }
    pub fn section_count(&self) -> usize { self.records.len() }
    pub fn payload_for(&self, idx: usize) -> Option<&[u8]> {
        self.records.get(idx).map(|r| r.payload.as_slice())
    }
    pub fn sections(&self) -> &[JournalRecord] { &self.records }
}
