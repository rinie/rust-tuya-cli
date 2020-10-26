use anyhow::Error;
use clap::{App, Arg};
use rust_tuyapi::{payload, tuyadevice::TuyaDevice, TuyaType};
use std::net::SocketAddr;

type Result<T> = std::result::Result<T, Error>;
fn main() -> Result<()> {
    let matches = App::new("tuya-cli")
        .version("1.0")
        .author("Emil Södergren <emil.sodergren@outlook.com>")
        .about("Uses rust-tuyapi to communicate with Smart Life/Tuya devices")
        .subcommand(
            App::new("set")
                .about("Send command to devices")
                .arg(
                    Arg::new("key")
                        .about("Tuya key")
                        .long("key")
                        .takes_value(true),
                )
                .arg(Arg::new("id").about("Tuya id").long("id").takes_value(true))
                .arg(
                    Arg::new("ip")
                        .about("Ip address to the tuya device")
                        .long("ip")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("version")
                        .about("Tuya protocol version [3.1 or 3.3]")
                        .short('v')
                        .long("protocol-version")
                        .takes_value(true)
                        .default_value("3.3")
                        .required(false),
                )
                .arg(
                    Arg::new("raw-value")
                        .about("Tuya protocol version [3.1 or 3.3]")
                        .long("value")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("dps")
                        .about("DPS index to set")
                        .long("dps")
                        .takes_value(true)
                        .default_value("1"),
                ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("set") {
        let version = matches.value_of("version").unwrap();
        let key = matches.value_of("key");
        let ip: SocketAddr = matches.value_of("key").unwrap().parse()?;
        let id = matches.value_of("id").unwrap();
        let value = matches.value_of("raw-value").unwrap();
        let td = TuyaDevice::create(version, key, ip)?;
        let payload = payload(id, TuyaType::Socket, value)?;
        td.set(&payload, 0)?;
    };
    Ok(())
}
