use bollard::Docker;
use db_launcher::launch_postgres_container;

#[tokio::main]
async fn main() {
    let container_name = "test_db";
    let cli = Docker::connect_with_local_defaults().expect("Failed to connect to Docker");
    if let Err(e) = launch_postgres_container(cli, container_name).await {
        eprintln!("Failed to launch Postgres container: {:#}", e);
        std::process::exit(1);
    }
}
