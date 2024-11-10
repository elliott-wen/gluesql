use {
    async_trait::async_trait, gluesql_core::prelude::Glue,
    gluesql_simplified_memory_storage::SimplifiedMemoryStorage, test_suite::*,
};

struct MemoryTester {
    glue: Glue<SimplifiedMemoryStorage>,
}

#[async_trait(?Send)]
impl Tester<SimplifiedMemoryStorage> for MemoryTester {
    async fn new(_: &str) -> Self {
        let storage = SimplifiedMemoryStorage::default();
        let glue = Glue::new(storage);

        MemoryTester { glue }
    }

    fn get_glue(&mut self) -> &mut Glue<SimplifiedMemoryStorage> {
        &mut self.glue
    }
}

generate_store_tests!(tokio::test, MemoryTester);

