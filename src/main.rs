use std::sync::Arc;
use wols::{Config, Docker, DockerResult};

#[tokio::main]
async fn main() -> Result<(), bollard::errors::Error> {
    println!("create a test container");
    let (docker_task, create_result) = create_container().await?;

    let () = tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("stopping container {:?}", create_result.container_id);

    stop_container(
        docker_task,
        create_result.container_id.expect("missing container id"),
    )
    .await?;

    Ok(())
}

async fn create_container() -> Result<(Docker, DockerResult), bollard::errors::Error> {
    let connection = Arc::new(
        bollard::Docker::connect_with_unix_defaults().expect("unable to connect to docker client"),
    );

    let config = Arc::new(Config::new(
        "test-container-1".to_owned(),
        "postgres".to_owned(),
        vec![
            "POSTGRES_USER=postgres".into(),
            "POSTGRES_PASSWORD=password".into(),
        ],
    ));

    let docker = Docker::new(connection, config.clone());

    let result = docker.run().await;
    match result {
        Ok(res) => {
            println!(
                "Container {:?} is running with config {:?}",
                res.container_id, &config
            );
            Ok((docker, res))
        }
        Err(e) => {
            eprintln!("{e}");
            Err(e)
        }
    }
}

async fn stop_container(
    docker: Docker,
    id: String,
) -> Result<DockerResult, bollard::errors::Error> {
    let result = docker.stop(id.clone()).await;
    match result {
        Ok(res) => {
            println!("Container {} has been stopped and removed", &id);
            Ok(res)
        }
        Err(e) => {
            eprintln!("{e}");
            Err(e)
        }
    }
}
