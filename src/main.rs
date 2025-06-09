use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use obws::Client;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(long, default_value = "localhost")]
        ip: String,

        #[arg(long, default_value = "4455")]
        port: u16,

        #[arg(long)]
        password: Option<String>,

        shortcut: Shortcut,
    },
}

#[derive(ValueEnum, Clone)]
enum Shortcut {
    ToggleReplayBuffer,
    SaveReplayBuffer,

    ToggleRecord,
    ToggleRecordPause,
}

impl Shortcut {
    async fn run(&self, ip: String, port: u16, password: Option<String>) -> Result<()> {
        let client = Client::connect(ip, port, password).await?;

        match self {
            Shortcut::ToggleReplayBuffer => {
                client.replay_buffer().toggle().await?;
            }
            Shortcut::SaveReplayBuffer => {
                client.replay_buffer().save().await?;
            }

            Shortcut::ToggleRecord => {
                client.recording().toggle().await?;
            }
            Shortcut::ToggleRecordPause => {
                client.recording().toggle_pause().await?;
            }
        };

        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            ip,
            port,
            password,
            shortcut,
        } => shortcut.run(ip, port, password).await,
    }?;

    Ok(())
}
