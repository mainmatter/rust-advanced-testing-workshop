use anyhow::Context;
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::models::{HealthConfig, HealthStatusEnum, HostConfig, PortBinding, PortMap};
use bollard::Docker;
use std::collections::HashMap;

/// Launch a named Postgres 14 container.
/// If a container with the same name already exists, do nothing.
///
/// The function waits for the container to become healthy before returning.
/// If it doesn't, it returns an error.
pub async fn launch_postgres_container(
    cli: Docker,
    container_name: &str,
) -> Result<(), anyhow::Error> {
    match cli
        .create_container(
            Some(CreateContainerOptions {
                name: container_name,
                ..Default::default()
            }),
            Config {
                image: Some("postgres:14"),
                exposed_ports: Some({
                    let mut ports = HashMap::new();
                    ports.insert("5432/tcp", HashMap::new());
                    ports
                }),
                host_config: Some(HostConfig {
                    port_bindings: Some({
                        let mut m = PortMap::new();
                        let v = vec![PortBinding {
                            host_ip: None,
                            host_port: Some("5432".to_string()),
                        }];
                        m.insert("5432/tcp".to_string(), Some(v));
                        m
                    }),
                    ..Default::default()
                }),
                healthcheck: Some(HealthConfig {
                    test: Some(vec!["CMD".to_string(), "pg_isready".to_string()]),
                    interval: Some(10000000000),
                    timeout: Some(5000000000),
                    retries: Some(5),
                    start_period: Some(1000000000),
                }),
                env: Some(vec![
                    "POSTGRES_PASSWORD=password",
                    "POSTGRES_USER=postgres",
                    "POSTGRES_DB=postgres",
                ]),
                ..Default::default()
            },
        )
        .await
    {
        Err(e) => {
            if let bollard::errors::Error::DockerResponseServerError {
                status_code: 409, ..
            } = e
            {
                println!("Container named {container_name} already exists, nothing to do.");
            } else {
                return Err(e.into());
            }
        }
        Ok(_) => {
            println!("Container created.");
            cli.start_container(container_name, None::<StartContainerOptions<String>>)
                .await
                .context("Failed to start container")?;
            println!("Container started.");
        }
    };

    let n_retries = 5;
    let wait_interval = std::time::Duration::from_secs(5);

    for _ in 0..n_retries {
        let c = cli.inspect_container(container_name, None).await.unwrap();
        let container_state = c.state.context("Missing container state field")?;
        let container_health = container_state
            .health
            .context("Missing container health field")?;
        let health_status = container_health
            .status
            .context("Missing health status field")?;
        match health_status {
            HealthStatusEnum::EMPTY
            | HealthStatusEnum::NONE
            | HealthStatusEnum::UNHEALTHY
            | HealthStatusEnum::STARTING => {
                println!("Container is not healthy, waiting...");
                tokio::time::sleep(wait_interval).await;
            }
            HealthStatusEnum::HEALTHY => {
                println!("Container is healthy, ready to use");
                return Ok(());
            }
        }
    }
    anyhow::bail!("Container failed to become healthy");
}
