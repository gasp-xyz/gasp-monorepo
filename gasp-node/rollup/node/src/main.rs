//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod metrics;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
