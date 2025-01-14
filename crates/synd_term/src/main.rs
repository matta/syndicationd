use std::{future, path::PathBuf, time::Duration};

use anyhow::Context as _;
use crossterm::event::EventStream;
use futures_util::TryFutureExt;
use synd_term::{
    application::{Application, Cache, Config},
    cli::{self, ApiOptions, Args, FeedOptions, Palette},
    client::Client,
    config::{self, Categories},
    terminal::Terminal,
    ui::theme::Theme,
};
use tracing::error;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use url::Url;

fn init_tracing(log_path: Option<PathBuf>) -> anyhow::Result<Option<WorkerGuard>> {
    use synd_o11y::opentelemetry::init_propagation;
    use tracing_subscriber::{
        filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt as _, Registry,
    };

    let (writer, guard) = if let Some(log_path) = log_path {
        if let Some(parent) = log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let log = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_path)?;
        let (non_blocking, guard) = tracing_appender::non_blocking(log);
        (BoxMakeWriter::new(non_blocking), Some(guard))
    } else {
        (BoxMakeWriter::new(std::io::stdout), None)
    };

    Registry::default()
        .with(
            fmt::Layer::new()
                .with_ansi(true)
                .with_timer(fmt::time::UtcTime::rfc_3339())
                .with_file(false)
                .with_line_number(false)
                .with_target(true)
                .with_writer(writer),
        )
        .with(
            EnvFilter::try_from_env(config::env::LOG_DIRECTIVE)
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .try_init()?;

    // Set text map progator globally
    init_propagation();

    Ok(guard)
}

fn build_app(
    endpoint: Url,
    timeout: Duration,
    palette: Palette,
    FeedOptions {
        categories,
        entries_limit,
    }: FeedOptions,
    cache_dir: PathBuf,
) -> anyhow::Result<Application> {
    let app = Application::builder()
        .terminal(Terminal::new().context("Failed to construct terminal")?)
        .client(Client::new(endpoint, timeout).context("Failed to construct client")?)
        .categories(
            categories
                .map(Categories::load)
                .transpose()?
                .unwrap_or_else(Categories::default_toml),
        )
        .config(Config {
            entries_limit,
            ..Default::default()
        })
        .cache(Cache::new(cache_dir))
        .theme(Theme::with_palette(&palette.into()))
        .build();

    Ok(app)
}

#[tokio::main]
async fn main() {
    let Args {
        api: ApiOptions {
            endpoint,
            client_timeout,
        },
        feed,
        log,
        cache_dir,
        command,
        palette,
    } = cli::parse();

    // Subcommand logs to the terminal, tui writes logs to a file.
    let log = if command.is_some() { None } else { Some(log) };
    let _guard = init_tracing(log).unwrap();

    if let Some(command) = command {
        let exit_code = match command {
            cli::Command::Clean(clean) => clean.run(),
            cli::Command::Check(check) => check.run(endpoint).await,
            cli::Command::Export(export) => export.run(endpoint).await,
        };

        std::process::exit(exit_code);
    };

    let mut event_stream = EventStream::new();

    if let Err(err) = future::ready(build_app(
        endpoint,
        client_timeout,
        palette,
        feed,
        cache_dir,
    ))
    .and_then(|app| {
        tracing::info!("Running...");
        app.run(&mut event_stream)
    })
    .await
    {
        error!("{err:?}");
        std::process::exit(1);
    }
}
