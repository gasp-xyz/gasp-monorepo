use sc_cli::RunCmd;

#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[arg(long, env, default_value_t = false, conflicts_with_all = &["chain_genesis_salt"], global = true)]
	pub randomize_chain_genesis_salt: bool,

	#[arg(long, env, conflicts_with_all = &["randomize_chain_genesis_salt"], global = true)]
	pub chain_genesis_salt: Option<String>,

	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[arg(long, value_parser, value_delimiter = ',', global = true)]
	pub override_eth_sequencers: Vec<String>,

	#[arg(long, value_parser, value_delimiter = ',', global = true)]
	pub override_arb_sequencers: Vec<String>,

	#[arg(long, value_parser, value_delimiter = ',', global = true)]
	pub override_base_sequencers: Vec<String>,

	#[arg(long, value_parser, value_delimiter = ',', global = true)]
	pub override_monad_sequencers: Vec<String>,

	#[arg(long, value_parser, value_delimiter = ',', global = true)]
	pub override_megaeth_sequencers: Vec<String>,

	#[arg(long, value_parser, value_delimiter = ',', global = true)]
	pub override_sonic_sequencers: Vec<String>,

	#[clap(flatten)]
	pub run: RunCmd,
}

#[derive(Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommand {
	/// Key management cli utilities
	#[command(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Sub-commands concerned with benchmarking.
	#[command(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try-runtime has migrated to a standalone CLI
	/// (<https://github.com/paritytech/try-runtime-cli>). The subcommand exists as a stub and
	/// deprecation notice. It will be removed entirely some time after Janurary 2024.
	TryRuntime,

	/// Db meta columns information.
	ChainInfo(sc_cli::ChainInfoCmd),
}
