use ark_ff::UniformRand;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, OptimizationGoal, SynthesisMode,
};
use decaf377::Fq;
use penumbra_asset::asset;
use penumbra_dex::{
    swap::SwapPlaintext,
    swap_claim::{SwapClaimCircuit, SwapClaimProof, SwapClaimProofPrivate, SwapClaimProofPublic},
    BatchSwapOutputData, TradingPair,
};
use penumbra_fee::Fee;
use penumbra_keys::keys::{Bip44Path, SeedPhrase, SpendKey};
use penumbra_num::Amount;
use penumbra_proof_params::{DummyWitness, SWAPCLAIM_PROOF_PROVING_KEY};
use penumbra_sct::Nullifier;
use penumbra_tct as tct;

use criterion::{criterion_group, criterion_main, Criterion};
use rand_core::OsRng;

#[allow(clippy::too_many_arguments)]
const EPOCH_DURATION: usize = 20;

fn prove(r: Fq, s: Fq, public: SwapClaimProofPublic, private: SwapClaimProofPrivate) {
    let _proof = SwapClaimProof::prove(r, s, &SWAPCLAIM_PROOF_PROVING_KEY, public, private)
        .expect("can create proof");
}

fn setup_swap_claim_proving(rng: &mut OsRng) -> (SwapClaimProofPublic, SwapClaimProofPrivate, Fq, Fq) {
    let seed_phrase = SeedPhrase::generate(rng);
    let sk_recipient = SpendKey::from_seed_phrase_bip44(seed_phrase, &Bip44Path::new(0));
    let fvk_recipient = sk_recipient.full_viewing_key();
    let ivk_recipient = fvk_recipient.incoming();
    let (claim_address, _dtk_d) = ivk_recipient.payment_address(0u32.into());
    let nk = *sk_recipient.nullifier_key();

    // ... [rest of the setup code, similar to your original code]

    let r = Fq::rand(rng);
    let s = Fq::rand(rng);

    (public, private, r, s)
}

fn swap_claim_proving_time(c: &mut Criterion) {
    let mut rng = OsRng;

    let (public, private, r, s) = setup_swap_claim_proving(&mut rng);
    c.bench_function("swap claim proving", |b| {
        b.iter(|| prove(r, s, public.clone(), private.clone()))
    });

    print_constraints();
}

fn print_constraints() {
    let circuit = SwapClaimCircuit::with_dummy_witness();
    // ... [rest of the constraint printing code]
}

// ... [rest of your code]

criterion_group!(benches, swap_claim_proving_time);
criterion_main!(benches);
