use std::sync::Mutex;
use cursive::{Cursive};
use cursive::views::{Dialog,};

use crate::uwsgi_reader::StatsReader;

use cursive::traits::*;
use lazy_static::lazy_static;
use crate::ui_table::{create_table, Foo, UiTableView};
use crate::uwsgi_struct::UwsgiStatus;

pub struct Ui {
    reader: Option<StatsReader>,
}

lazy_static! {
    pub static ref APP: Mutex<Ui> = Mutex::new(Ui {
        reader: None,
    });
}

impl Ui {
    pub fn run(reader: StatsReader) {
        APP.lock().unwrap().reader = Some(reader);

        let mut siv = cursive::default();

        siv.add_global_callback('q', |s| s.quit());
        siv.add_global_callback('r', |s| Ui::refresh(s));

        let table = create_table();

        siv.add_layer(Dialog::around(table.with_name("table").full_screen()));

        siv.run();
    }

    fn refresh(siv: &mut Cursive) {

        let mut table = siv.find_name::<UiTableView>("table").expect("Should find table");

        let rows: Vec<Foo> = Ui::read().workers.iter().map(|w| {
            Foo::new_from_worker(w)
        }).collect();

        table.set_items(rows);
    }

    fn read() -> UwsgiStatus {
        APP.lock().unwrap().reader.as_mut().unwrap().read()
    }
}