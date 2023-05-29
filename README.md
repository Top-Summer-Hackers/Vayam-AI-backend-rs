# VAYAM-ai BACKEND

## Setup and run

Make sure you have Rust and Cargo installed with the `default` toolchain.  
With rustup `curl https://sh.rustup.rs -sSf | sh -s`

1. Clone the repo and go in the directory,  
   `git clone https://github.com/Top-Summer-Hackers/Vayam-AI-backend-rs.git`.
2. Install all the project’s dependencies and build the project by running the command `cargo build`.
3. Start the Axum-Backend HTTP server by running `cargo run`.

## Test the API endpoints

Fetch all the providers:

`curl http://localhost:8080/api/provider`

Submit a new provider:

`curl -X POST http://localhost:8080/api/provider -d '{"user_name": "Binance",	"description": "DEX", "password":"123"}' -H "content-type: application/json"`

Fetch all the tasks:

`curl http://localhost:8080/api/task`

Submit a new task:

`curl -X POST http://localhost:8080/api/task -d '{"title": "Create bank-end","start_time": "22/01/2023","deadline": "29/10/2023","description": "Back end on rust" }' -H "content-type: application/json"`
