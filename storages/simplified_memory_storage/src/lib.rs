#![deny(clippy::str_to_string)]

use {
    async_trait::async_trait,
    futures::stream::iter,
    gluesql_core::{
        data::{Key, Schema}, error::Result, store::{
            AlterTable, CustomFunction, CustomFunctionMut, DataRow, Index, IndexMut, Metadata,
            RowIter, Store, StoreMut, Transaction,
        }
    },
    serde::{Deserialize, Serialize},
    std::collections::{BTreeMap, HashMap},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub schema: Schema,
    pub rows: BTreeMap<Key, DataRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SimplifiedMemoryStorage {
    pub id_counter: i64,
    pub items: HashMap<String, Item>,
}

impl SimplifiedMemoryStorage {
    pub fn scan_data(&self, table_name: &str) -> Vec<(Key, DataRow)> {
        match self.items.get(table_name) {
            Some(item) => item.rows.clone().into_iter().collect(),
            None => vec![],
        }
    }
}

#[async_trait(?Send)]
impl Store for SimplifiedMemoryStorage {
    async fn fetch_all_schemas(&self) -> Result<Vec<Schema>> {
        let mut schemas = self
            .items
            .values()
            .map(|item| item.schema.clone())
            .collect::<Vec<_>>();
        schemas.sort_by(|a, b| a.table_name.cmp(&b.table_name));

        Ok(schemas)
    }
    async fn fetch_schema(&self, table_name: &str) -> Result<Option<Schema>> {
        self.items
            .get(table_name)
            .map(|item| Ok(item.schema.clone()))
            .transpose()
    }

    async fn fetch_data(&self, table_name: &str, key: &Key) -> Result<Option<DataRow>> {
        let row = self
            .items
            .get(table_name)
            .and_then(|item| item.rows.get(key).cloned());

        Ok(row)
    }

    async fn scan_data(&self, table_name: &str) -> Result<RowIter> {
        let rows = SimplifiedMemoryStorage::scan_data(self, table_name)
            .into_iter()
            .map(Ok);

        Ok(Box::pin(iter(rows)))
    }
}

#[async_trait(?Send)]
impl StoreMut for SimplifiedMemoryStorage {
    async fn insert_schema(&mut self, schema: &Schema) -> Result<()> {
        let table_name = schema.table_name.clone();
        
        let item = Item {
            schema: schema.clone(),
            rows: BTreeMap::new(),
        };
        self.items.insert(table_name, item);

        Ok(())
    }

    async fn delete_schema(&mut self, table_name: &str) -> Result<()> {
        self.items.remove(table_name);
        Ok(())
    }

    async fn append_data(&mut self, table_name: &str, rows: Vec<DataRow>) -> Result<()> {
        // Delegate the task to insert_data, we need to transform Vec<DataRow> into Vec<(Key, DataRow)>
        let mut transformed: Vec<(Key, DataRow)> = Vec::new();
        for row in rows {
            self.id_counter += 1;
            transformed.push((Key::I64(self.id_counter), row));
        }

        return self.insert_data(table_name, transformed).await;
    }

    async fn insert_data(&mut self, table_name: &str, rows: Vec<(Key, DataRow)>) -> Result<()> {
        if let Some(item) = self.items.get_mut(table_name) {
            for (key, row) in rows {
                item.rows.insert(key, row);
            }
        }

        Ok(())
    }

    async fn delete_data(&mut self, table_name: &str, keys: Vec<Key>) -> Result<()> {
        if let Some(item) = self.items.get_mut(table_name) {
            for key in keys {
                item.rows.remove(&key);
            }
        }

        Ok(())
    }
}

impl AlterTable for SimplifiedMemoryStorage {}
impl Index for SimplifiedMemoryStorage {}
impl IndexMut for SimplifiedMemoryStorage {}
impl Transaction for SimplifiedMemoryStorage {}
impl Metadata for SimplifiedMemoryStorage {}
impl CustomFunction for SimplifiedMemoryStorage {}
impl CustomFunctionMut for SimplifiedMemoryStorage {}
