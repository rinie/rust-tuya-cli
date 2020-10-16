use anyhow::Error;
use clap::{App, Arg};

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
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::new("id").about("Tuya id").long("id").takes_value(true))
                .arg(
                    Arg::new("ip")
                        .about("Ip address to the tuya device")
                        .long("ip")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("set") {
        println!(
            "Value of ip: {}, key: {}",
            matches.value_of("ip").unwrap(),
            matches.value_of("key").unwrap()
        );
    };

    Ok(())
}
