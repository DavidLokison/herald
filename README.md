# Herald
This is the REST-API application layer of the historischer-besiedlungszug.de website written in Rust using the [Rocket](https://rocket.rs/) crate. Its purpose is to take HTTP(S) requests from the webpage in JSON format and to trigger the respective backend handles on the database. The API shall then return an appropriate response code, potentially also a JSON body with the requested data.

## Guidelines
- This project is built security-first. No communication with other services shall be logged in production mode whatsoever.
- Encryption and other concerns are bound to change. It is advised to define and call a small amount of wrapper functions for communicating with other services.

## Dependencies
To build and test the project you need a [Docker](https://www.docker.com) compatible runtime and a compose wrapper. To build the project, all you need to do is firing up the shipped `compose.yaml` which sets up the [Herald SQL Server](https://github.com/besiedlungszug/herald-sql-server) image the application is targeted against. The integration tests use the `DOCKER_HOST` environment variable to connect to a pipe. If setting up a test environment using [Podman](https://podman.io) make sure to set the environment variable accordingly and create the `herald` bridge network beforehand.

If you also want to contribute to the [Herald Data Project](https://www.dolthub.com/repositories/besiedlungszug/herald), you need to install [Dolt](https://www.dolthub.com) and clone the repository. The `.gitignore` file is set up to exclude local dolt related files so that the data project can live in the same directory as the software project.

## Contribution
Contribution is highly welcome, whether by writing Issues or providing code. You acknowledge that all contribution will be published under the same terms and conditions as this main repository.

## Legacy Codebase
The old Python/FastAPI based approach has been moved over to the `legacy-python-fastapi` branch in case we would need any of it again later. The main reason to switch branches was for security and atomicity reasons, cause Rust provides hardened static typing and also compiles to a single binary for ease of deployment.
