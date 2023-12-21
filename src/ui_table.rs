use std::cmp::Ordering;
use cursive::align::HAlign;
use cursive_table_view::{TableView, TableViewItem};
use crate::uwsgi_struct::Worker;

pub type UiTableView = TableView::<Foo, BasicColumn>;

// Provide a type for the table's columns
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BasicColumn {
    Name,
    Count,
    Rate
}

// Define the item type
#[derive(Clone, Debug)]
pub struct Foo {
    name: String,
    count: usize,
    rate: usize
}

impl Foo {
    pub fn new_from_worker(worker: &Worker) -> Foo {
        Foo {
            name: worker.status.clone(),
            count: worker.id as usize,
            rate: worker.id as usize,
        }
    }
}

impl TableViewItem<BasicColumn> for Foo {

    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::Count => format!("{}", self.count),
            BasicColumn::Rate => format!("{}", self.rate)
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering where Self: Sized {
        match column {
            BasicColumn::Name => self.name.cmp(&other.name),
            BasicColumn::Count => self.count.cmp(&other.count),
            BasicColumn::Rate => self.rate.cmp(&other.rate)
        }
    }

}

pub fn create_table() -> TableView::<Foo, BasicColumn> {
    // Configure the actual table
    TableView::<Foo, BasicColumn>::new()
        .column(BasicColumn::Name, "Name", |c| c.width(20))
        .column(BasicColumn::Count, "Count", |c| c.align(HAlign::Center))
        .column(BasicColumn::Rate, "Rate", |c| {
            c.ordering(Ordering::Greater).align(HAlign::Right).width(20)
        })
        .default_column(BasicColumn::Name)
}
