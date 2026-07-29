    # RelayRing

    Lock-free append-only telemetry journal for gateways and observability agents.

    Informed by Kafka append logs, ring buffers, and OpenTelemetry batch exporters.

    ## Modules

    - ring buffer append paths
- journal offset indexing
- telemetry export batches

    Top-level layout: `ring/`, `journal/`, `export/`, `gateway/`, `wire/`, `analyze/`, `config/`, `util/`.

    ## Wire format

    Magic `RLRG` with extension `.rlrg` — see `docs/FORMAT.md`.

    ## Profiles

    Domain-specific config profiles (max 6): `hot_ring`, `cold_archive`, `otlp_batch`, `agent_edge`, `compact_lazy`, `fsync_strict`.

    ## Fuzz harnesses

    `journal_fuzzer`, `stream_fuzzer`, `state_fuzzer`, `recovery_fuzzer` — each feeds
    `journal_workflow::process_journal_bytes` (decode → derived views → seal/compact
    rebuild rounds → late export/audit/materialize/recovery observers).

    ## CLI

    ```bash
    cargo run --release -- inspect examples/sample.rlrg
    cargo run --release -- validate --strict examples/sample.rlrg
    cargo run --release -- report examples/sample.rlrg
    ```

    ## Build

    ```bash
    ./scripts/build.sh
    ```
