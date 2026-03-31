use testcontainers::{
    Container,
    GenericImage,
    GenericBuildableImage,
    ImageExt,
    core::{
        IntoContainerPort,
        WaitFor,
        ExecCommand,
    },
    runners::{
        SyncRunner,
        SyncBuilder,
    },
};

pub struct SqlServerContainer(Container<GenericImage>);

impl SqlServerContainer {
    pub fn create_open_event(&self) {
        self.0.exec(ExecCommand::new(vec!["dolt", "sql", "-q", include_str!("create_open_event.sql")]))
            .expect("Failed to create open event");
    }
}

pub struct TestSuite {
    pub sql_server: SqlServerContainer,
    rocket: Container<GenericImage>,
}

impl TestSuite {
    pub fn spawn() -> Self {
        let sql_server = GenericImage::new("localhost/herald/dolt-sql-server", "latest")
            .with_wait_for(WaitFor::message_on_stdout("Ready for connections."))
            .with_network("herald")
            .with_env_var("DOLT_ROOT_HOST", "%")
            .start()
            .expect("Failed to start dolt sql-server");
        let mut ip = String::new();
        sql_server.exec(ExecCommand::new(["hostname", "-I"]))
            .expect("freshly spawned container should accept exec")
            .stdout()
            .read_to_string(&mut ip)
            .expect("command buffer should be parseable");
        let rocket = GenericBuildableImage::new("localhost/herald/rocket-test", "latest")
            .with_dockerfile_string(
                r#"FROM debian:stable-slim
                COPY ./herald /usr/local/bin/
                CMD ["/usr/local/bin/herald"]"#
            )
            .with_file(env!("CARGO_BIN_EXE_herald"), "./herald")
            .build_image()
            .expect("Failed to build test image")
            .with_exposed_port(8000.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Rocket has launched from http://0.0.0.0:8000"))
            .with_network("herald")
            .with_env_var("DATABASE_URL", format!("mysql://root@{}:3306/herald", ip.trim()))
            .with_env_var("ROCKET_ADDRESS", "0.0.0.0")
            .start()
            .expect("Failed to start rocket");
        TestSuite {
            sql_server: SqlServerContainer(sql_server),
            rocket: rocket,
        }
    }

    pub fn path(&self, path: &str) -> String {
        format!("http://localhost:{}{}", self.port(), path)
    }

    fn port(&self) -> u16 {
        self.rocket.get_host_port_ipv4(8000).expect("test suite rocket runner should expose port 8000")
    }
}
