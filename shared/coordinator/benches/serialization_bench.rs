use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::Space;
use bytemuck::{Pod, Zeroable};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psyche_coordinator::{Client, Coordinator, CoordinatorConfig, RunState, MAX_MODEL_SIZE};
use psyche_core::{FixedString, NodeIdentity};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use ts_rs::TS;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Zeroable, TS,
)]
#[repr(C)]
struct MockClientId([u8; 32]);

unsafe impl Pod for MockClientId {}

impl AsRef<[u8]> for MockClientId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Display for MockClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mock-client")
    }
}

impl BorshSerialize for MockClientId {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.0)
    }
}

impl BorshDeserialize for MockClientId {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        if buf.len() < 32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unexpected length",
            ));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&buf[..32]);
        *buf = &buf[32..];
        Ok(MockClientId(bytes))
    }

    fn deserialize_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut bytes = [0u8; 32];
        reader.read_exact(&mut bytes)?;
        Ok(MockClientId(bytes))
    }
}

// AnchorSerialize/Deserialize have blanket impls for Borsh types in recent versions.
// If they don't, we'd need them, but the conflict error suggests they do.

impl Space for MockClientId {
    const INIT_SPACE: usize = 32;
}

impl NodeIdentity for MockClientId {
    fn get_p2p_public_key(&self) -> &[u8; 32] {
        &self.0
    }
}

fn create_mock_coordinator() -> Coordinator<MockClientId> {
    let mut coordinator = Coordinator {
        run_id: FixedString::from_str_truncated("benchmark_run"),
        run_state: RunState::RoundTrain as u8,
        model: [0u8; MAX_MODEL_SIZE],
        config: CoordinatorConfig {
            warmup_time: 100,
            cooldown_time: 100,
            max_round_train_time: 100,
            round_witness_time: 100,
            global_batch_size_warmup_tokens: 100,
            epoch_time: 100,
            total_steps: 1000,
            init_min_clients: 10,
            min_clients: 10,
            witness_nodes: 5,
            global_batch_size_start: 1,
            global_batch_size_end: 1,
            verification_percent: 10,
            waiting_for_members_extra_time: 10,
        },
        progress: Default::default(),
        epoch_state: Default::default(),
        run_state_start_unix_timestamp: 123456789,
        pending_pause: Default::default(),
    };

    coordinator
        .set_model(psyche_coordinator::model::Model::LLM(
            psyche_coordinator::model::LLM::dummy(),
        ))
        .unwrap();

    // Fill with some dummy clients to simulate 50KB payload
    // Coordinator has fixed size arrays e.g. clients: FixedVec<Client<T>, 256>
    // So the valid content vs empty content matters for TOML (text skips defaults usually? no, FixedVec serializes all items usually unless skipped)
    // Actually FixedVec implementation in core usually serializes as Vec.

    // Let's populate 50 clients
    for i in 0..50 {
        let mut id = [0u8; 32];
        id[0] = i as u8;
        let _ = coordinator
            .epoch_state
            .clients
            .push(Client::new(MockClientId(id)));
    }

    coordinator
}

fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    let coordinator = create_mock_coordinator();

    group.bench_function("toml_serialize", |b| {
        b.iter(|| toml::to_string(black_box(&coordinator)).unwrap())
    });

    group.bench_function("bincode_serialize", |b| {
        b.iter(|| bincode::serialize(black_box(&coordinator)).unwrap())
    });

    group.finish();
}

criterion_group!(benches, bench_serialization);
criterion_main!(benches);
