use cursive::direction::Orientation::Vertical;
use cursive::theme::{ColorStyle};
use cursive::views::{Dialog, LinearLayout, SelectView, TextView};
use cursive::{Cursive, Vec2};
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

        siv.add_fullscreen_layer(
            LinearLayout::vertical()
                .child(Dialog::around(
                    LinearLayout::new(Vertical)
                        .child(table.with_name("table").full_screen())
                        .full_screen(),
                ))
                .child(
                    TextView::new("UwsgiTop - R = Refresh, Select = Show request detail")
                        .style(ColorStyle::highlight_inactive())
                        .fixed_height(1)
                        .full_width(),
                ),
        );

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
        let value = TextView::new("value").with_name("value");

        let mut select: SelectView<String> =
            SelectView::new().on_select(|s: &mut Cursive, v: &String| {
                let mut value_view = s.find_name::<TextView>("value").expect("Should find value");
                value_view.set_content(v)
            });

        let worker = &status.workers[row];

        select.add_item("Url", worker.get_uri());
        select.add_item("Method", worker.get_core().get_var("HTTP_METHOD"));
        select.add_item("Body", "".to_string());

        s.add_layer(
            Dialog::around(
                LinearLayout::new(Vertical)
                    .child(select)
                    .child(value)
                    .min_size(Vec2::new(50, 50)),
            )
            .button("OK", |s| {
                s.pop_layer();
            }),
        )
    }
}
