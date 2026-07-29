#![no_main]

use libfuzzer_sys::fuzz_target;
use relayring::journal_workflow;

fuzz_target!(|input: &[u8]| {
    if input.len() > 400_000 {
        return;
    }
    let _ = journal_workflow::process_journal_bytes(input);
});
