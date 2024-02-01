use std::path::Path;

use datafusion::{execution::{context::SessionContext, options::CsvReadOptions}, scalar::ScalarValue};
#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    read("./sample.csv").await
}

pub async fn read(file_path: impl AsRef<Path>) -> datafusion::error::Result<()> {
    let file_path = file_path.as_ref().as_os_str().to_str().unwrap();
    // REGISTER THE TABLE
    let ctx = SessionContext::new();
    let options = CsvReadOptions::new().has_header(true);
    ctx.register_csv("my_table", file_path, options).await.unwrap();
    // CREATE A PLAN TO RUN A SQL QUERY
    let data_frame = ctx
        .sql("SELECT value FROM my_table")
        .await
        .unwrap()
        .with_param_values(vec![
            // ScalarValue::Utf8(Some(String::from("todo"))),
            ScalarValue::Utf8(Some(String::from("águila"))),
        ])
        .unwrap();
    // PRINT
    data_frame.show().await.unwrap();
    Ok(())
}