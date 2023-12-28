use cursive::views::{Dialog, TextView};
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
    status: Option<UwsgiStatus>,
}

lazy_static! {
    pub static ref APP: Mutex<Ui> = Mutex::new(Ui {
        reader: None,
        status: None
    });
}

impl Ui {
    pub fn run(reader: StatsReader) {
        APP.lock().unwrap().reader = Some(reader);

        let mut siv = cursive::default();

        let table = create_table().on_submit(|s, row, _index| Ui::show_detail(s, row));

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

        Ui::read();

        let rows: Vec<UwsgiTableRow> = APP
            .lock()
            .unwrap()
            .status
            .as_mut()
            .unwrap()
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

    fn read() {
        let mut app = APP.lock().unwrap();
        let status = app.reader.as_mut().unwrap().read();
        app.status = Some(status);
    }

    fn show_detail(s: &mut Cursive, row: usize) {
        let app = APP.lock().unwrap();
        let status = app.status.as_ref().unwrap();
        s.add_layer(
            Dialog::around(TextView::new(status.workers[row].get_uri())).button("OK", |s| {
                s.pop_layer();
            }),
        )
    }
}
