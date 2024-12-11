use sp1_core_executor::{ExecutionReport, HookEnv, SP1ContextBuilder};
use sp1_core_machine::io::SP1Stdin;
use sp1_primitives::io::SP1PublicValues;
use sp1_prover::{components::DefaultProverComponents, SP1ProvingKey};

use anyhow::{Ok, Result};
use sp1_stark::{SP1CoreOpts, SP1ProverOpts};
use std::time::Duration;

#[cfg(feature = "network-v2")]
use crate::network_v2::FulfillmentStrategy;

use crate::{
    opts::ProofOpts, proof::SP1ProofKind, proof::SP1ProofWithPublicValues, prover::Prover,
};

// /// Builder to prepare and configure execution of a program on an input.
// /// May be run with [Self::run].
// pub struct Execute<'a> {
//     prover: &'a dyn Prover<DefaultProverComponents>,
//     context_builder: SP1ContextBuilder<'a>,
//     elf: &'a [u8],
//     stdin: SP1Stdin,
// }

// impl<'a> Execute<'a> {
//     /// Prepare to execute the given program on the given input (without generating a proof).
//     ///
//     /// Prefer using [ProverClient::execute](super::ProverClient::execute).
//     /// See there for more documentation.
//     pub fn new(
//         prover: &'a dyn Prover<DefaultProverComponents>,
//         elf: &'a [u8],
//         stdin: SP1Stdin,
//     ) -> Self {
//         Self { prover, elf, stdin, context_builder: Default::default() }
//     }

//     /// Execute the program on the input, consuming the built action `self`.
//     pub fn run(self) -> Result<(SP1PublicValues, ExecutionReport)> {
//         let Self { prover, elf, stdin, mut context_builder } = self;
//         let context = context_builder.build();
//         Ok(prover.sp1_prover().execute(elf, &stdin, context)?)
//     }

//     /// Add a runtime [Hook](super::Hook) into the context.
//     ///
//     /// Hooks may be invoked from within SP1 by writing to the specified file descriptor `fd`
//     /// with [`sp1_zkvm::io::write`], returning a list of arbitrary data that may be read
//     /// with successive calls to [`sp1_zkvm::io::read`].
//     pub fn with_hook(
//         mut self,
//         fd: u32,
//         f: impl FnMut(HookEnv, &[u8]) -> Vec<Vec<u8>> + Send + Sync + 'a,
//     ) -> Self {
//         self.context_builder.hook(fd, f);
//         self
//     }

//     /// Avoid registering the default hooks in the runtime.
//     ///
//     /// It is not necessary to call this to override hooks --- instead, simply
//     /// register a hook with the same value of `fd` by calling [`Self::with_hook`].
//     pub fn without_default_hooks(mut self) -> Self {
//         self.context_builder.without_default_hooks();
//         self
//     }

//     /// Set the maximum number of cpu cycles to use for execution.
//     ///
//     /// If the cycle limit is exceeded, execution will return
//     /// [`sp1_core_executor::ExecutionError::ExceededCycleLimit`].
//     pub fn max_cycles(mut self, max_cycles: u64) -> Self {
//         self.context_builder.max_cycles(max_cycles);
//         self
//     }

//     /// Skip deferred proof verification.
//     pub fn set_skip_deferred_proof_verification(mut self, value: bool) -> Self {
//         self.context_builder.set_skip_deferred_proof_verification(value);
//         self
//     }
// }

// /// Builder to prepare and configure proving execution of a program on an input.
// /// May be run with [Self::run].
// pub struct Prove<'a> {
//     prover: &'a dyn Prover<DefaultProverComponents>,
//     kind: SP1ProofKind,
//     context_builder: SP1ContextBuilder<'a>,
//     pk: &'a SP1ProvingKey,
//     stdin: SP1Stdin,
//     core_opts: SP1CoreOpts,
//     recursion_opts: SP1CoreOpts,
//     timeout: Option<Duration>,
//     #[cfg(feature = "network-v2")]
//     fulfillment_strategy: Option<FulfillmentStrategy>,
//     #[cfg(feature = "network-v2")]
//     skip_simulation: bool,
// }

// impl<'a> Prove<'a> {
//     /// Prepare to prove the execution of the given program with the given input.
//     ///
//     /// Prefer using [ProverClient::prove](super::ProverClient::prove).
//     /// See there for more documentation.
//     pub fn new(
//         prover: &'a dyn Prover<DefaultProverComponents>,
//         pk: &'a SP1ProvingKey,
//         stdin: SP1Stdin,
//     ) -> Self {
//         Self {
//             prover,
//             kind: Default::default(),
//             pk,
//             stdin,
//             context_builder: Default::default(),
//             core_opts: SP1CoreOpts::default(),
//             recursion_opts: SP1CoreOpts::recursion(),
//             timeout: None,
//             #[cfg(feature = "network-v2")]
//             fulfillment_strategy: None,
//             #[cfg(feature = "network-v2")]
//             skip_simulation: false,
//         }
//     }

//     /// Prove the execution of the program on the input, consuming the built action `self`.
//     pub fn run(self) -> Result<SP1ProofWithPublicValues> {
//         let Self {
//             prover,
//             kind,
//             pk,
//             stdin,
//             mut context_builder,
//             core_opts,
//             recursion_opts,
//             timeout,
//             #[cfg(feature = "network-v2")]
//             fulfillment_strategy,
//             #[cfg(feature = "network-v2")]
//             skip_simulation,
//         } = self;
//         let opts = SP1ProverOpts { core_opts, recursion_opts };
//         let proof_opts = ProofOpts {
//             sp1_prover_opts: opts,
//             timeout,
//             #[cfg(feature = "network-v2")]
//             fulfillment_strategy,
//             #[cfg(feature = "network-v2")]
//             cycle_limit: context_builder.get_max_cycles(),
//             #[cfg(feature = "network-v2")]
//             skip_simulation,
//         };
//         let context = context_builder.build();

