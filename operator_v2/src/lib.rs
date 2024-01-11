use cli::CliArgs;
use config::NodeConfig;
use operator::Operator;

mod cli;
mod config;
mod executor;
mod operator;

pub mod bindings {
    pub use bindings::incredible_squaring_service_manager::IncredibleSquaringServiceManager as ServiceManager;
    pub use bindings::incredible_squaring_task_manager::IncredibleSquaringTaskManager as TaskManager;
}

pub async fn subscribe_new_task() -> eyre::Result<()> {
    let cli = CliArgs::build();
    let config = NodeConfig::from_path(&cli.config)?;
    let operator = Operator::from_config(&config).await?;

    // operator.watch_new_tasks().await?;
    operator.execute_block().await?;

    Ok(())
}
