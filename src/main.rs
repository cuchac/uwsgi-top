use crate::ui::Ui;

mod ui;
mod ui_table;
mod uwsgi_reader;
mod uwsgi_struct;

fn main() {
    let reader = uwsgi_reader::StatsReader::new("./test/input.json");

    Ui::run(reader);
}
