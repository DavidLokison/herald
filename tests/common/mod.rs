#![allow(unused)]

use std::thread;
use std::time::Duration;
use testcontainers::{
    ContainerAsync,
    GenericImage,
    GenericBuildableImage,
    ImageExt,
    core::{
        IntoContainerPort,
        WaitFor,
        ExecCommand,
    },
    runners::{
        AsyncRunner,
        AsyncBuilder,
    },
};
use tokio::io::AsyncReadExt;

pub struct SqlServerContainer(ContainerAsync<GenericImage>);

impl SqlServerContainer {
    pub async fn spawn() -> Self {
        let sql_server = GenericImage::new("besiedlungszug/herald-sql-server", "0.4.1")
            .with_wait_for(WaitFor::message_on_stdout("Ready for connections."))
            .with_exposed_port(3306.tcp())
            .with_env_var("DOLT_ROOT_HOST", "%")
            .start()
            .await
            .unwrap();
        Self(sql_server)
    }

    pub async fn url(&self) -> String {
        format!("mysql://root@localhost:{}/herald", self.port().await)
    }
    
    async fn port(&self) -> u16 {
        self.0.get_host_port_ipv4(3306).await.expect("test suite sql runner should expose port 3306")
    }

    async fn exec(&self, c: ExecCommand) {
        let mut result = self.0.exec(c)
            .await.unwrap();
        assert_eq!(result.exit_code().await.unwrap().unwrap(), 0);
        let mut buf = String::new();
        result.stdout().read_to_string(&mut buf).await;
        result.stderr().read_to_string(&mut buf).await;
        println!("> dolt sql");
        println!("{}", buf);
        println!("<<<");
    }

    pub async fn create_open_event(&self) {
        self.exec(ExecCommand::new(vec!["dolt", "sql", "-q", include_str!("create_open_event.sql")])).await;
    }

    pub async fn create_pseudo_registration(&self) {
        self.exec(ExecCommand::new(vec!["dolt", "sql", "-q", include_str!("create_pseudo_registration.sql")])).await;
    }

    pub async fn create_club_information_stub(&self) {
        self.exec(ExecCommand::new(vec!["dolt", "sql", "-q", include_str!("create_club_information_stub.sql")])).await;
    }

    pub async fn approve_single_registration(&self) {
        self.exec(ExecCommand::new(vec!["dolt", "sql", "-q", include_str!("approve_single_registration.sql")])).await;
    }
}

pub struct TestSuite {
    pub sql_server: SqlServerContainer,
    rocket: ContainerAsync<GenericImage>,
}

impl TestSuite {
    pub async fn spawn() -> Self {
        let sql_server = SqlServerContainer::spawn().await;
        let rocket = GenericBuildableImage::new("localhost/herald/rocket", "latest")
            .with_dockerfile_string(
                r#"FROM debian:stable-slim
                COPY ./herald /usr/local/bin/
                CMD ["/usr/local/bin/herald"]"#
            )
            .with_file(env!("CARGO_BIN_EXE_herald"), "./herald")
            .build_image()
            .await
            .unwrap()
            .with_exposed_port(8000.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Rocket has launched from http://0.0.0.0:8000"))
            .with_env_var("DATABASE_URL", format!("mysql://root@host.containers.internal:{}/herald", sql_server.port().await))
            .with_env_var("ROCKET_ADDRESS", "0.0.0.0")
            .start()
            .await
            .unwrap();
        TestSuite {
            sql_server: sql_server,
            rocket: rocket,
        }
    }

    pub async fn path(&self, path: &str) -> String {
        format!("http://localhost:{}{}", self.port().await, path)
    }

    async fn port(&self) -> u16 {
        self.rocket.get_host_port_ipv4(8000).await.expect("test suite rocket runner should expose port 8000")
    }
}
