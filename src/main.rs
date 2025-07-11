mod config;
use clap::{Args, Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(name = "easyiac")]
struct Cli {
    #[command(subcommand)]
    command: IacCommands,
}

#[derive(Debug, Subcommand)]
enum IacCommands {
    Fuga(FugaArgs),
    Test,
}

#[derive(Debug, Args)]
struct FugaArgs {
    #[command(subcommand)]
    command: FugaCommands,
}

#[derive(Debug, Subcommand)]
enum FugaCommands {
    Piyo,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        IacCommands::Fuga(fuga_args) => match fuga_args.command {
            FugaCommands::Piyo => {
                println!("piyo");
            }
        },
        IacCommands::Test => {
            server().unwrap();
        }
    }
}



fn server() -> Result<(), Box<dyn std::error::Error>> {
    // server.toml を読み込む
    let config = config::Config::load_from_file("server.toml")?;

    // 値を表示
    println!("Server: {}", config.profile.server);
    println!("User: {}", config.profile.user);
    println!("Password: {}", config.profile.passwd);

    Ok(())
}