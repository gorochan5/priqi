use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use sqlx::Row;
use sqlx::sqlite::{
    SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions, SqliteSynchronous,
};

use crate::core::datasets::models::TableData;
use crate::core::templates::models::Template;

#[derive(Debug, Clone)]
pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    pub async fn init() -> Result<Self, anyhow::Error> {
        let db_path = get_database_path();
        let options = SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .busy_timeout(Duration::from_secs(5))
            .pragma("cache_size", "-512")
            .statement_cache_capacity(16);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;

        // 1. Tabela de templates
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS templates (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                zpl TEXT NOT NULL,
                width_mm REAL NOT NULL DEFAULT 100.0,
                height_mm REAL NOT NULL DEFAULT 150.0,
                dpi INTEGER NOT NULL DEFAULT 203,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .execute(&pool)
        .await?;

        let user_version: i64 = sqlx::query("PRAGMA user_version")
            .fetch_one(&pool)
            .await?
            .get(0);

        if user_version < 1 {
            // Migração idempotente para bancos priqi.db existentes
            for stmt in [
                "ALTER TABLE templates ADD COLUMN width_mm REAL NOT NULL DEFAULT 100.0",
                "ALTER TABLE templates ADD COLUMN height_mm REAL NOT NULL DEFAULT 150.0",
                "ALTER TABLE templates ADD COLUMN dpi INTEGER NOT NULL DEFAULT 203",
            ] {
                if let Err(err) = sqlx::query(stmt).execute(&pool).await {
                    let msg = err.to_string();
                    if !msg.contains("duplicate column name") {
                        return Err(err.into());
                    }
                }
            }

            sqlx::query("PRAGMA user_version = 1")
                .execute(&pool)
                .await?;
        }

        // 2. Tabela de datasets
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS datasets (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                headers TEXT NOT NULL,
                rows TEXT NOT NULL,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn load_templates(&self) -> Result<Vec<Template>, anyhow::Error> {
        let rows = sqlx::query(
            "SELECT id, name, zpl, width_mm, height_mm, dpi FROM templates ORDER BY created_at ASC",
        )
        .fetch_all(&self.pool)
        .await?;

        let templates = rows
            .into_iter()
            .map(|r| {
                let id: String = r.get("id");
                let name: String = r.get("name");
                let zpl: String = r.get("zpl");
                let width_mm: f64 = r.get("width_mm");
                let height_mm: f64 = r.get("height_mm");
                let dpi: i64 = r.get("dpi");

                let mut tpl = Template::new(id, name, zpl);
                tpl.width_mm = width_mm as f32;
                tpl.height_mm = height_mm as f32;
                tpl.dpi = dpi as u32;
                tpl
            })
            .collect();

        Ok(templates)
    }

    pub async fn save_template(&self, template: &Template) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            INSERT INTO templates (id, name, zpl, width_mm, height_mm, dpi, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, CURRENT_TIMESTAMP)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                zpl = excluded.zpl,
                width_mm = excluded.width_mm,
                height_mm = excluded.height_mm,
                dpi = excluded.dpi,
                updated_at = CURRENT_TIMESTAMP;
            "#,
        )
        .bind(&template.id)
        .bind(&template.name)
        .bind(&template.zpl)
        .bind(template.width_mm as f64)
        .bind(template.height_mm as f64)
        .bind(template.dpi as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_template(&self, id: &str) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM templates WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn load_dataset_metadata(&self) -> Result<Vec<(String, String)>, anyhow::Error> {
        let rows = sqlx::query("SELECT id, name FROM datasets ORDER BY created_at ASC")
            .fetch_all(&self.pool)
            .await?;

        let metadata = rows
            .into_iter()
            .map(|r| {
                let id: String = r.get("id");
                let name: String = r.get("name");
                (id, name)
            })
            .collect();

        Ok(metadata)
    }

    pub async fn load_dataset_table(
        &self,
        id: &str,
    ) -> Result<Option<Arc<TableData>>, anyhow::Error> {
        let row = sqlx::query("SELECT headers, rows FROM datasets WHERE id = ?1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let headers_csv: String = row.get("headers");
        let rows_csv: String = row.get("rows");

        let table = tokio::task::spawn_blocking(move || {
            csv_strings_to_table(&headers_csv, &rows_csv)
        })
        .await??;

        Ok(Some(Arc::new(table)))
    }

    pub async fn save_dataset(
        &self,
        id: &str,
        name: &str,
        table: Arc<TableData>,
    ) -> Result<(), anyhow::Error> {
        let (headers_csv, rows_csv) =
            tokio::task::spawn_blocking(move || table_to_csv_strings(&table)).await??;

        sqlx::query(
            r#"
            INSERT INTO datasets (id, name, headers, rows, updated_at)
            VALUES (?1, ?2, ?3, ?4, CURRENT_TIMESTAMP)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                headers = excluded.headers,
                rows = excluded.rows,
                updated_at = CURRENT_TIMESTAMP;
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(&headers_csv)
        .bind(&rows_csv)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn rename_dataset(&self, id: &str, name: &str) -> Result<(), anyhow::Error> {
        sqlx::query(
            "UPDATE datasets SET name = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
        )
        .bind(name)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_dataset(&self, id: &str) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM datasets WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn table_to_csv_strings(table: &TableData) -> Result<(String, String), anyhow::Error> {
    let mut w_head = csv::WriterBuilder::new().flexible(true).from_writer(vec![]);
    w_head.write_record(&table.headers)?;
    let headers_csv = String::from_utf8(w_head.into_inner()?)?;

    let mut w_rows = csv::WriterBuilder::new().flexible(true).from_writer(vec![]);
    for row in &table.rows {
        w_rows.write_record(row)?;
    }
    let rows_csv = String::from_utf8(w_rows.into_inner()?)?;

    Ok((headers_csv, rows_csv))
}

fn csv_strings_to_table(headers_csv: &str, rows_csv: &str) -> Result<TableData, anyhow::Error> {
    let mut r_head = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(headers_csv.as_bytes());

    let headers = r_head
        .records()
        .next()
        .transpose()?
        .map(|rec| rec.iter().map(String::from).collect())
        .unwrap_or_default();

    let mut r_rows = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(rows_csv.as_bytes());

    let mut rows = Vec::new();
    for rec in r_rows.records() {
        let rec = rec?;
        rows.push(rec.iter().map(String::from).collect());
    }

    Ok(TableData { headers, rows })
}

fn get_database_path() -> PathBuf {
    PathBuf::from("priqi.db")
}

#[cfg(test)]
mod tests {
    use super::{csv_strings_to_table, table_to_csv_strings};
    use crate::core::datasets::models::TableData;

    #[test]
    fn csv_round_trip_uniform_and_quoted_values() {
        let table = TableData {
            headers: vec!["NOME".to_string(), "OBS".to_string()],
            rows: vec![
                vec!["Item A".to_string(), "Com, vírgula".to_string()],
                vec!["Item \"B\"".to_string(), "Linha 2".to_string()],
            ],
        };

        let (h_csv, r_csv) = table_to_csv_strings(&table).unwrap();
        let decoded = csv_strings_to_table(&h_csv, &r_csv).unwrap();
        assert_eq!(decoded, table);
    }

    #[test]
    fn csv_round_trip_ragged_rows_does_not_fail() {
        let table = TableData {
            headers: vec!["A".to_string(), "B".to_string()],
            rows: vec![
                vec!["1".to_string()],
                vec!["1".to_string(), "2".to_string(), "3".to_string()],
            ],
        };

        let (h_csv, r_csv) = table_to_csv_strings(&table).unwrap();
        let decoded = csv_strings_to_table(&h_csv, &r_csv).unwrap();
        assert_eq!(decoded, table);
    }
}
