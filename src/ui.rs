use cursive::views::Dialog;
use cursive::Cursive;
use std::ops::DerefMut;
use std::sync::Mutex;

use crate::uwsgi_reader::StatsReader;

use crate::ui_table::{create_table, UiTableView, UwsgiTableRow};
use crate::uwsgi_struct::UwsgiStatus;
use cursive::traits::*;
use lazy_static::lazy_static;

pub struct Ui {
    reader: Option<StatsReader>,
}

lazy_static! {
    pub static ref APP: Mutex<Ui> = Mutex::new(Ui { reader: None });
}

impl Ui {
    pub fn run(reader: StatsReader) {
        APP.lock().unwrap().reader = Some(reader);

        let mut siv = cursive::default();

        let table = create_table();

        siv.add_layer(Dialog::around(table.with_name("table").full_screen()));

        siv.add_global_callback('q', |s| s.quit());
        siv.add_global_callback('r', |s| Ui::refresh(s));

        Ui::refresh(siv.deref_mut());

        siv.run();
    }

    fn refresh(siv: &mut Cursive) {
        let mut table = siv
            .find_name::<UiTableView>("table")
            .expect("Should find table");

        let rows: Vec<UwsgiTableRow> = Ui::read()
            .workers
            .iter_mut()
            .filter_map(|w| {
                if w.id <= 0 {
                    return None;
                }

                Some(UwsgiTableRow::new_from_worker(w))
            })
            .collect();

        table.set_items(rows);
    }

    fn read() -> UwsgiStatus {
        APP.lock().unwrap().reader.as_mut().unwrap().read()
    }
}
