use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the SurrealDB server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Sign in to the SurrealDB server
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    // Select the specific namespace and database
    db.use_ns("test").use_db("test").await?;

    // Execute an SQL-like query to create a table
    let query = r#"
        CREATE TABLE person (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255),
            age INT
        )
    "#;
    // db.export(query).await?;
    // db.execute(query).await?;
    db.query(query).await?;
    let query = r#"
    INSERT INTO person (name, age)
    VALUES ('John Doe', 30)
"#;
        db.query(query).await?;


    println!("Table 'person' created successfully");

    Ok(())
}
