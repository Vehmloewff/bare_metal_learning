use agincourt_runtime_ops::DatabaseResponseItem;
use libsql::Database;

pub struct DatabaseResource {
	database: Database,
	responses: Vec<DatabaseResponseItem>,
}
