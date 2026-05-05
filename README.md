# Herald
This is the REST-API application layer of the historischer-besiedlungszug.de website written in Rust using the [Rocket](https://rocket.rs/) crate. Its purpose is to take HTTP(S) requests from the webpage in JSON format and to trigger the respective backend handles on the database. The API shall then return an appropriate response code, potentially also a JSON body with the requested data.

## Guidelines
- This project is built security-first. No communication with other services shall be logged in production mode whatsoever.
- Encryption and other concerns are bound to change. It is advised to define and call a small amount of wrapper functions for communicating with other services.

## Configuration
- `Rocket.toml`: The service uses Rocket's default configuration file. See its [Configuration Guide](https://rocket.rs/guide/v0.5/configuration/) for details.
- `.env`: SQLx has a feature of compile-time checking queries and resolving to the actual data types. For that to work, the database URL has to be set. The local `.env` provides the defaults for the local environment and really shouldn't be changed unless you know exactly what you are doing.

## Dependencies
To run the test cases you need some kind of [Docker](https://www.docker.com) compatible runtime, the integration tests use the `DOCKER_HOST` environment variable to connect to a pipe. If setting up a test environment using [Podman](https://podman.io) make sure to set the environment variable accordingly and create the `herald` network beforehand.

If you also want to contribute to the [Herald Data Project](https://www.dolthub.com/repositories/besiedlungszug/herald), you need to install [Dolt](https://www.dolthub.com) and clone the repository. The `.gitignore` file is set up to exclude local dolt related files so that the data project can live in the same directory as the software project.

**Notice:** Currently, to add queries to the system, you also need to have a dolt sql server running. This currently can't be done with the Docker subsystem, so you need to install Dolt and the Herald data repository either way. This is bound to change before the first full release version.

## Contribution
Contribution is highly welcome, whether by writing Issues or providing code. You acknowledge that all contribution will be published under the same terms and conditions as this main repository.

## Legacy Codebase
The old Python/FastAPI based approach has been moved over to the `legacy-python-fastapi` branch in case we would need any of it again later. The main reason to switch branches was for security and atomicity reasons, cause Rust provides hardened static typing and also compiles to a single binary for ease of deployment.
