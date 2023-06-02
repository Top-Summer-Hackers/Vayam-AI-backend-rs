# VAYAM-ai BACKEND

## Setup and run

Make sure you have Rust and Cargo installed with the `default` toolchain.  
With rustup `curl https://sh.rustup.rs -sSf | sh -s`

1. Clone the repo and go in the directory,  
   `git clone https://github.com/Top-Summer-Hackers/Vayam-AI-backend-rs.git`.
2. Install all the project’s dependencies and build the project by running the command `cargo build`.
3. Start the Axum-Backend HTTP server by running `cargo run`.

## Test the API endpoints

Fetch all the client(provider or employee):

`curl http://localhost:8080/api/client`

Submit a new client(provider or employee):

`curl -X POST http://localhost:8080/api/client -d '{"_id": "0x546847854","user_name":"Scroll","description":"zk","password":"123"}' -H "content-type: application/json"`

Fetch all the tasks:

`curl http://localhost:8080/api/task`

Get task by skill:

`curl http://localhost:8080/api/task/{:skill}`

Submit a new task:

`curl -X POST http://localhost:8080/api/task -d '{"title":"Create bank-end","start_time":"22/01/2023","deadline":"29/10/2023","description":"Back end on rust", "skills":["Solidity","Rust"],"bounty":400 }' -H "content-type: application/json"`

Fetch all the proposals:

`curl http://localhost:8080/api/proposal`

Submit a new proposal:

`curl -X POST http://localhost:8080/api/proposal -d '{"task_id": "1","freelancer_id": "0x5456498798","milestones": [{"description": "Dataset collection ","deadline": "12-05-2023","price": 220},{"description": "Cleanning dataset","deadline": "12-06-2023","price": 50}]}' -H "content-type: application/json"`

Fetch all the freelancers:

`curl http://localhost:8080/api/freelancer`

Submit a new freelancer:

`curl -X POST http://localhost:8080/api/freelancer -d '{"_id": "0x001546847854","user_name":"Medhi",	"description":"Auditor","password":"123"}' -H "content-type: application/json"`