//         // Dump the program and stdin to files for debugging if `SP1_DUMP` is set.
//         if std::env::var("SP1_DUMP")
//             .map(|v| v == "1" || v.to_lowercase() == "true")
//             .unwrap_or(false)
//         {
//             let program = pk.elf.clone();
//             std::fs::write("program.bin", program).unwrap();
//             let stdin = bincode::serialize(&stdin).unwrap();
//             std::fs::write("stdin.bin", stdin.clone()).unwrap();
//         }

//         prover.prove(pk, stdin, proof_opts, context, kind)
//     }

//     /// Set the proof kind to the core mode. This is the default.
//     pub fn core(mut self) -> Self {
//         self.kind = SP1ProofKind::Core;
//         self
//     }

//     /// Set the proof kind to the compressed mode.
//     pub fn compressed(mut self) -> Self {
//         self.kind = SP1ProofKind::Compressed;
//         self
//     }

//     /// Set the proof mode to the plonk bn254 mode.
//     pub fn plonk(mut self) -> Self {
//         self.kind = SP1ProofKind::Plonk;
//         self
//     }

//     /// Set the proof mode to the groth16 bn254 mode.
//     pub fn groth16(mut self) -> Self {
//         self.kind = SP1ProofKind::Groth16;
//         self
//     }

//     /// Add a runtime [Hook](super::Hook) into the context.
//     ///
//     /// Hooks may be invoked from within SP1 by writing to the specified file descriptor `fd`
//     /// with [`sp1_zkvm::io::write`], returning a list of arbitrary data that may be read
//     /// with successive calls to [`sp1_zkvm::io::read`].
//     pub fn with_hook(
//         mut self,
//         fd: u32,
//         f: impl FnMut(HookEnv, &[u8]) -> Vec<Vec<u8>> + Send + Sync + 'a,
//     ) -> Self {
//         self.context_builder.hook(fd, f);
//         self
//     }

//     /// Avoid registering the default hooks in the runtime.
//     ///
//     /// It is not necessary to call this to override hooks --- instead, simply
//     /// register a hook with the same value of `fd` by calling [`Self::with_hook`].
//     pub fn without_default_hooks(mut self) -> Self {
//         self.context_builder.without_default_hooks();
//         self
//     }

//     /// Set the shard size for proving.
//     pub fn shard_size(mut self, value: usize) -> Self {
//         self.core_opts.shard_size = value;
//         self
//     }

//     /// Set the shard batch size for proving.
//     pub fn shard_batch_size(mut self, value: usize) -> Self {
//         self.core_opts.shard_batch_size = value;
//         self
//     }

//     /// Set whether we should reconstruct commitments while proving.
//     pub fn reconstruct_commitments(mut self, value: bool) -> Self {
//         self.core_opts.reconstruct_commitments = value;
//         self
//     }

//     /// Set the skip deferred proof verification flag.
//     pub fn set_skip_deferred_proof_verification(mut self, value: bool) -> Self {
//         self.context_builder.set_skip_deferred_proof_verification(value);
//         self
//     }

//     /// Set the maximum number of cpu cycles to use for execution. The request fails if the cycles
//     /// used exceed this limit.
//     ///
//     /// When set, this value will always be used as the cycle limit, regardless of the
//     /// `skip_simulation` setting.
//     ///
//     /// If this is not set:
//     /// - The cycle limit will be calculated by simulating the program (if simulation is enabled)
//     /// - The default cycle limit will be used (if simulation is disabled via `skip_simulation()`)
//     ///
//     /// In the case that cycle limit is greater than the cycles used, a refund will be issued.
//     ///
//     /// In the case of running locally, if the cycle limit is exceeded, execution will return
//     /// [`sp1_core_executor::ExecutionError::ExceededCycleLimit`]
//     pub fn cycle_limit(mut self, cycle_limit: u64) -> Self {
//         self.context_builder.max_cycles(cycle_limit);
//         self
//     }
//     /// Sets the timeout for the proof's generation. The network will ignore any requests that take longer
//     /// than this timeout.
//     ///
//     /// Additionally, the prover will stop polling for the proof request status when this
//     /// timeout is reached.
//     pub fn timeout(mut self, timeout: Duration) -> Self {
//         self.timeout = Some(timeout);
//         self
//     }

//     /// Sets the fulfillment strategy for the proof's generation.
//     ///
//     /// See `FulfillmentStrategy` for more details about each strategy.
//     #[cfg(feature = "network-v2")]
//     pub fn strategy(mut self, strategy: FulfillmentStrategy) -> Self {
//         self.fulfillment_strategy = Some(strategy);
//         self
//     }

//     /// Disables simulation for cycle limit calculation.
//     ///
//     /// This is useful if program execution requires significant computation, and you already have
//     /// an expected cycle count you can use with `cycle_limit()`.
//     ///
//     /// When simulation is disabled:
//     /// - If a cycle limit was set via `cycle_limit()`, that value will be used
//     /// - Otherwise, the default cycle limit will be used
//     #[cfg(feature = "network-v2")]
//     pub fn skip_simulation(mut self) -> Self {
//         self.skip_simulation = true;
//         self
//     }
// }
