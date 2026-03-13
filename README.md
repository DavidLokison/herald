# Herald
This is the REST-API application layer of the historischer-besiedlungszug.de website written in Rust using the [Rocket](https://rocket.rs/) crate. Its purpose is to take HTTP(S) requests from the webpage in JSON format and to trigger the respective backend handles on the database. The API shall then return an appropriate response code, potentially also a JSON body with the requested data.

## Guidelines
- This project is built security-first. No communication with other services shall be logged in production mode whatsoever.
- Encryption and other concerns are bound to change. It is advised to define and call a small amount of wrapper functions for communicating with other services.

## Configuration
- `Rocket.toml`: The service uses Rocket's default configuration file. See its [Configuration Guide](https://rocket.rs/guide/v0.5/configuration/) for details.
- `config.yaml`: Provided by the repository is the default generated config for running a Dolt SQL Server locally. The only purpose is that it doesn't have to be gitignored as it "feels strange" to gitignore a public non-dotfile configuration file.
- `.env`: SQLx has a feature of compile-time checking queries and resolving to the actual data types. For that to work, the database URL has to be set. The local `.env` provides the defaults for the local environment and really shouldn't be changed unless you know exactly what you are doing.

## Dependencies
### Binary Dependencies
- [Rust](https://rust-lang.org/) (obviously)
- [Dolt](https://www.dolthub.com/) for the database backend

### Data Dependencies
- [Herald](https://www.dolthub.com/repositories/besiedlungszug/herald)
- [base32](https://www.dolthub.com/repositories/davidlokison/base32)

To set up the database backend for contribution to this service, run the following commands from inside the repository:
```sh
dolt clone besiedlungszug/herald .
dolt clone davidlokison/base32
```

The `.gitignore` file is set up to exclude local dolt related files so that the data project can live in the same directory as the software project.

## Contribution
Contribution is highly welcome, whether by writing Issues or providing code. The Cargo framework should take care about most of the dependencies, but you need a local copy of the backend database service as well.

To get the database running, you can run `dolt sql-server` from the project directory.

## Legacy Codebase
The old Python/FastAPI based approach has been moved over to the `legacy-python-fastapi` branch in case we would need any of it again later. The main reason to switch branches was for security and atomicity reasons, cause Rust provides hardened static typing and also compiles to a single binary for ease of deployment.
