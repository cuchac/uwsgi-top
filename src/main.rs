use crate::ui::{Ui};

mod uwsgi_reader;
mod uwsgi_struct;
mod ui;
mod ui_table;


fn main() {
    let reader = uwsgi_reader::StatsReader::new("./test/input.json");

    Ui::run(reader);
}
