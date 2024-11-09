#![cfg(target_arch = "wasm32")]

use {
    async_trait::async_trait, 
    gluesql_core::prelude::Glue, test_suite::*,
    gluesql_quickjs::storage::QuickjsStorage,
    wasm_bindgen_test::*,
   
};

wasm_bindgen_test_configure!(run_in_browser);

struct QuickjsTester {
    glue: Glue<QuickjsStorage>,
}

#[async_trait(?Send)]
impl Tester<QuickjsStorage> for QuickjsTester {
    async fn new(_: &str) -> Self {
        let storage = QuickjsStorage::default();
        let glue = Glue::new(storage);

        Self { glue }
    }

    fn get_glue(&mut self) -> &mut Glue<QuickjsStorage> {
        &mut self.glue
    }
}

generate_store_tests!(wasm_bindgen_test, QuickjsTester);

generate_alter_table_tests!(wasm_bindgen_test, QuickjsTester);

generate_metadata_table_tests!(wasm_bindgen_test, QuickjsTester);

generate_custom_function_tests!(wasm_bindgen_test, QuickjsTester);
