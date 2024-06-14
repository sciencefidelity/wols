use crate::PortSet;
use bollard::container::{self, CreateContainerOptions, LogsOptions};
use bollard::container::{RemoveContainerOptions, StartContainerOptions, StopContainerOptions};
use bollard::secret::ContainerCreateResponse;
use std::{collections::HashMap, io, sync::Arc};

#[derive(Debug)]
/// Encapsulates all necessary information about a task's configuration
pub struct Config {
    /// Name of the running container and identifier in orchestration system
    name: String,
    attach_stdin: bool,
    attach_stdout: bool,
    attach_stderr: bool,
    exposed_ports: PortSet,
    cmd: Vec<String>,
    /// Name of the image running in the container
    image: String,
    /// Used by the scheduler to find a node in the cluster capable of running a task
    /// Used to tell the Docker daemon the number of resources a task requires
    cpu: Option<u64>,
    memory: Option<u64>,
    disk: Option<u64>,
    /// Environment variables to be passes into the container
    env: Vec<String>,
    /// Tells the Docker daemon what to do if the container dies unexpectedly
    restart_policy: Option<String>,
}

impl Config {
    #[must_use]
    pub fn new(name: String, image: String, env: Vec<String>) -> Self {
        Self {
            name,
            attach_stdin: false,
            attach_stdout: false,
            attach_stderr: false,
            exposed_ports: HashMap::new(),
            cmd: Vec::new(),
            image,
            cpu: None,
            memory: None,
            disk: None,
            env,
            restart_policy: None,
        }
    }
}

pub struct Client;

pub struct Docker {
    connection: Arc<bollard::Docker>,
    config: Arc<Config>,
}

impl Docker {
    /// Create a new Docker instance.
    #[must_use]
    pub fn new(connection: Arc<bollard::Docker>, config: Arc<Config>) -> Self {
        Self { connection, config }
    }

    /// Run a container.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the container fails to initialize.
    pub async fn run(&self) -> Result<DockerResult, bollard::errors::Error> {
        let options = Some(CreateContainerOptions {
            name: &self.config.name,
            platform: None,
        });

        let config = container::Config {
            image: Some(self.config.image.clone()),
            tty: Some(false),
            env: Some(self.config.env.clone()),
            exposed_ports: Some(self.config.exposed_ports.clone()),
            ..Default::default()
        };

        let container = self.connection.create_container(options, config);
        let container_id = match container.await {
            Ok(ContainerCreateResponse { id, .. }) => id,
            Err(e) => {
                eprintln!(
                    "error starting container using image {}: {e}",
                    &self.config.image
                );
                return Err(e);
            }
        };

        if let Err(e) = self
            .connection
            .start_container(&self.config.name, None::<StartContainerOptions<String>>)
            .await
        {
            eprintln!("error starting container {}: {e}", &self.config.image);
            return Err(e);
        };

        let options = Some(LogsOptions::<String> {
            stdout: true,
            ..Default::default()
        });

        // TODO: write logs to stdout on new thread
        self.connection.logs(&self.config.name, options);

        Ok(DockerResult {
            action: "start".to_owned(),
            container_id: Some(container_id),
            result: "success".to_owned(),
        })
    }

    /// Stop a running container
    ///
    /// # Errors
    ///
    /// Will return `Err` if the container fails to stop.
    pub async fn stop(&self, id: String) -> Result<DockerResult, bollard::errors::Error> {
        println!("attempting to stop container {id}");

        let options = Some(StopContainerOptions { t: 30 });

        if let Err(e) = self
            .connection
            .stop_container(&self.config.name, options)
            .await
        {
            println!("error stopping container {}: {e}", &self.config.name);
            return Err(e);
        };

        let options = Some(RemoveContainerOptions {
            v: true,
            force: false,
            link: false,
        });

        if let Err(e) = self
            .connection
            .remove_container(&self.config.name, options)
            .await
        {
            println!("error removeing container {}: {e}", &self.config.name);
            return Err(e);
        };

        Ok(DockerResult {
            action: "stop".into(),
            result: "success".into(),
            container_id: None,
        })
    }
}

#[derive(Debug)]
pub struct DockerResult {
    action: String,
    pub container_id: Option<String>,
    result: String,
}
