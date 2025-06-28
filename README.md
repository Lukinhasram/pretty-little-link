# Pretty Little Link

![Rust](https://img.shields.io/badge/Rust-DEA584?style=for-the-badge&logo=rust)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust)
![Postgres](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)
![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)
![Vite](https://img.shields.io/badge/Vite-B73BFE?style=for-the-badge&logo=vite&logoColor=FFD62E)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)

### [🚀 View the Live Application Here](https://pretty-little-link.onrender.com/)

---

This project is a full-stack URL shortening service used to create smaller, shareable links from long URLs, built with a Rust backend and a React frontend.

## 🖼️ Application Screenshot 

<p align="center">
  <img src="https://i.imgur.com/0BqQwcg.png" alt="Application Screenshot" width="800"/>
  <img src="https://i.imgur.com/g9BO6oa.png" alt="Application Screenshot" width="800"/>
  While the generated code is short, the full URL is currently hosted on Render's free service domain. Making the shortened URL pretty large too 😂.
</p>

---

## ✨ Features

* **High-Performance Backend:** Built with Rust and the Axum framework for speed and reliability.
* **Modern Frontend:** A clean, responsive user interface built with React and Vite.
* **Unique Link Generation:** Generates unique, fixed-length alphanumeric codes for each URL.
* **Database Integration:** Persists link data in a PostgreSQL database using the `sqlx` toolkit.
* **Comprehensive Test Suite:** Includes a full suite of integration tests that create and destroy a separate database for each test run, ensuring 100% test isolation.
* **Fully Containerized:** The entire stack (backend, frontend, and database) is managed with Docker Compose for a one-command local development setup.
* **Clean Architecture:** The backend and frontend are separated into distinct modules (`services`, `routes`, `hooks`, `components`), demonstrating a clean, maintainable codebase.

---

## 🛠️ Technologies Used

* **Backend Framework:** Axum (on Tokio)
* **Frontend Framework:** React (with Vite)
* **Database & ORM:** PostgreSQL, SQLx
* **Language:** Rust, JavaScript
* **Containerization:** Docker, Docker Compose
* **Deployment:** Render

---

## ⚙️ How to Run the Project Locally

This project is fully containerized. You will need `docker` and `docker-compose` installed globally on your machine.

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/Lukinhasram/link_shortener.git](https://github.com/Lukinhasram/link_shortener.git)
    cd link_shortener
    ```

2.  **Create the environment file:**
    Create a `.env` file in the project root by copying the example:
    ```bash
    cp .env.example .env
    ```

3.  **Run the application stack:**
    ```bash
    docker-compose up --build
    ```

4.  **Run database migrations (first time only):**
    With the application running, open a new terminal and run the migrations:
    ```bash
    sqlx migrate run --source ./backend/migrations --database-url "postgres://user:password@localhost:5432/link_shortener"
    ```

5.  **Visit the App:**
    * **Frontend UI:** <http://localhost:5173>
    * **Backend API:** `http://localhost:3000`

---

## Reflection

This was a personal project undertaken to gain practical experience building a full-stack application, with the **primary focus being to learn backend development with Rust.** The goal was to make a production-ready web service as a foundational step toward building more complex systems in the future, particularly in areas like blockchain.

The backend was architected with a clean separation of concerns, dividing the code into distinct `routes` (for handling web traffic), `services` (for business logic), `models` (for data structures), and a dedicated `errors` module. A key part of the learning process was implementing an error handling strategy using a custom `AppError` enum and Axum's `IntoResponse` trait, which allowed for clean and maintainable handler logic.

A significant challenge, and therefore a major learning experience, was building the integration test suite. This involved creating a test harness that programmatically spins up a new, isolated PostgreSQL database for each test run and automatically tears it down using Rust's `Drop` trait. This ensured 100% test isolation and reliability. Learning about the `sqlx` toolkit for both application logic and test database management was crucial.

Integrating the backend with the React frontend introduced challenges like **CORS**, which required a thorough understanding of browser security policies and Axum's `tower-http` middleware. The decision to use **Docker** for the entire development lifecycle was solidified after troubleshooting environment-specific bugs, as it guaranteed perfect consistency between the local development setup and the production deployment on Render.
