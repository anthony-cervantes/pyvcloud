use clap::{Parser, Subcommand};
use pyvcloud::{ClientError, VCloudClient};
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Parser, Debug)]
#[command(version, about = "Rust CLI for VMware vCloud Director", long_about = None)]
struct Cli {
    /// Base URL for the vCloud Director instance (e.g. https://vcloud.example.com)
    #[arg(long, env = "PVCLOUD_BASE_URL")]
    base_url: String,

    /// Organization to operate against.
    #[arg(long, default_value = "System", env = "PVCLOUD_ORG")]
    organization: String,

    /// Authorization token to reuse across commands.
    #[arg(long, env = "PVCLOUD_TOKEN")]
    token: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Perform a simple GET /api to confirm connectivity.
    HealthCheck,
    /// Login using username and password and print the resulting token.
    Login {
        /// Username used for authentication.
        username: String,
        /// Password used for authentication.
        password: String,
    },
    /// List organizations visible to the authenticated user.
    ListOrgs,
}

fn main() -> Result<(), ClientError> {
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter_layer).init();

    let cli = Cli::parse();
    let mut client = VCloudClient::new(&cli.base_url, &cli.organization)?;
    if let Some(token) = cli.token {
        client = client.with_token(token);
    }

    match cli.command {
        Command::HealthCheck => {
            client.health_check()?;
            println!("vCloud API at {} is reachable", cli.base_url);
        }
        Command::Login { username, password } => {
            let token = client.login_with_credentials(&username, &password)?;
            println!("Received token: {}", token);
        }
        Command::ListOrgs => {
            let orgs = client.list_organizations()?;
            if orgs.is_empty() {
                println!("No organizations returned.");
            } else {
                for org in orgs {
                    println!("- {} ({})", org.name, org.id);
                }
            }
        }
    }

    Ok(())
}
