use std::cmp::Ordering;
use cursive::align::HAlign;
use cursive_table_view::{TableView, TableViewItem};
use crate::uwsgi_struct::Worker;

pub type UiTableView = TableView::<UwsgiTableRow, BasicColumn>;

// Provide a type for the table's columns
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BasicColumn {
    Index,
    Status,
    Uri,
    Duration,
}

// Define the item type
#[derive(Clone, Debug)]
pub struct UwsgiTableRow {
    index: usize,
    status: String,
    uri: String,
    duration: usize,
}

impl UwsgiTableRow {
    pub fn new_from_worker(worker: &mut Worker) -> UwsgiTableRow {
        UwsgiTableRow {
            index: worker.id as usize,
            uri: worker.get_uri().clone(),
            status: worker.status.clone(),
            duration: worker.id as usize,
        }
    }
}

impl TableViewItem<BasicColumn> for UwsgiTableRow {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Index => format!("{}", self.index),
            BasicColumn::Status => self.status.clone(),
            BasicColumn::Uri => self.uri.clone(),
            BasicColumn::Duration => format!("{}", self.duration),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering where Self: Sized {
        match column {
            BasicColumn::Index => self.index.cmp(&other.index),
            BasicColumn::Status => self.status.cmp(&other.status),
            BasicColumn::Uri => self.uri.cmp(&other.uri),
            BasicColumn::Duration => self.duration.cmp(&other.duration),
        }
    }

}

pub fn create_table() -> TableView::<UwsgiTableRow, BasicColumn> {
    // Configure the actual table
    TableView::<UwsgiTableRow, BasicColumn>::new()
        .column(BasicColumn::Index, "Worker", |c| {
            c.align(HAlign::Right).width(2)
        })
        .column(BasicColumn::Status, "Status", |c| c.width(5))
        .column(BasicColumn::Duration, "Duration", |c| c.width(5).align(HAlign::Center))
        .column(BasicColumn::Uri, "Uri", |c| c)
        .default_column(BasicColumn::Index)
}
