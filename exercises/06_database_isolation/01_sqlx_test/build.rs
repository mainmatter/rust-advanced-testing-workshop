use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!(); // defaults to "./migrations"

#[tokio::main]
async fn main() {
    // Load .env file if available.
    dotenvy::dotenv().ok();

    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(&db_url).await.unwrap();
    MIGRATOR.run(&pool).await.unwrap();
}
