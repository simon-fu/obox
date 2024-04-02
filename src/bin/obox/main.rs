
use anyhow::Result;
use clap::{Parser, Subcommand};


mod cmd_zstd;

#[tokio::main]
async fn main() -> Result<()>{
    let args = Args::parse();
    match args.command {
        Commands::ZstdDec(args) => cmd_zstd::run_decode(args).await,
        Commands::ZstdEnc(args) => cmd_zstd::run_encode(args).await,
    }

}



#[derive(Parser, Debug, Clone)]
#[clap(name = "obox", author, about = "ops tools box", long_about = None, version, )]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone)]
#[derive(Subcommand)]
pub enum Commands {
    ZstdDec(cmd_zstd::DecodeArgs),
    ZstdEnc(cmd_zstd::EncodeArgs),
}


