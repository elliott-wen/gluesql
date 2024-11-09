#![deny(clippy::str_to_string)]

use {
    async_trait::async_trait,
    gluesql_core::{
        data::{Key, Schema},
        error::{Error, Result},
        store::{
            AlterTable, CustomFunction, CustomFunctionMut, DataRow, Index, IndexMut, Metadata,
            RowIter, Store, StoreMut, Transaction,
        },
    },
    wasm_bindgen::prelude::*,
};

#[derive(Debug, Clone, Default)]
pub struct QuickjsStorage {
    
}

#[wasm_bindgen]
extern "C" {
    // Import a JavaScript function
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[async_trait(?Send)]
impl Store for QuickjsStorage {
    async fn fetch_all_schemas(&self) -> Result<Vec<Schema>> {
        log("fetch_all_schemas not implemented");
        Err(Error::StorageMsg(
            "[QuickjsStorage] not implemented".to_owned(),
        ))
    }

    async fn fetch_schema(&self, table_name: &str) -> Result<Option<Schema>> {
        log(format!("fetch_schema not implemented: {table_name}").as_str());
        Err(Error::StorageMsg(
            "[QuickjsStorage] not implemented".to_owned(),
        ))
    }

    async fn fetch_data(&self, table_name: &str, _target: &Key) -> Result<Option<DataRow>> {
        log(format!("fetch_data not implemented: {table_name}").as_str());
        Err(Error::StorageMsg(
            "[QuickjsStorage] not implemented".to_owned(),
        ))
    }

    async fn scan_data(&self, table_name: &str) -> Result<RowIter> {
        log(format!("scan_data not implemented: {table_name}").as_str());
        Err(Error::StorageMsg(
            "[QuickjsStorage] not implemented".to_owned(),
        ))
    }
}

#[async_trait(?Send)]
impl StoreMut for QuickjsStorage {
    async fn insert_schema(&mut self, _schema: &Schema) -> Result<()> {
        log("insert_schema not implemented");
        Err(Error::StorageMsg(
            "[QuickjsStorage] not implemented".to_owned(),
        ))
    }

    async fn delete_schema(&mut self, table_name: &str) -> Result<()> {
        log(format!("delete_schema not implemented: {table_name}").as_str());
        Err(Error::StorageMsg(
            "[QuickjsStorage] not implemented".to_owned(),
        ))
    }

    async fn append_data(&mut self, table_name: &str, _new_rows: Vec<DataRow>) -> Result<()> {
        log(format!("append_data not implemented: {table_name}").as_str());
        Err(Error::StorageMsg(
            "[QuickjsStorage] not implemented".to_owned(),
        ))
    }

    async fn insert_data(&mut self, table_name: &str, _new_rows: Vec<(Key, DataRow)>) -> Result<()> {
        log(format!("insert_data not implemented: {table_name}").as_str());
        Err(Error::StorageMsg(
            "[QuickjsStorage] not implemented".to_owned(),
        ))
    }

    async fn delete_data(&mut self, table_name: &str, _keys: Vec<Key>) -> Result<()> {
        log(format!("delete_data not implemented: {table_name}").as_str());
        Err(Error::StorageMsg(
            "[QuickjsStorage] not implemented".to_owned(),
        ))
    }
}


impl AlterTable for QuickjsStorage {}
impl Index for QuickjsStorage {}
impl IndexMut for QuickjsStorage {}
impl Transaction for QuickjsStorage {}
impl Metadata for QuickjsStorage {}
impl CustomFunction for QuickjsStorage {}
impl CustomFunctionMut for QuickjsStorage {}
