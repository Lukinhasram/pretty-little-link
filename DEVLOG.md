## Date: 2025-06-07

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
* No previous experience with the chosen crates, which required significant time reading documentation to compare and select dependencies.
* While the overall backend logic is familiar from my Node.js experience, understanding Rust's specific ownership and type system in this context was the main hurdle.

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