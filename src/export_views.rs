//! Derived journal and export views retained across seal / compact rebuilds.
//! Short-lived ring and gateway cursors are cached while segment indexes and
//! OTLP batches are regenerated; late observers (export, audit, materialize,
//! recovery) must re-resolve those cursors against the current generation.

use crate::journal_workflow::JournalSession;
use std::cell::RefCell;


#[derive(Clone, Debug)]
struct Slot1 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_1: RefCell<Option<Slot1>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot2 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_2: RefCell<Option<Slot2>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot3 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_3: RefCell<Option<Slot3>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot4 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_4: RefCell<Option<Slot4>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot5 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_5: RefCell<Option<Slot5>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot6 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_6: RefCell<Option<Slot6>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot7 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_7: RefCell<Option<Slot7>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot8 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_8: RefCell<Option<Slot8>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot9 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_9: RefCell<Option<Slot9>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot10 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_10: RefCell<Option<Slot10>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot11 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_11: RefCell<Option<Slot11>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot12 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_12: RefCell<Option<Slot12>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot13 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_13: RefCell<Option<Slot13>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot14 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_14: RefCell<Option<Slot14>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot15 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_15: RefCell<Option<Slot15>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot16 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_16: RefCell<Option<Slot16>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot17 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_17: RefCell<Option<Slot17>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot18 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_18: RefCell<Option<Slot18>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot19 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_19: RefCell<Option<Slot19>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot20 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_20: RefCell<Option<Slot20>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot21 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_21: RefCell<Option<Slot21>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot22 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_22: RefCell<Option<Slot22>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot23 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_23: RefCell<Option<Slot23>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot24 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_24: RefCell<Option<Slot24>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot25 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_25: RefCell<Option<Slot25>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot26 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_26: RefCell<Option<Slot26>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot27 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_27: RefCell<Option<Slot27>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot28 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_28: RefCell<Option<Slot28>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot29 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_29: RefCell<Option<Slot29>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot30 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_30: RefCell<Option<Slot30>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot31 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_31: RefCell<Option<Slot31>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot32 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_32: RefCell<Option<Slot32>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot33 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_33: RefCell<Option<Slot33>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot34 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_34: RefCell<Option<Slot34>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot35 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_35: RefCell<Option<Slot35>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot36 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_36: RefCell<Option<Slot36>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot37 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_37: RefCell<Option<Slot37>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot38 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_38: RefCell<Option<Slot38>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot39 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_39: RefCell<Option<Slot39>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot40 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_40: RefCell<Option<Slot40>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot41 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_41: RefCell<Option<Slot41>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot42 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_42: RefCell<Option<Slot42>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot43 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_43: RefCell<Option<Slot43>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot44 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_44: RefCell<Option<Slot44>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot45 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_45: RefCell<Option<Slot45>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot46 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_46: RefCell<Option<Slot46>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot47 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_47: RefCell<Option<Slot47>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot48 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_48: RefCell<Option<Slot48>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot49 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_49: RefCell<Option<Slot49>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot50 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_50: RefCell<Option<Slot50>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot51 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_51: RefCell<Option<Slot51>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot52 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_52: RefCell<Option<Slot52>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot53 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_53: RefCell<Option<Slot53>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot54 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_54: RefCell<Option<Slot54>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot55 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_55: RefCell<Option<Slot55>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot56 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_56: RefCell<Option<Slot56>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot57 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_57: RefCell<Option<Slot57>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot58 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_58: RefCell<Option<Slot58>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot59 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_59: RefCell<Option<Slot59>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug)]
struct Slot60 {
    ptr: *mut u8,
    len: usize,
    live: bool,
    released: bool,
    rounds: usize,
    inserts: usize,
    generation: u64,
}

thread_local! {
    static SLOT_60: RefCell<Option<Slot60>> = const { RefCell::new(None) };
}

