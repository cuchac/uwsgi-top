use crate::uwsgi_struct::Worker;
use cursive::direction::Orientation::Vertical;
use cursive::event::{Key};
use cursive::views::{Dialog, DummyView, LinearLayout, OnEventView, TextArea};
use cursive::{Cursive};
use cursive_table_view::{TableView, TableViewItem};
use std::cmp::Ordering;

use cursive::traits::*;

type Table = TableView<DetailItem, DetailColumn>;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum DetailColumn {
    Name,
    Value,
}

#[derive(Clone, Debug)]
pub struct DetailItem {
    name: String,
    value: String,
}

impl DetailItem {
    pub fn open_detail_for_worker(s: &mut Cursive, worker: &Worker) {
        let mut table = Table::new()
            .column(DetailColumn::Name, "Variable", |c| c.width(30))
            .column(DetailColumn::Value, "Value", |c| c)
            .default_column(DetailColumn::Name)
            .on_select(|s: &mut Cursive, _row, index| {
                let table = s
                    .find_name::<Table>("detail_table")
                    .expect("Should find detail table");
                let mut value = s
                    .find_name::<TextArea>("value")
                    .expect("Should find value widget");

                let item = table.borrow_item(index).unwrap();
                value.set_content(item.value.clone());
            });

        table.set_items(
            worker
                .cores
                .first()
                .unwrap()
                .vars
                .iter()
                .map(|worker| DetailItem {
                    name: worker.0.to_string(),
                    value: worker.1.clone(),
                })
                .collect(),
        );

        let mut value = TextArea::new();
        value.disable();

        let dialog = Dialog::around(
            LinearLayout::new(Vertical)
                .child(table.with_name("detail_table").full_screen())
                .child(DummyView)
                .child(value.with_name("value").fixed_height(2)),
        )
        .button("OK", |s| {
            s.pop_layer();
        })
        .full_screen();

        s.add_layer(OnEventView::new(dialog).on_event(Key::Esc, |s| {
            s.pop_layer();
        }))
    }
}

impl TableViewItem<DetailColumn> for DetailItem {
    fn to_column(&self, column: DetailColumn) -> String {
        match column {
            DetailColumn::Name => self.name.clone(),
            DetailColumn::Value => self.value.clone(),
        }
    }

    fn cmp(&self, other: &Self, column: DetailColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            DetailColumn::Name => self.name.cmp(&other.name),
            DetailColumn::Value => self.value.cmp(&other.value),
        }
    }
}
