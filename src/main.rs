use std::fmt::Display;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use futures_util::StreamExt;
use rseip::client::ab_eip::*;
use rseip::precludes::*;

#[derive(Parser)]
#[command(name = "Cobalt")]
#[command(author = "Abdelkader Madoui. <abdelkadermadoui@protonmail.com>")]
#[command(version = "1.0")]
#[command(
    about = "A command line utility for parsing and reading tags on Allen Bradley CompactLogix PLCs.",
    long_about = "Cobalt is an open source utility for communicating with Allen Bradley PLCs. That includes reading and writing tag values and listing controller tags."
)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// PLC address
    #[arg(short, long)]
    address: String,

    /// Commands
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List controller tags.
    List,
    /// Read the INT value of a tag.
    ReadInt { tag: String },
    /// Read the DINT value of a tag.
    ReadDint { tag: String },
    /// Read the REAL value of a tag.
    ReadReal { tag: String },
    /// Read the BOOL value of a tag.
    ReadBool { tag: String },
    /// Write a BOOL value to the specified tag.
    WriteBool { tag: String, value: BoolValue },
    /// Write an INT value to the specified tag.
    WriteInt { tag: String, value: i16 },
    /// Write a DINT value to the specified tag.
    WriteDint { tag: String, value: i32 },
    /// Write a REAL value to the specified tag.
    WriteReal { tag: String, value: f32 },
}

#[derive(Clone, Subcommand, ValueEnum)]
enum BoolValue {
    False,
    True,
}

impl Display for BoolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoolValue::False => {
                write!(f, "false")
            }
            BoolValue::True => {
                write!(f, "true")
            }
        }
    }
}

#[cfg(not(windows))]
#[tokio::main]
pub async fn main() -> Result<()> {
    let cli = Args::parse();

    let address: String = cli.address;

    let mut client = AbEipClient::new_host_lookup(address)
        .await?
        .with_connection_path(PortSegment::default());

    match &cli.command {
        Commands::List => {
            let stream = client.list_tag().call();
            stream
                .for_each(|item| async move {
                    if let Ok(item) = item {
                        println!("    {}    {:?}", item.name.bold(), item.symbol_type);
                    }
                })
                .await;
        }
        Commands::ReadInt { tag } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<i16> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::ReadDint { tag } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<i32> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::ReadReal { tag } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<f32> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::ReadBool { tag } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<bool> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::WriteInt { tag, value } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value = TagValue {
                tag_type: TagType::Int,
                value: *value,
            };
            client.write_tag(tag, &tag_value).await.unwrap();
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::WriteDint { tag, value } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value = TagValue {
                tag_type: TagType::Dint,
                value: *value,
            };
            client.write_tag(tag, &tag_value).await.unwrap();
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::WriteBool { tag, value } => {
            let tag = EPath::parse_tag(tag)?;

            match value {
                BoolValue::False => {
                    let tag_value = TagValue {
                        tag_type: TagType::Bool,
                        value: false,
                    };
                    client.write_tag(tag, &tag_value).await.unwrap();
                    println!(
                        "Tag type:    {:?}    Tag value:    {}",
                        &tag_value.tag_type,
                        &tag_value.value.to_string().bold().green(),
                    );
                }
                BoolValue::True => {
                    let tag_value = TagValue {
                        tag_type: TagType::Bool,
                        value: true,
                    };
                    client.write_tag(tag, &tag_value).await.unwrap();
                    println!(
                        "Tag type:    {:?}    Tag value:    {}",
                        &tag_value.tag_type,
                        &tag_value.value.to_string().bold().green(),
                    );
                }
            }
        }
        Commands::WriteReal { tag, value } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value = TagValue {
                tag_type: TagType::Real,
                value: *value,
            };
            client.write_tag(tag, &tag_value).await.unwrap();
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
    }

    client.close().await?;
    Ok(())
}

#[cfg(windows)]
#[tokio::main]
pub async fn main() -> Result<()> {
    colored::control::set_virtual_terminal(true);
    let cli = Args::parse();

    let address: String = cli.address;

    let mut client = AbEipClient::new_host_lookup(address)
        .await?
        .with_connection_path(PortSegment::default());

    match &cli.command {
        Commands::List => {
            let stream = client.list_tag().call();
            stream
                .for_each(|item| async move {
                    if let Ok(item) = item {
                        println!("    {}    {:?}", item.name.bold(), item.symbol_type);
                    }
                })
                .await;
        }
        Commands::ReadInt { tag } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<i16> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::ReadDint { tag } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<i32> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::ReadReal { tag } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<f32> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::ReadBool { tag } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value: TagValue<bool> = client.read_tag(tag.clone()).await?;
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::WriteBool { tag, value } => {
            let tag = EPath::parse_tag(tag)?;

            match value {
                BoolValue::False => {
                    let tag_value = TagValue {
                        tag_type: TagType::Bool,
                        value: false,
                    };
                    client.write_tag(tag, &tag_value).await.unwrap();
                    println!(
                        "Tag type:    {:?}    Tag value:    {}",
                        &tag_value.tag_type,
                        &tag_value.value.to_string().bold().green(),
                    );
                }
                BoolValue::True => {
                    let tag_value = TagValue {
                        tag_type: TagType::Bool,
                        value: true,
                    };
                    client.write_tag(tag, &tag_value).await.unwrap();
                    println!(
                        "Tag type:    {:?}    Tag value:    {}",
                        &tag_value.tag_type,
                        &tag_value.value.to_string().bold().green(),
                    );
                }
            }
        }
        Commands::WriteInt { tag, value } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value = TagValue {
                tag_type: TagType::Int,
                value: *value,
            };
            client.write_tag(tag, &tag_value).await.unwrap();
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::WriteDint { tag, value } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value = TagValue {
                tag_type: TagType::Dint,
                value: *value,
            };
            client.write_tag(tag, &tag_value).await.unwrap();
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
        Commands::WriteReal { tag, value } => {
            let tag = EPath::parse_tag(tag)?;
            let tag_value = TagValue {
                tag_type: TagType::Real,
                value: *value,
            };
            client.write_tag(tag, &tag_value).await.unwrap();
            println!(
                "Tag type:    {:?}    Tag value:    {}",
                &tag_value.tag_type,
                &tag_value.value.to_string().bold().green(),
            );
        }
    }

    client.close().await?;
    Ok(())
}
