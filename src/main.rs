use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
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

    /// Tag value type.
    // #[arg(value_enum)]
    // tag_type: Option<TagType>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TagType {
    Bool,
    Int,
    Dint,
    Real,
}

#[derive(Subcommand)]
enum Commands {
    /// List controller tags.
    List,
    /// Read the INT value of a tag.
    ReadInt {
        tag: String,
    },
    /// Read the DINT value of a tag.
    ReadDint {
        tag: String,
    },
    /// Read the REAL value of a tag.
    ReadReal {
        tag: String,
    },
    /// Read the BOOL value of a tag.
    ReadBool {
        tag: String,
    },
    Write {
        tag: String,
        tag_type: TagType,
    },
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
                        println!("    {}    {:?}", item.name, item.symbol_type);
                    }
                })
                .await;
        }
        Some(Commands::ReadInt { tag }) => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<i16> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type, &tag_value.value
            );
        }
        Some(Commands::ReadDint { tag }) => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<i32> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type, &tag_value.value
            );
        }
        Some(Commands::ReadReal { tag }) => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<f32> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type, &tag_value.value
            );
        }
        Some(Commands::ReadBool { tag }) => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<bool> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type, &tag_value.value
            );
        }
        Some(_) => {}
        None => {}
    }

    client.close().await?;
    Ok(())
}
