
use anyhow::{Result, Context};
use async_compression::tokio::bufread::ZstdDecoder;
use async_compression::tokio::write::ZstdEncoder;
use clap::Parser;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt ;
use tokio::io::BufReader;

pub async fn run_decode(_args: DecodeArgs) -> Result<()> {
    let reader = BufReader::new(tokio::io::stdin());
    let mut reader = ZstdDecoder::new(reader);

    let mut writer = tokio::io::stdout();

    let mut buf = vec![0; 1024];
    loop {
        let n = reader.read_buf(&mut buf).await.with_context(||"read fail")?;
        if n == 0 {
            break;
        }
        writer.write_all(&buf[..n]).await.with_context(||"write fail")?;
    }

    writer.flush().await.with_context(||"flush fail")?;
    writer.shutdown().await.with_context(||"shutdown fail")?;

    Ok(())
}

pub async fn run_encode(_args: EncodeArgs) -> Result<()> {
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut writer = ZstdEncoder::new(tokio::io::stdout());

    let mut buf = vec![0; 1024];
    loop {
        let n = reader.read_buf(&mut buf).await.with_context(||"read fail")?;
        if n == 0 {
            break;
        }
        writer.write_all(&buf[..n]).await.with_context(||"write fail")?;
    }

    writer.flush().await.with_context(||"flush fail")?;
    writer.shutdown().await.with_context(||"shutdown fail")?;

    Ok(())
}

#[derive(Parser, Debug, Clone)]
#[clap(about = "read from stdin and decode to stdout")]
pub struct DecodeArgs {}

#[derive(Parser, Debug, Clone)]
#[clap(about = "read from stdin and encode to stdout")]
pub struct EncodeArgs {}
