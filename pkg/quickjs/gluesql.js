import init, { Glue } from './dist_web/gluesql_quickjs.js';

let loaded = false;

async function load(module_or_path) {
  await init(module_or_path);

  loaded = true;
}

export async function gluesql(module_or_path) {
  if (!loaded) {
    await load(module_or_path);
  }

  return new Glue();
}

window.log2Console = function(log_string) {
  console.log(log_string);
}

window.fetch_schema = async function(table_name) {

  return "mother fucker " + table_name;
}