# Herald
This is the REST-API application layer of the historischer-besiedlungszug.de website written in Rust using the [Rocket](https://rocket.rs/) crate. Its purpose is to take HTTP(S) requests from the webpage in JSON format and to trigger the respective backend handles on the database. The API shall then return an appropriate response code, potentially also a JSON body with the requested data.

## Guidelines
- This project is built security-first. No communication with other services shall be logged in production mode whatsoever.
- Encryption and other concerns are bound to change. It is advised to define and call a small amount of wrapper functions for communicating with other services.

## Configuration
The service uses Rocket's default configuration file `Rocket.toml`. See its [Configuration Guide](https://rocket.rs/guide/v0.5/configuration/) for details.

## Contribution
Contribution is highly welcome, whether by writing Issues or providing code. The Cargo framework should take care about most of the dependencies, but you need a local copy of the backend database service as well. For that you need to install [Dolt](https://www.dolthub.com/) and clone the publicly hosted database schema from DoltHub, which is called [Herald](https://www.dolthub.com/repositories/besiedlungszug/herald) as well. For the most simple setup, just run `dolt clone besiedlungszug/herald . && dolt clone davidlokison/base32` in the git repository, the `.gitignore` is already set up to exclude any dolt related files.

## Legacy Codebase
The old Python/FastAPI based approach has been moved over to the `legacy-python-fastapi` branch in case we would need any of it again later. The main reason to switch branches was for security and atomicity reasons, cause Rust provides hardened static typing and also compiles to a single binary for ease of deployment.
