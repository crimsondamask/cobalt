use std::{f32::consts::PI, fmt::Display};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use futures_util::StreamExt;
use rseip::client::ab_eip::*;
use rseip::precludes::*;
use std::io::{self, Write};
use std::time::Duration;
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;

#[derive(Parser)]
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
    /// Bridge a serial Modbus RTU to the PLC.
    BridgeWrite {
        port: String,
        slave: u8,
        baudrate: u32,
        rtu_register_velocity: u16,
        rtu_register_rate: u16,
        pressure_tag: String,
        temperature_tag: String,
        diameter: f32,
        rate_tag_base: String,
        rate_tag: String,
    },
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
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Commands::BridgeWrite {
            port,
            slave,
            baudrate,
            rtu_register_velocity,
            rtu_register_rate,
            pressure_tag,
            temperature_tag,
            diameter,
            rate_tag_base,
            rate_tag,
        } => {
            let pressure_tag = EPath::parse_tag(pressure_tag)?;
            let temperature_tag = EPath::parse_tag(temperature_tag)?;
            let rate_tag = EPath::parse_tag(rate_tag)?;
            let rate_tag_base = EPath::parse_tag(rate_tag_base)?;

            let slave = Slave(*slave);
            let builder = tokio_serial::new(port, *baudrate);
            let stream = SerialStream::open(&builder).unwrap();
            let mut ctx = rtu::connect_slave(stream, slave).await.unwrap();

            println!("Connected to slave over {}", port.bold());
            println!("Starting bridge loop.");

            loop {
                let rsp = ctx
                    .read_holding_registers(*rtu_register_velocity, 2)
                    .await?;
                let velocity = u16_to_f32(rsp[0], rsp[1]);
                let rsp = ctx.read_holding_registers(*rtu_register_rate, 2).await?;
                let rate = u16_to_f32(rsp[0], rsp[1]);
                let pressure: TagValue<f32> = client.read_tag(pressure_tag.clone()).await?;
                let temperature: TagValue<f32> = client.read_tag(temperature_tag.clone()).await?;
                let rate_base =
                    velocity_to_rate(velocity, *diameter, pressure.value, temperature.value);

                let now = chrono::Local::now();
                io::stdout().flush().unwrap();
                print!(
                    "\r[{}] ===> Velocity: {} m/s, P: {} barg, T: {} degC, Q: {} Sm3/d",
                    now, velocity, pressure.value, temperature.value, rate_base
                );

                let rate_to_plc = TagValue {
                    tag_type: TagType::Real,
                    value: rate,
                };
                let rate_to_plc_base = TagValue {
                    tag_type: TagType::Real,
                    value: rate_base,
                };
                client
                    .write_tag(rate_tag.clone(), &rate_to_plc)
                    .await
                    .unwrap();
                client
                    .write_tag(rate_tag_base.clone(), &rate_to_plc_base)
                    .await
                    .unwrap();
                std::thread::sleep(Duration::from_millis(500));
            }
        }
    }

    client.close().await?;
    Ok(())
}

#[cfg(windows)]
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Commands::BridgeWrite {
            port,
            slave,
            baudrate,
            rtu_register_velocity,
            rtu_register_rate,
            pressure_tag,
            temperature_tag,
            diameter,
            rate_tag_base,
            rate_tag,
        } => {
            let pressure_tag = EPath::parse_tag(pressure_tag)?;
            let temperature_tag = EPath::parse_tag(temperature_tag)?;
            let rate_tag = EPath::parse_tag(rate_tag)?;
            let rate_tag_base = EPath::parse_tag(rate_tag_base)?;

            let slave = Slave(*slave);
            let builder = tokio_serial::new(port, *baudrate);
            let stream = SerialStream::open(&builder).unwrap();
            let mut ctx = rtu::connect_slave(stream, slave).await.unwrap();

            println!("Connected to slave over {}", port.bold());
            println!("Starting bridge loop.");

            loop {
                let rsp = ctx
                    .read_holding_registers(*rtu_register_velocity, 2)
                    .await?;
                let velocity = u16_to_f32(rsp[0], rsp[1]);
                let rsp = ctx.read_holding_registers(*rtu_register_rate, 2).await?;
                let rate = u16_to_f32(rsp[0], rsp[1]);
                let pressure: TagValue<f32> = client.read_tag(pressure_tag.clone()).await?;
                let temperature: TagValue<f32> = client.read_tag(temperature_tag.clone()).await?;
                let rate_base =
                    velocity_to_rate(velocity, *diameter, pressure.value, temperature.value);

                let now = chrono::Local::now();
                io::stdout().flush().unwrap();
                print!(
                    "\r[{}] ===> Velocity: {} m/s, P: {} barg, T: {} degC, Q: {} Sm3/d",
                    now,
                    velocity.to_string().bold().green(),
                    pressure.value.to_string().bold().green(),
                    temperature.value.to_string().bold().green(),
                    rate_base.to_string().bold().green()
                );

                let rate_to_plc = TagValue {
                    tag_type: TagType::Real,
                    value: rate,
                };
                let rate_to_plc_base = TagValue {
                    tag_type: TagType::Real,
                    value: rate_base,
                };
                client
                    .write_tag(rate_tag.clone(), &rate_to_plc)
                    .await
                    .unwrap();
                client
                    .write_tag(rate_tag_base.clone(), &rate_to_plc_base)
                    .await
                    .unwrap();
                std::thread::sleep(Duration::from_millis(500));
            }
        }
    }

    client.close().await?;
    Ok(())
}

fn u16_to_f32(first: u16, second: u16) -> f32 {
    let data_32bit_rep = ((first as u32) << 16) | second as u32;
    let data_32_array = data_32bit_rep.to_ne_bytes();
    f32::from_ne_bytes(data_32_array)
}

fn velocity_to_rate(velocity: f32, diameter: f32, pressure: f32, temperature: f32) -> f32 {
    use aga8::composition::Composition;
    use aga8::detail::Detail;

    let mut aga8_test: Detail = Detail::new();

    let comp = Composition {
        methane: 0.79,
        nitrogen: 0.04,
        carbon_dioxide: 0.04,
        ethane: 0.0,
        propane: 0.13,
        isobutane: 0.0,
        n_butane: 0.0,
        isopentane: 0.0,
        n_pentane: 0.0,
        hexane: 0.0,
        heptane: 0.0,
        octane: 0.0,
        nonane: 0.0,
        decane: 0.0,
        hydrogen: 0.0,
        oxygen: 0.0,
        carbon_monoxide: 0.0,
        water: 0.0,
        hydrogen_sulfide: 0.0,
        helium: 0.0,
        argon: 0.0,
    };

    aga8_test.set_composition(&comp).unwrap();
    aga8_test.p = pressure as f64 * 100.0;
    aga8_test.t = temperature as f64 + 273.15;

    aga8_test.density();
    aga8_test.properties();
    let z_f = aga8_test.z;

    aga8_test.p = 14.73 * 6.89476;
    aga8_test.t = ((60.0 as f64) - 32.0) * 5.0 / 9.0 + 273.15;
    aga8_test.density();
    aga8_test.properties();
    let z_b = aga8_test.z;

    let act_flow =
        (PI * (diameter / 12.0) * (diameter / 12.0) / 4.0) * (velocity * 3.28083) * 3600.0;

    let base_flow = ((act_flow * (((pressure / 0.068947573) + 14.696) * 6894.7573)
        / (14.73 * 6894.7573))
        * ((288.7056) / (temperature + 273.15))
        * (z_b / z_f) as f32)
        * 0.0283168466
        * 24.0;
    base_flow
}
