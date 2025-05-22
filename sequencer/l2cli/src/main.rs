use clap::arg;
use clap::Parser;
use common::PKeyWrapper;
use l2api::L2Error;
use l2api::L2Interface;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long)]
    pub uri: String,

    #[arg(long, value_parser = common::parse_pkey)]
    pub private_key: PKeyWrapper,

    #[arg(long, default_value_t = 1)]
    pub count: u32,

    #[arg(long, default_value_t = 1)]
    pub chain: u32,

    #[arg(long, value_parser = common::parse_addr)]
    pub recipient: Option<[u8; 20]>,

    #[arg(long, value_parser = common::parse_addr)]
    pub token: [u8; 20],

    #[arg(long, default_value_t = 2)]
    pub amount: u128,

    #[arg(long)]
    pub tip: Option<u128>,
}

#[tokio::main]
async fn main() -> Result<(), L2Error> {
    tracing_subscriber::fmt().init();

    let args = Cli::parse();
    let l2 = l2api::Gasp::new(&args.uri, args.private_key.into()).await?;
    let chain = gasp_types::Chain::try_from(args.chain).expect("is valid chain");
    let recipient = args.recipient.unwrap_or(l2.account_address());

    for i in 1..=args.count {
        tracing::info!("sending withdrawal {i} / {}", args.count);
        match l2
            .withdraw(chain, recipient, args.token, args.amount, args.tip)
            .await
        {
            Ok(true) => {
                tracing::info!("withdrawal successful");
            }
            Ok(false) => {
                tracing::info!("withdrawal failed");
            }
            Err(e) => {
                tracing::error!("Err: {e}");
            }
        }
    }
    Ok(())
}
