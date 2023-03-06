//! Deployment profile `cold_archive` for relayring.

#[derive(Debug, Clone)]
pub struct ColdArchiveSettings {
    pub ring_segment_bytes: u32,
    pub batch_export_rows: u16,
    pub fsync_every_n: u8,
}

impl ColdArchiveSettings {
    pub fn load() -> Self {
        Self {
            ring_segment_bytes: 524288,
            batch_export_rows: 256,
            fsync_every_n: 8,
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.ring_segment_bytes == 0 { return Err("ring_segment_bytes"); }
        Ok(())
    }
}
