pub mod candidate_repository;
pub mod pledge_repository;

use sqlx::postgres::PgPoolOptions;
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions,
    StartContainerOptions,
};
use bollard::image::CreateImageOptions;
use bollard::Docker;
use futures::stream::StreamExt;

pub use candidate_repository::CandidateRepository;
pub use pledge_repository::PledgeRepository;

pub struct DatabaseManager {
    container_name: String,
    docker: Docker,
}

impl DatabaseManager {
    pub fn new(container_name: &str) -> Self {
        Self {
            container_name: container_name.to_string(),
            docker: Docker::connect_with_local_defaults().expect("Failed to connect to Docker"),
        }
    }

    pub async fn start_database(&self) -> Result<(), String> {
        let image = "postgres:15-alpine";

        self.docker
            .create_image(
                Some(CreateImageOptions {
                    from_image: image,
                    ..Default::default()
                }),
                None,
                None,
            )
            .for_each(|_| async {})
            .await;

        let config = Config {
            image: Some(image),
            env: Some(vec![
                "POSTGRES_USER=vote_ray",
                "POSTGRES_PASSWORD=your_password",
                "POSTGRES_DB=vote_ray",
            ]),
            host_config: Some(bollard::service::HostConfig {
                port_bindings: Some(
                    vec![(
                        "5432/tcp".to_string(),
                        Some(vec![bollard::service::PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
                            host_port: Some("5432".to_string()),
                        }]),
                    )]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            }),
            ..Default::default()
        };

        let options = Some(CreateContainerOptions {
            name: &self.container_name,
            platform: None,
        });

        self.docker
            .create_container(options, config)
            .await
            .map_err(|e| e.to_string())?;

        self.docker
            .start_container(&self.container_name, None::<StartContainerOptions<String>>)
            .await
            .map_err(|e| e.to_string())?;

        let init_sql = r#"
            CREATE TABLE IF NOT EXISTS candidates (
                id UUID PRIMARY KEY,
                name VARCHAR(255) NOT NULL
            );

            CREATE TABLE IF NOT EXISTS pledges (
                id UUID PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                description TEXT NOT NULL
            );
        "#;

        use bollard::exec::{CreateExecOptions, StartExecResults};

        let exec = self
            .docker
            .create_exec(
                &self.container_name,
                CreateExecOptions {
                    cmd: Some(vec!["psql", "-U", "vote_ray", "-d", "vote_ray", "-c", init_sql]),
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| e.to_string())?;

        if let StartExecResults::Attached { mut output, .. } = self.docker.start_exec(&exec.id, None).await.map_err(|e| e.to_string())? {
            while let Some(Ok(_)) = output.next().await {}
        }

        Ok(())
    }

    pub async fn cleanup_database(&self) -> Result<(), String> {
        let options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });

        self.docker
            .remove_container(&self.container_name, options)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

pub async fn create_pool(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
