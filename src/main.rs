use crate::ui::Ui;

use clap::Parser;

mod ui;
mod ui_table;
mod uwsgi_reader;
mod uwsgi_struct;

/// Top-like interface for uwsgi workers status
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Settings {
    #[arg(short, long, value_name = "/path/to/uwsgi/dump.json")]
    file: Option<std::path::PathBuf>,
    #[arg(short, long, value_name = "ip:port")]
    network: Option<std::net::SocketAddr>,
    #[arg(short, long, value_name = "/path/to/uwsgi/stats/socket")]
    socket: Option<std::path::PathBuf>,
}

fn main() {
    let args = Settings::parse();

    let reader = uwsgi_reader::StatsReader::new(&args);

    Ui::run(reader);
}
