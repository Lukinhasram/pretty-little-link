### Date: 2025-06-09

**Today's focus:**

* Implement the core link shortening logic and integrate it with Postgres database.

**Progress:**

* Set up a Postgres database using `docker-compose`.
* Established a modular project structure with `routes`, `services`, `models`, and a `lib.rs` / `main.rs` binary/library
  pattern.
* Used `sqlx-cli` to create and run a database migration for the `links` table.
* Implemented the `AppState` pattern to share the `sqlx::PgPool` database connection pool across the application.
* Wrote the core business logic in the `shortener` service, including unique short code generation with a
  collision-retry loop.
* Implemented the `POST /shorten` API handler to create new links.
* Implemented the `GET /{short_code}` API handler for performing HTTP redirects.
* Successfully tested the end-to-end functionality using Postman (I've changed to Insomnia in the end for no particular
  reason), debugging and fixing connection and syntax errors along the way.

**Challenges:**

* Debugging the `docker-compose` and `.env` interaction to resolve the `FATAL: role does not exist` error in the
  database container.
* Troubleshooting network-level `Connection refused` errors and identifying typos in the `docker-compose.yml`.
* Understanding the correct `Axum` syntax for path parameters (e.g., `/{short_code}` vs `/:short_code`).
* Ensuring the `AppState` was correctly created in `main.rs` and passed to the router to be available in the handlers.

**Learnings:**

* How to use `sqlx` to connect to a database, run migrations, and map query results to Rust structs using
  `#[derive(FromRow)]`.
* How to separate modules in Rust.
* Learned about the "binary and library" crate pattern and how to follow it, clearly separating the
  executable `main.rs` from the core logic `lib.rs`.
* Learned about the AppState pattern.
* The trade-offs between short code generation strategies: a stateless random approach with database collision handling
  versus a deterministic, encoded database sequence (serial id -> base62).

**Next Steps:**

* Refine the application's error handling and logging.
* Prepare the application for deployment by creating the backend Docker container.
* Write basic project documentation, including a `README.md` with setup instructions.
* Thoroughly test the create and redirect functionality.

___

### Date: 2025-06-07

**Today's focus**

* complete the initial project setup and basic axum server

**Progress:**

* Created the cargo project.
* Added the core dependencies *(Axum, Tokio, Serde and Tracing)*.
* Implemented the basic async `main` function with the `#[tokio::main]` macro.
* Configured the `tracing-subscriber` with a default `EnvFilter` for logging.
* Created the basic Axum `Router` and defined a simple `GET /health` endpoint.
    * Implemented the `health_check_handler` function.
* Created a `TcpListener` bound to localhost:3000 and served the application.
* Created an `AppError` enum and implemented the `IntoResponse` trait for centralized error handling.

**Challenges**

* This is my first practical application in Rust outside of Rustlings exercises.
* No previous experience with the chosen crates, which required significant time reading documentation to compare and
  select dependencies.
* While the overall backend logic is familiar from my Node.js experience, understanding Rust's specific ownership and
  type system in this context was the main hurdle.

**Learnings**

* Basics of Tokio's async runtime and the role of `#[tokio::main]`.
* How to build a basic API with Axum, including the Router, method routing with get(), and handler functions.
* The fundamentals of the tracing framework for structured logging.
* How to define a custom error type and implement Axum's IntoResponse trait to create standardized HTTP error responses.
* How to serve the application with Axum.

**Next Steps:**

* Define the API requests/responses.
* Basic short code generation strategy.
* Set up Postgres database and database interaction.
* Basic input validation.