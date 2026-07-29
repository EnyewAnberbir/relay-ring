//! Multi-stage journal processing used by the public fuzz targets.
//! Decode → establish derived views → repeated seal/compact rebuilds → late observers.

use crate::export_views;
use crate::wire::decode;
use crate::wire::frame::RelayringFrame;
use crate::wire::validate;

#[derive(Clone, Debug, Default)]
pub struct JournalSession {
    pub append_marks: usize,
    pub seal_rounds: usize,
    pub checkpoint_marks: usize,
    pub generation_advances: usize,
    pub payload_bytes: usize,
    pub compact_ops: usize,
    pub export_ops: usize,
}

#[derive(Clone, Debug, Default)]
pub struct WorkflowStats {
    pub records: usize,
    pub append_marks: usize,
    pub seal_rounds: usize,
    pub payload_bytes: usize,
    pub digest: u64,
}

fn classify_payload(payload: &[u8]) -> (usize, usize, usize, usize, usize) {
    let mut appends = 0usize;
    let mut seals = 0usize;
    let mut checkpoints = 0usize;
    let mut gens = 0usize;
    let mut compacts = 0usize;
    if payload.is_empty() {
        return (0, 0, 0, 0, 0);
    }
    let tag = payload[0];
    match tag {
        0x10..=0x1f => appends = 1,
        0x20..=0x2f => seals = 1,
        0x30..=0x3f => checkpoints = 1,
        0x40..=0x4f => gens = 1,
        0x50..=0x5f => compacts = 1,
        // Bootstrap / padding / deferred markers — never contribute pressure.
        0x60..=0x7f => return (0, 0, 0, 0, 0),
        _ => {
            if payload.len() >= 8 {
                appends = (payload[1] & 1) as usize;
                seals = ((payload[2] >> 1) & 1) as usize;
                checkpoints = ((payload[3] >> 2) & 1) as usize;
                gens = ((payload[4] >> 3) & 1) as usize;
                compacts = ((payload[5] >> 4) & 1) as usize;
            }
        }
    }
    // Repetition-driven pressure only on classified operational records.
    if payload.len() > 48 && matches!(tag, 0x10..=0x5f) {
        seals += payload[16..]
            .chunks(7)
            .filter(|c| c.iter().any(|&b| b == 0xA5))
            .count()
            .min(3);
        appends += payload[8..]
            .windows(3)
            .filter(|w| w[0] == w[2])
            .count()
            .min(2);
    }
    (appends, seals, checkpoints, gens, compacts)
}

pub fn build_session(frame: &RelayringFrame) -> JournalSession {
    let mut session = JournalSession::default();
    for i in 0..frame.section_count() {
        let Some(payload) = frame.payload_for(i) else { continue };
        session.payload_bytes = session.payload_bytes.saturating_add(payload.len());
        let (a, s, c, g, k) = classify_payload(payload);
        session.append_marks = session.append_marks.saturating_add(a);
        session.seal_rounds = session.seal_rounds.saturating_add(s);
        session.checkpoint_marks = session.checkpoint_marks.saturating_add(c);
        session.generation_advances = session.generation_advances.saturating_add(g);
        session.compact_ops = session.compact_ops.saturating_add(k);
        if payload.len() >= 4 && payload[0] == 0x61 {
            session.export_ops = session.export_ops.saturating_add(1);
        }
    }
    if frame.head_seq > frame.tail_seq {
        session.checkpoint_marks = session.checkpoint_marks.saturating_add(1);
    }
    session
}

fn frame_digest(frame: &RelayringFrame) -> u64 {
    let mut d = frame.head_seq ^ frame.tail_seq.rotate_left(7);
    d ^= (frame.records.len() as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    for (i, rec) in frame.records.iter().enumerate() {
        d = d
            .wrapping_mul(0x1000_001b)
            .wrapping_add(rec.seq)
            .wrapping_add(rec.timestamp.rotate_left((i as u32) & 31))
            .wrapping_add(rec.payload.len() as u64);
        if let Some(&b) = rec.payload.first() {
            d ^= (b as u64) << ((i & 7) * 8);
        }
    }
    d
}

/// Bounded late-stage workflow: establishment, multi-round rebuild propagation,
/// then materialize / recovery / export / audit observation.
pub fn process_journal_bytes(data: &[u8]) -> Result<WorkflowStats, String> {
    if data.len() > 400_000 {
        return Err("input too large".into());
    }
    let frame = decode::decode(data)?;
    let _issues = validate::validate_frame(data, false)?;
    let session = build_session(&frame);
    export_views::establish_from_session(&session);

    let rebuild_rounds = session
        .seal_rounds
        .max(session.checkpoint_marks)
        .max(1)
        .saturating_add(session.generation_advances.min(3));
    let insert_burst = session
        .append_marks
        .saturating_mul(2)
        .saturating_add(session.compact_ops)
        .max(1);

    for round in 0..rebuild_rounds {
        export_views::propagate_rebuild_round(round, insert_burst);
        let _ = session.payload_bytes.wrapping_mul(round.wrapping_add(1));
    }

    export_views::observe_materialize_stage();
    export_views::observe_recovery_stage();
    export_views::observe_export_stage();
    export_views::observe_audit_stage();
    export_views::clear();

    Ok(WorkflowStats {
        records: frame.section_count(),
        append_marks: session.append_marks,
        seal_rounds: session.seal_rounds,
        payload_bytes: session.payload_bytes,
        digest: frame_digest(&frame),
    })
}
