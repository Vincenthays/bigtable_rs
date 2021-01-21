use bigtable_rs::bigtable;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let instance_name = "test1";
    let table_name = "my-table";
    let key: String = "key1".to_owned();

    println!("hello_world");
    let connection = bigtable::BigTableConnection::new(instance_name, true, None, 4).await?;
    let mut bigtable = connection.client();
    let row_data = bigtable.get_single_row_data(table_name, key).await?;
    if let Some((cell, value)) = row_data.get(0) {
        println!("{}, {}", cell, String::from_utf8(value.to_vec())?);
    }
    Ok(())
}