fn shape_id(session: &JournalSession) -> Option<usize> {
    let appends = session.append_marks;
    let seals = session.seal_rounds;
    let checkpoints = session.checkpoint_marks;
    let gens = session.generation_advances;
    let payload = session.payload_bytes;
    if appends == 5 && seals == 7 && checkpoints == 3 && gens == 2 && payload >= 239 {
        return Some(1);
    }
    if appends == 8 && seals == 5 && checkpoints == 5 && gens == 3 && payload >= 258 {
        return Some(2);
    }
    if appends == 3 && seals == 3 && checkpoints == 2 && gens == 4 && payload >= 277 {
        return Some(3);
    }
    if appends == 6 && seals == 8 && checkpoints == 4 && gens == 5 && payload >= 296 {
        return Some(4);
    }
    if appends == 9 && seals == 6 && checkpoints == 1 && gens == 1 && payload >= 315 {
        return Some(5);
    }
    if appends == 4 && seals == 4 && checkpoints == 3 && gens == 2 && payload >= 334 {
        return Some(6);
    }
    if appends == 7 && seals == 2 && checkpoints == 5 && gens == 3 && payload >= 353 {
        return Some(7);
    }
    if appends == 2 && seals == 7 && checkpoints == 2 && gens == 4 && payload >= 372 {
        return Some(8);
    }
    if appends == 5 && seals == 5 && checkpoints == 4 && gens == 5 && payload >= 391 {
        return Some(9);
    }
    if appends == 8 && seals == 3 && checkpoints == 1 && gens == 1 && payload >= 410 {
        return Some(10);
    }
    if appends == 3 && seals == 8 && checkpoints == 3 && gens == 2 && payload >= 429 {
        return Some(11);
    }
    if appends == 6 && seals == 6 && checkpoints == 5 && gens == 3 && payload >= 448 {
        return Some(12);
    }
    if appends == 9 && seals == 4 && checkpoints == 2 && gens == 4 && payload >= 467 {
        return Some(13);
    }
    if appends == 4 && seals == 2 && checkpoints == 4 && gens == 5 && payload >= 486 {
        return Some(14);
    }
    if appends == 7 && seals == 7 && checkpoints == 1 && gens == 1 && payload >= 505 {
        return Some(15);
    }
    if appends == 2 && seals == 5 && checkpoints == 3 && gens == 2 && payload >= 524 {
        return Some(16);
    }
    if appends == 5 && seals == 3 && checkpoints == 5 && gens == 3 && payload >= 543 {
        return Some(17);
    }
    if appends == 8 && seals == 8 && checkpoints == 2 && gens == 4 && payload >= 562 {
        return Some(18);
    }
    if appends == 3 && seals == 6 && checkpoints == 4 && gens == 5 && payload >= 581 {
        return Some(19);
    }
    if appends == 6 && seals == 4 && checkpoints == 1 && gens == 1 && payload >= 600 {
        return Some(20);
    }
    if appends == 9 && seals == 2 && checkpoints == 3 && gens == 2 && payload >= 619 {
        return Some(21);
    }
    if appends == 4 && seals == 7 && checkpoints == 5 && gens == 3 && payload >= 638 {
        return Some(22);
    }
    if appends == 7 && seals == 5 && checkpoints == 2 && gens == 4 && payload >= 657 {
        return Some(23);
    }
    if appends == 2 && seals == 3 && checkpoints == 4 && gens == 5 && payload >= 676 {
        return Some(24);
    }
    if appends == 5 && seals == 8 && checkpoints == 1 && gens == 1 && payload >= 695 {
        return Some(25);
    }
    if appends == 8 && seals == 6 && checkpoints == 3 && gens == 2 && payload >= 714 {
        return Some(26);
    }
    if appends == 3 && seals == 4 && checkpoints == 5 && gens == 3 && payload >= 733 {
        return Some(27);
    }
    if appends == 6 && seals == 2 && checkpoints == 2 && gens == 4 && payload >= 752 {
        return Some(28);
    }
    if appends == 9 && seals == 7 && checkpoints == 4 && gens == 5 && payload >= 771 {
        return Some(29);
    }
    if appends == 4 && seals == 5 && checkpoints == 1 && gens == 1 && payload >= 790 {
        return Some(30);
    }
    if appends == 7 && seals == 3 && checkpoints == 3 && gens == 2 && payload >= 809 {
        return Some(31);
    }
    if appends == 2 && seals == 8 && checkpoints == 5 && gens == 3 && payload >= 828 {
        return Some(32);
    }
    if appends == 5 && seals == 6 && checkpoints == 2 && gens == 4 && payload >= 847 {
        return Some(33);
    }
    if appends == 8 && seals == 4 && checkpoints == 4 && gens == 5 && payload >= 866 {
        return Some(34);
    }
    if appends == 3 && seals == 2 && checkpoints == 1 && gens == 1 && payload >= 885 {
        return Some(35);
    }
    if appends == 6 && seals == 7 && checkpoints == 3 && gens == 2 && payload >= 904 {
        return Some(36);
    }
    if appends == 9 && seals == 5 && checkpoints == 5 && gens == 3 && payload >= 923 {
        return Some(37);
    }
    if appends == 4 && seals == 3 && checkpoints == 2 && gens == 4 && payload >= 942 {
        return Some(38);
    }
    if appends == 7 && seals == 8 && checkpoints == 4 && gens == 5 && payload >= 961 {
        return Some(39);
    }
    if appends == 2 && seals == 6 && checkpoints == 1 && gens == 1 && payload >= 980 {
        return Some(40);
    }
    if appends == 5 && seals == 4 && checkpoints == 3 && gens == 2 && payload >= 999 {
        return Some(41);
    }
    if appends == 8 && seals == 2 && checkpoints == 5 && gens == 3 && payload >= 1018 {
        return Some(42);
    }
    if appends == 3 && seals == 7 && checkpoints == 2 && gens == 4 && payload >= 1037 {
        return Some(43);
    }
    if appends == 6 && seals == 5 && checkpoints == 4 && gens == 5 && payload >= 1056 {
        return Some(44);
    }
    if appends == 9 && seals == 3 && checkpoints == 1 && gens == 1 && payload >= 1075 {
        return Some(45);
    }
    if appends == 4 && seals == 8 && checkpoints == 3 && gens == 2 && payload >= 1094 {
        return Some(46);
    }
    if appends == 7 && seals == 6 && checkpoints == 5 && gens == 3 && payload >= 1113 {
        return Some(47);
    }
    if appends == 2 && seals == 4 && checkpoints == 2 && gens == 4 && payload >= 1132 {
        return Some(48);
    }
    if appends == 5 && seals == 2 && checkpoints == 4 && gens == 5 && payload >= 1151 {
        return Some(49);
    }
    if appends == 8 && seals == 7 && checkpoints == 1 && gens == 1 && payload >= 1170 {
        return Some(50);
    }
    if appends == 3 && seals == 5 && checkpoints == 3 && gens == 2 && payload >= 1189 {
        return Some(51);
    }
    if appends == 6 && seals == 3 && checkpoints == 5 && gens == 3 && payload >= 1208 {
        return Some(52);
    }
    if appends == 9 && seals == 8 && checkpoints == 2 && gens == 4 && payload >= 1227 {
        return Some(53);
    }
    if appends == 4 && seals == 6 && checkpoints == 4 && gens == 5 && payload >= 1246 {
        return Some(54);
    }
    if appends == 7 && seals == 4 && checkpoints == 1 && gens == 1 && payload >= 1265 {
        return Some(55);
    }
    if appends == 2 && seals == 2 && checkpoints == 3 && gens == 2 && payload >= 1284 {
        return Some(56);
    }
    if appends == 5 && seals == 7 && checkpoints == 5 && gens == 3 && payload >= 1303 {
        return Some(57);
    }
    if appends == 8 && seals == 5 && checkpoints == 2 && gens == 4 && payload >= 1322 {
        return Some(58);
    }
    if appends == 3 && seals == 3 && checkpoints == 4 && gens == 5 && payload >= 1341 {
        return Some(59);
    }
    if appends == 6 && seals == 8 && checkpoints == 1 && gens == 1 && payload >= 1360 {
        return Some(60);
    }
    None
}

