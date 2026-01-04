use anchor_lang::{AnchorDeserialize, AnchorSerialize, InitSpace};
use bytemuck::{Pod, Zeroable};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psyche_coordinator::{Commitment, Coordinator, Witness, WitnessBloom, WitnessProof};
use psyche_core::{MerkleRoot, NodeIdentity, SmallBoolean};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use ts_rs::TS;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Zeroable,
    Pod,
    Serialize,
    Deserialize,
    InitSpace,
    TS,
    Default,
)]
#[repr(C)]
pub struct MockClientId {
    pub id: u64,
    pub p2p_key: [u8; 32],
}

impl Display for MockClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl AsRef<[u8]> for MockClientId {
    fn as_ref(&self) -> &[u8] {
        bytemuck::bytes_of(&self.id)
    }
}

impl AnchorSerialize for MockClientId {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        AnchorSerialize::serialize(&self.id, writer)?;
        AnchorSerialize::serialize(&self.p2p_key, writer)
    }
}

impl AnchorDeserialize for MockClientId {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let id = AnchorDeserialize::deserialize_reader(reader)?;
        let p2p_key = AnchorDeserialize::deserialize_reader(reader)?;
        Ok(MockClientId { id, p2p_key })
    }
}

impl NodeIdentity for MockClientId {
    fn get_p2p_public_key(&self) -> &[u8; 32] {
        &self.p2p_key
    }
}

fn bench_consensus(c: &mut Criterion) {
    let mut group = c.benchmark_group("consensus");

    // Setup inputs
    let num_witnesses = 32;
    let num_commitments = 100; // Typical number of proposals?

    let mut witnesses = Vec::new();
    let mut commitments = Vec::new();

    // Create random commitments
    for i in 0..num_commitments {
        let hash = [i as u8; 32];
        let commitment = Commitment {
            data_hash: hash,
            signature: [0u8; 64],
        };
        commitments.push(commitment);
    }

    // Create witnesses that vote for commitment 0
    for _ in 0..num_witnesses {
        let mut broadcast_bloom = WitnessBloom::default();
        broadcast_bloom.add(&commitments[99].data_hash); // Vote for last one (worst case)

        witnesses.push(Witness {
            proof: WitnessProof {
                position: 0,
                index: 0,
                witness: SmallBoolean::TRUE,
            },
            participant_bloom: WitnessBloom::default(),
            broadcast_bloom,
            broadcast_merkle: MerkleRoot::default(),
        });
    }

    group.bench_function("select_consensus", |b| {
        b.iter(|| {
            Coordinator::<MockClientId>::select_consensus_commitment_by_witnesses(
                black_box(&commitments),
                black_box(&witnesses),
                black_box(20), // Quorum
            )
        })
    });

    group.finish();
}

criterion_group!(benches, bench_consensus);
criterion_main!(benches);
