use anyhow::Result;
use clap::{Parser, Subcommand};
use futures_util::StreamExt;
use rseip::client::ab_eip::*;
use rseip::precludes::*;

#[derive(Parser)]
#[command(name = "Allen Bradley Parser")]
#[command(author = "Abdelkader Madoui. <abdelkadermadoui@protonmail.com>")]
#[command(version = "1.0")]
#[command(about = "A command line utility for parsing and reading tags on Allen Bradley ControLogix PLCs.", long_about = None)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// PLC address
    #[arg(short, long)]
    address: String,

    /// Controller tag to read.
    #[arg(short, long)]
    tag: Option<String>,

    /// Value to write into the specified tag.
    #[arg(short, long)]
    value: Option<f32>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List controller tags.
    List,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let cli = Args::parse();

    let address: String = cli.address;

    let mut client = AbEipClient::new_host_lookup(address)
        .await?
        .with_connection_path(PortSegment::default());

    match &cli.command {
        Some(Commands::List) => {
            let stream = client.list_tag().call();
            stream
                .for_each(|item| async move {
                    if let Ok(item) = item {
                        println!("{:?}", item.symbol_type);
                        println!("=====>     {:?}", item);
                    }
                })
                .await;
        }
        None => {}
    }

    if let Some(tag) = cli.tag.as_deref() {
        let tag = EPath::parse_tag(tag)?;
        println!("reading tag...");
        let value: TagValue<f32> = client.read_tag(tag.clone()).await?;
        println!(
            "Tag type: {:?}. Tag value: {:?}",
            &value.tag_type, &value.value
        );
    }

    if let (Some(value), Some(tag)) = (cli.value, cli.tag.as_deref()) {
        let tag = EPath::parse_tag(tag)?;
        let mut tag_value: TagValue<f32> = client.read_tag(tag.clone()).await?;
        tag_value.value = value;
        client.write_tag(tag, &tag_value).await?;
        println!("Tag written:    value:    {}", &tag_value.value);
    }

    client.close().await?;
    Ok(())
}