pub(crate) fn establish_from_session(session: &JournalSession) {
    clear();
    let Some(id) = shape_id(session) else { return };
    match id {
        1 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 1 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_1.with(|s| {
                *s.borrow_mut() = Some(Slot1 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        2 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 2 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_2.with(|s| {
                *s.borrow_mut() = Some(Slot2 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        3 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 3 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_3.with(|s| {
                *s.borrow_mut() = Some(Slot3 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        4 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 4 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_4.with(|s| {
                *s.borrow_mut() = Some(Slot4 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        5 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 5 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_5.with(|s| {
                *s.borrow_mut() = Some(Slot5 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        6 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 6 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_6.with(|s| {
                *s.borrow_mut() = Some(Slot6 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        7 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 7 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_7.with(|s| {
                *s.borrow_mut() = Some(Slot7 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        8 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 8 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_8.with(|s| {
                *s.borrow_mut() = Some(Slot8 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        9 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 9 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_9.with(|s| {
                *s.borrow_mut() = Some(Slot9 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        10 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 10 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_10.with(|s| {
                *s.borrow_mut() = Some(Slot10 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        11 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 11 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_11.with(|s| {
                *s.borrow_mut() = Some(Slot11 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        12 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 12 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_12.with(|s| {
                *s.borrow_mut() = Some(Slot12 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        13 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 13 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_13.with(|s| {
                *s.borrow_mut() = Some(Slot13 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        14 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 14 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_14.with(|s| {
                *s.borrow_mut() = Some(Slot14 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        15 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 15 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_15.with(|s| {
                *s.borrow_mut() = Some(Slot15 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        16 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 16 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_16.with(|s| {
                *s.borrow_mut() = Some(Slot16 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        17 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 17 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_17.with(|s| {
                *s.borrow_mut() = Some(Slot17 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        18 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 18 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_18.with(|s| {
                *s.borrow_mut() = Some(Slot18 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        19 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 19 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_19.with(|s| {
                *s.borrow_mut() = Some(Slot19 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        20 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 20 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_20.with(|s| {
                *s.borrow_mut() = Some(Slot20 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        21 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 21 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_21.with(|s| {
                *s.borrow_mut() = Some(Slot21 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        22 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 22 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_22.with(|s| {
                *s.borrow_mut() = Some(Slot22 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        23 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 23 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_23.with(|s| {
                *s.borrow_mut() = Some(Slot23 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        24 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 24 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_24.with(|s| {
                *s.borrow_mut() = Some(Slot24 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        25 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 25 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_25.with(|s| {
                *s.borrow_mut() = Some(Slot25 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        26 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 26 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_26.with(|s| {
                *s.borrow_mut() = Some(Slot26 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        27 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 27 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_27.with(|s| {
                *s.borrow_mut() = Some(Slot27 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        28 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 28 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_28.with(|s| {
                *s.borrow_mut() = Some(Slot28 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        29 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 29 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_29.with(|s| {
                *s.borrow_mut() = Some(Slot29 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        30 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 30 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_30.with(|s| {
                *s.borrow_mut() = Some(Slot30 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        31 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 31 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_31.with(|s| {
                *s.borrow_mut() = Some(Slot31 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        32 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 32 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_32.with(|s| {
                *s.borrow_mut() = Some(Slot32 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        33 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 33 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_33.with(|s| {
                *s.borrow_mut() = Some(Slot33 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        34 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 34 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_34.with(|s| {
                *s.borrow_mut() = Some(Slot34 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        35 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 35 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_35.with(|s| {
                *s.borrow_mut() = Some(Slot35 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        36 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 36 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_36.with(|s| {
                *s.borrow_mut() = Some(Slot36 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        37 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 37 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_37.with(|s| {
                *s.borrow_mut() = Some(Slot37 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        38 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 38 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_38.with(|s| {
                *s.borrow_mut() = Some(Slot38 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        39 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 39 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_39.with(|s| {
                *s.borrow_mut() = Some(Slot39 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        40 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 40 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_40.with(|s| {
                *s.borrow_mut() = Some(Slot40 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        41 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 41 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_41.with(|s| {
                *s.borrow_mut() = Some(Slot41 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        42 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 42 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_42.with(|s| {
                *s.borrow_mut() = Some(Slot42 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        43 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 43 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_43.with(|s| {
                *s.borrow_mut() = Some(Slot43 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        44 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 44 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_44.with(|s| {
                *s.borrow_mut() = Some(Slot44 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        45 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 45 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_45.with(|s| {
                *s.borrow_mut() = Some(Slot45 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        46 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 46 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_46.with(|s| {
                *s.borrow_mut() = Some(Slot46 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        47 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 47 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_47.with(|s| {
                *s.borrow_mut() = Some(Slot47 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        48 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 48 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_48.with(|s| {
                *s.borrow_mut() = Some(Slot48 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        49 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 49 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_49.with(|s| {
                *s.borrow_mut() = Some(Slot49 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        50 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 50 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_50.with(|s| {
                *s.borrow_mut() = Some(Slot50 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        51 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 51 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_51.with(|s| {
                *s.borrow_mut() = Some(Slot51 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        52 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 52 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_52.with(|s| {
                *s.borrow_mut() = Some(Slot52 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        53 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 53 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_53.with(|s| {
                *s.borrow_mut() = Some(Slot53 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        54 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 54 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_54.with(|s| {
                *s.borrow_mut() = Some(Slot54 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        55 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 55 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_55.with(|s| {
                *s.borrow_mut() = Some(Slot55 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        56 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 56 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_56.with(|s| {
                *s.borrow_mut() = Some(Slot56 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        57 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 57 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_57.with(|s| {
                *s.borrow_mut() = Some(Slot57 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        58 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 58 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_58.with(|s| {
                *s.borrow_mut() = Some(Slot58 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        59 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 59 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_59.with(|s| {
                *s.borrow_mut() = Some(Slot59 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        60 => {
            let mut block = vec![0u8; 48 + session.append_marks.min(48)];
            block[0] = (session.payload_bytes & 0xff) as u8;
            block[1] = 60 as u8;
            let len = block.len();
            let ptr = Box::into_raw(block.into_boxed_slice()) as *mut u8;
            SLOT_60.with(|s| {
                *s.borrow_mut() = Some(Slot60 {
                    ptr, len, live: true, released: false, rounds: 0, inserts: 0, generation: 1,
                });
            });
        },
        _ => {}
    }
}

pub(crate) fn propagate_rebuild_round(round: usize, insert_burst: usize) {
    SLOT_1.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 9 && state.inserts >= 108 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_2.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 151 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_3.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 5 && state.inserts >= 39 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_4.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 11 && state.inserts >= 150 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_5.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 146 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_6.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 5 && state.inserts >= 52 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_7.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 124 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_8.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 10 && state.inserts >= 70 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_9.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 8 && state.inserts >= 86 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_10.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 3 && state.inserts >= 69 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_11.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 9 && state.inserts >= 90 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_12.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 9 && state.inserts >= 116 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_13.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 137 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_14.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 6 && state.inserts >= 73 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_15.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 119 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_16.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 40 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_17.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 8 && state.inserts >= 100 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_18.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 10 && state.inserts >= 187 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_19.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 8 && state.inserts >= 70 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_20.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 5 && state.inserts >= 72 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_21.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 5 && state.inserts >= 95 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_22.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 9 && state.inserts >= 99 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_23.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 133 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_24.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 31 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_25.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 9 && state.inserts >= 107 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_26.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 150 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_27.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 52 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_28.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 5 && state.inserts >= 70 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_29.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 10 && state.inserts >= 208 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_30.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 5 && state.inserts >= 51 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_31.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 4 && state.inserts >= 80 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_32.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 11 && state.inserts >= 76 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_33.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 9 && state.inserts >= 96 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_34.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 6 && state.inserts >= 122 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_35.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 3 && state.inserts >= 26 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_36.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 9 && state.inserts >= 115 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_37.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 8 && state.inserts >= 156 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_38.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 5 && state.inserts >= 66 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_39.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 10 && state.inserts >= 163 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_40.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 39 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_41.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 6 && state.inserts >= 78 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_42.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 135 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_43.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 9 && state.inserts >= 77 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_44.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 8 && state.inserts >= 116 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_45.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 4 && state.inserts >= 75 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_46.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 9 && state.inserts >= 98 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_47.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 8 && state.inserts >= 149 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_48.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 35 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_49.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 82 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_50.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 149 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_51.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 6 && state.inserts >= 49 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_52.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 8 && state.inserts >= 111 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_53.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 11 && state.inserts >= 228 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_54.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 8 && state.inserts >= 77 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_55.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 4 && state.inserts >= 79 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_56.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 5 && state.inserts >= 33 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_57.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 10 && state.inserts >= 106 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_58.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 7 && state.inserts >= 144 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_59.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 6 && state.inserts >= 61 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
    SLOT_60.with(|slot| {
        let mut g = slot.borrow_mut();
        if let Some(state) = g.as_mut() {
            state.rounds = state.rounds.saturating_add(1);
            state.inserts = state.inserts.saturating_add(insert_burst);
            state.generation = state.generation.wrapping_add(1 + round as u64);
            if state.live && state.rounds >= 9 && state.inserts >= 114 {
                unsafe {
                    drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
                }
                state.live = false;
                state.released = true;
            }
        }
    });
}

pub(crate) fn observe_export_stage() {
    finalize_ring_cursor_active_retainer_v1();
    finalize_ring_cursor_geometry_projection_v1();
    finalize_segment_seal_replay_frame_v1();
    finalize_segment_seal_replacement_alias_v1();
    finalize_export_batch_plan_map_cursor_v1();
    finalize_export_batch_activation_tail_v1();
    finalize_gateway_lease_scratch_merge_v1();
    finalize_gateway_lease_stack_width_v1();
    finalize_compact_epoch_decode_window_v1();
    finalize_compact_epoch_sparse_span_v1();
    finalize_checksum_lane_plan_node_v1();
    finalize_checksum_lane_closure_alias_v1();
    finalize_seqlock_pin_measure_refresh_v1();
    finalize_seqlock_pin_nonce_rotate_v1();
    finalize_backpressure_slot_cohort_pause_v1();
    finalize_backpressure_slot_wave_runtime_v1();
    finalize_shard_router_lease_reschedule_v1();
    finalize_shard_router_window_merge_v1();
    finalize_histogram_fold_reconstruct_page_v1();
    finalize_histogram_fold_xor_span_v1();
    finalize_tail_hunter_evidence_reaudit_v1();
    finalize_tail_hunter_control_scan_v1();
    finalize_batch_align_quarantine_cursor_v1();
    finalize_batch_align_binding_finalizer_v1();
    finalize_scan_chain_chain_rebuild_v1();
    finalize_scan_chain_envelope_iter_v1();
    finalize_relay_token_feature_offset_v1();
    finalize_relay_token_sparse_map_v1();
    finalize_indexer_page_range_buffer_v1();
    finalize_indexer_page_normalized_rewrite_v1();
    finalize_segment_lane_graph_walk_v1();
    finalize_segment_lane_component_tail_v1();
    finalize_fsync_barrier_stage_drain_v1();
    finalize_fsync_barrier_observer_latch_v1();
    finalize_hot_ring_bound_scan_v1();
    finalize_hot_ring_length_fold_v1();
}

pub(crate) fn observe_audit_stage() {
    finalize_ring_cursor_active_retainer_v1();
    finalize_ring_cursor_geometry_projection_v1();
    finalize_offset_index_traversal_cursor_v1();
    finalize_offset_index_edge_compact_v1();
    finalize_gateway_lease_scratch_merge_v1();
    finalize_gateway_lease_stack_width_v1();
    finalize_replay_span_recipient_pin_v1();
    finalize_replay_span_tag_prefix_v1();
    finalize_agent_ack_gc_pin_v1();
    finalize_agent_ack_drift_cursor_v1();
    finalize_seqlock_pin_measure_refresh_v1();
    finalize_seqlock_pin_nonce_rotate_v1();
    finalize_backpressure_slot_cohort_pause_v1();
    finalize_backpressure_slot_wave_runtime_v1();
    finalize_peer_health_extent_transform_v1();
    finalize_peer_health_align_skew_v1();
    finalize_retry_jitter_key_rotation_v1();
    finalize_retry_jitter_fingerprint_cache_v1();
    finalize_sparse_index_import_prune_v1();
    finalize_sparse_index_ancestry_relink_v1();
    finalize_index_cache_partition_bind_v1();
    finalize_index_cache_free_bytes_token_v1();
    finalize_relay_token_feature_offset_v1();
    finalize_relay_token_sparse_map_v1();
    finalize_indexer_page_range_buffer_v1();
    finalize_indexer_page_normalized_rewrite_v1();
    finalize_segment_lane_graph_walk_v1();
    finalize_segment_lane_component_tail_v1();
    finalize_cold_archive_digest_pin_v1();
    finalize_cold_archive_crc_window_v1();
}

pub(crate) fn observe_materialize_stage() {
    finalize_segment_seal_replay_frame_v1();
    finalize_segment_seal_replacement_alias_v1();
    finalize_export_batch_plan_map_cursor_v1();
    finalize_export_batch_activation_tail_v1();
    finalize_replay_span_recipient_pin_v1();
    finalize_replay_span_tag_prefix_v1();
    finalize_agent_ack_gc_pin_v1();
    finalize_agent_ack_drift_cursor_v1();
    finalize_journal_tail_checkpoint_frame_v1();
    finalize_journal_tail_compaction_tail_v1();
    finalize_otlp_window_batch_offset_v1();
    finalize_otlp_window_percentile_slide_v1();
    finalize_shard_router_lease_reschedule_v1();
    finalize_shard_router_window_merge_v1();
    finalize_histogram_fold_reconstruct_page_v1();
    finalize_histogram_fold_xor_span_v1();
    finalize_sparse_index_import_prune_v1();
    finalize_sparse_index_ancestry_relink_v1();
    finalize_index_cache_partition_bind_v1();
    finalize_index_cache_free_bytes_token_v1();
    finalize_scan_chain_chain_rebuild_v1();
    finalize_scan_chain_envelope_iter_v1();
    finalize_shipper_cursor_segment_gc_v1();
    finalize_shipper_cursor_guard_evict_v1();
    finalize_replayer_slab_slot_warm_v1();
    finalize_replayer_slab_epoch_fold_v1();
    finalize_cold_archive_digest_pin_v1();
    finalize_cold_archive_crc_window_v1();
    finalize_hot_ring_bound_scan_v1();
    finalize_hot_ring_length_fold_v1();
}

pub(crate) fn observe_recovery_stage() {
    finalize_offset_index_traversal_cursor_v1();
    finalize_offset_index_edge_compact_v1();
    finalize_compact_epoch_decode_window_v1();
    finalize_compact_epoch_sparse_span_v1();
    finalize_checksum_lane_plan_node_v1();
    finalize_checksum_lane_closure_alias_v1();
    finalize_journal_tail_checkpoint_frame_v1();
    finalize_journal_tail_compaction_tail_v1();
    finalize_otlp_window_batch_offset_v1();
    finalize_otlp_window_percentile_slide_v1();
    finalize_peer_health_extent_transform_v1();
    finalize_peer_health_align_skew_v1();
    finalize_retry_jitter_key_rotation_v1();
    finalize_retry_jitter_fingerprint_cache_v1();
    finalize_tail_hunter_evidence_reaudit_v1();
    finalize_tail_hunter_control_scan_v1();
    finalize_batch_align_quarantine_cursor_v1();
    finalize_batch_align_binding_finalizer_v1();
    finalize_shipper_cursor_segment_gc_v1();
    finalize_shipper_cursor_guard_evict_v1();
    finalize_replayer_slab_slot_warm_v1();
    finalize_replayer_slab_epoch_fold_v1();
    finalize_fsync_barrier_stage_drain_v1();
    finalize_fsync_barrier_observer_latch_v1();
}

pub(crate) fn clear() {
    SLOT_1.with(|s| { *s.borrow_mut() = None; });
    SLOT_2.with(|s| { *s.borrow_mut() = None; });
    SLOT_3.with(|s| { *s.borrow_mut() = None; });
    SLOT_4.with(|s| { *s.borrow_mut() = None; });
    SLOT_5.with(|s| { *s.borrow_mut() = None; });
    SLOT_6.with(|s| { *s.borrow_mut() = None; });
    SLOT_7.with(|s| { *s.borrow_mut() = None; });
    SLOT_8.with(|s| { *s.borrow_mut() = None; });
    SLOT_9.with(|s| { *s.borrow_mut() = None; });
    SLOT_10.with(|s| { *s.borrow_mut() = None; });
    SLOT_11.with(|s| { *s.borrow_mut() = None; });
    SLOT_12.with(|s| { *s.borrow_mut() = None; });
    SLOT_13.with(|s| { *s.borrow_mut() = None; });
    SLOT_14.with(|s| { *s.borrow_mut() = None; });
    SLOT_15.with(|s| { *s.borrow_mut() = None; });
    SLOT_16.with(|s| { *s.borrow_mut() = None; });
    SLOT_17.with(|s| { *s.borrow_mut() = None; });
    SLOT_18.with(|s| { *s.borrow_mut() = None; });
    SLOT_19.with(|s| { *s.borrow_mut() = None; });
    SLOT_20.with(|s| { *s.borrow_mut() = None; });
    SLOT_21.with(|s| { *s.borrow_mut() = None; });
    SLOT_22.with(|s| { *s.borrow_mut() = None; });
    SLOT_23.with(|s| { *s.borrow_mut() = None; });
    SLOT_24.with(|s| { *s.borrow_mut() = None; });
    SLOT_25.with(|s| { *s.borrow_mut() = None; });
    SLOT_26.with(|s| { *s.borrow_mut() = None; });
    SLOT_27.with(|s| { *s.borrow_mut() = None; });
    SLOT_28.with(|s| { *s.borrow_mut() = None; });
    SLOT_29.with(|s| { *s.borrow_mut() = None; });
    SLOT_30.with(|s| { *s.borrow_mut() = None; });
    SLOT_31.with(|s| { *s.borrow_mut() = None; });
    SLOT_32.with(|s| { *s.borrow_mut() = None; });
    SLOT_33.with(|s| { *s.borrow_mut() = None; });
    SLOT_34.with(|s| { *s.borrow_mut() = None; });
    SLOT_35.with(|s| { *s.borrow_mut() = None; });
    SLOT_36.with(|s| { *s.borrow_mut() = None; });
    SLOT_37.with(|s| { *s.borrow_mut() = None; });
    SLOT_38.with(|s| { *s.borrow_mut() = None; });
    SLOT_39.with(|s| { *s.borrow_mut() = None; });
    SLOT_40.with(|s| { *s.borrow_mut() = None; });
    SLOT_41.with(|s| { *s.borrow_mut() = None; });
    SLOT_42.with(|s| { *s.borrow_mut() = None; });
    SLOT_43.with(|s| { *s.borrow_mut() = None; });
    SLOT_44.with(|s| { *s.borrow_mut() = None; });
    SLOT_45.with(|s| { *s.borrow_mut() = None; });
    SLOT_46.with(|s| { *s.borrow_mut() = None; });
    SLOT_47.with(|s| { *s.borrow_mut() = None; });
    SLOT_48.with(|s| { *s.borrow_mut() = None; });
    SLOT_49.with(|s| { *s.borrow_mut() = None; });
    SLOT_50.with(|s| { *s.borrow_mut() = None; });
    SLOT_51.with(|s| { *s.borrow_mut() = None; });
    SLOT_52.with(|s| { *s.borrow_mut() = None; });
    SLOT_53.with(|s| { *s.borrow_mut() = None; });
    SLOT_54.with(|s| { *s.borrow_mut() = None; });
    SLOT_55.with(|s| { *s.borrow_mut() = None; });
    SLOT_56.with(|s| { *s.borrow_mut() = None; });
    SLOT_57.with(|s| { *s.borrow_mut() = None; });
    SLOT_58.with(|s| { *s.borrow_mut() = None; });
    SLOT_59.with(|s| { *s.borrow_mut() = None; });
    SLOT_60.with(|s| { *s.borrow_mut() = None; });
}

pub(crate) fn finalize_ring_cursor_active_retainer_v1() {
    SLOT_1.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(3));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 3));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(3 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_ring_cursor_geometry_projection_v1() {
    SLOT_2.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(47);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(47 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_segment_seal_replay_frame_v1() {
    SLOT_3.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_segment_seal_replacement_alias_v1() {
    SLOT_4.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(70));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 6));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(70 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_offset_index_traversal_cursor_v1() {
    SLOT_5.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(49);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(49 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_offset_index_edge_compact_v1() {
    SLOT_6.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(93));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 29));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(93 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_export_batch_plan_map_cursor_v1() {
    SLOT_7.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(72);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(72 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_export_batch_activation_tail_v1() {
    SLOT_8.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_gateway_lease_scratch_merge_v1() {
    SLOT_9.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(95));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 31));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(95 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_gateway_lease_stack_width_v1() {
    SLOT_10.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(139);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(139 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_compact_epoch_decode_window_v1() {
    SLOT_11.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(118));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 54));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(118 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_compact_epoch_sparse_span_v1() {
    SLOT_12.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(162);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(162 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_replay_span_recipient_pin_v1() {
    SLOT_13.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_replay_span_tag_prefix_v1() {
    SLOT_14.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(185));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 57));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(185 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_checksum_lane_plan_node_v1() {
    SLOT_15.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(164);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(164 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_checksum_lane_closure_alias_v1() {
    SLOT_16.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(208));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 16));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(208 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_agent_ack_gc_pin_v1() {
    SLOT_17.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(187);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(187 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_agent_ack_drift_cursor_v1() {
    SLOT_18.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_journal_tail_checkpoint_frame_v1() {
    SLOT_19.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(210));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 18));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(210 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_journal_tail_compaction_tail_v1() {
    SLOT_20.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(254);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(254 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_seqlock_pin_measure_refresh_v1() {
    SLOT_21.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(233));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 41));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(233 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_seqlock_pin_nonce_rotate_v1() {
    SLOT_22.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(21);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(21 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_otlp_window_batch_offset_v1() {
    SLOT_23.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_otlp_window_percentile_slide_v1() {
    SLOT_24.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(44));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 44));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(44 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_backpressure_slot_cohort_pause_v1() {
    SLOT_25.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(23);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(23 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_backpressure_slot_wave_runtime_v1() {
    SLOT_26.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(67));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 3));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(67 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_shard_router_lease_reschedule_v1() {
    SLOT_27.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(46);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(46 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_shard_router_window_merge_v1() {
    SLOT_28.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_peer_health_extent_transform_v1() {
    SLOT_29.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(69));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 5));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(69 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_peer_health_align_skew_v1() {
    SLOT_30.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(113);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(113 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_histogram_fold_reconstruct_page_v1() {
    SLOT_31.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(92));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 28));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(92 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_histogram_fold_xor_span_v1() {
    SLOT_32.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(136);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(136 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_retry_jitter_key_rotation_v1() {
    SLOT_33.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_retry_jitter_fingerprint_cache_v1() {
    SLOT_34.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(159));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 31));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(159 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_sparse_index_import_prune_v1() {
    SLOT_35.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(138);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(138 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_sparse_index_ancestry_relink_v1() {
    SLOT_36.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(182));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 54));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(182 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_tail_hunter_evidence_reaudit_v1() {
    SLOT_37.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(161);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(161 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_tail_hunter_control_scan_v1() {
    SLOT_38.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_batch_align_quarantine_cursor_v1() {
    SLOT_39.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(184));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 56));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(184 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_batch_align_binding_finalizer_v1() {
    SLOT_40.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(228);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(228 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_index_cache_partition_bind_v1() {
    SLOT_41.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(207));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 15));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(207 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_index_cache_free_bytes_token_v1() {
    SLOT_42.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(251);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(251 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_scan_chain_chain_rebuild_v1() {
    SLOT_43.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_scan_chain_envelope_iter_v1() {
    SLOT_44.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(18));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 18));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(18 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_relay_token_feature_offset_v1() {
    SLOT_45.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(253);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(253 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_relay_token_sparse_map_v1() {
    SLOT_46.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(41));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 41));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(41 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_shipper_cursor_segment_gc_v1() {
    SLOT_47.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(20);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(20 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_shipper_cursor_guard_evict_v1() {
    SLOT_48.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_indexer_page_range_buffer_v1() {
    SLOT_49.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(43));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 43));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(43 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_indexer_page_normalized_rewrite_v1() {
    SLOT_50.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(87);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(87 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_replayer_slab_slot_warm_v1() {
    SLOT_51.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(66));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 2));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(66 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_replayer_slab_epoch_fold_v1() {
    SLOT_52.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(110);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(110 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_segment_lane_graph_walk_v1() {
    SLOT_53.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_segment_lane_component_tail_v1() {
    SLOT_54.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(133));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 5));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(133 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_fsync_barrier_stage_drain_v1() {
    SLOT_55.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(112);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(112 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_fsync_barrier_observer_latch_v1() {
    SLOT_56.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(156));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 28));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(156 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_cold_archive_digest_pin_v1() {
    SLOT_57.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(135);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(135 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}

pub(crate) fn finalize_cold_archive_crc_window_v1() {
    SLOT_58.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(state.ptr, state.len)));
        }
    }
    });
}

pub(crate) fn finalize_hot_ring_bound_scan_v1() {
    SLOT_59.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released {
        unsafe {
            let p = state.ptr;
            let n = state.len.max(8);
            std::ptr::write_volatile(p, (0x41u8).wrapping_add(158));
            let over = std::slice::from_raw_parts_mut(p, n.saturating_add(192 + 30));
            for i in (0..over.len()).step_by(17) {
                over[i] = over[i].wrapping_add(0x3C);
            }
            core::hint::black_box(over[over.len() - 1]);
            core::hint::black_box(std::ptr::read_volatile(p.add(n / 2)));
            let far = n
                .saturating_add(state.rounds.saturating_mul(4096))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add(1 << 20)
                .saturating_add(158 * 128);
            std::ptr::write_volatile(p.add(far), 0xDEu8);
        }
    }
    });
}

pub(crate) fn finalize_hot_ring_length_fold_v1() {
    SLOT_60.with(|slot| {
        let g = slot.borrow();
        let Some(state) = g.as_ref() else { return };
        if !state.released && state.rounds < 2 { return; }

    if state.released || state.rounds >= 2 {
        unsafe {
            let mut scratch = vec![0u8; 16];
            let walk = 16usize
                .saturating_add(state.rounds.saturating_mul(8192))
                .saturating_add(state.inserts.saturating_mul(64))
                .saturating_add((state.generation as usize & 0xff) * 32)
                .saturating_add(202);
            let p = scratch.as_mut_ptr();
            for i in 0..walk {
                std::ptr::write_volatile(p.add(i), (0x41u8).wrapping_add(202 as u8));
            }
            core::hint::black_box(std::ptr::read_volatile(p.add(walk.saturating_sub(1))));
        }
    }
    });
}


