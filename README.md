# VAYAM-ai BACKEND

## Linea-Infura

Web3 API Key: https://linea-goerli.infura.io/v3/73089c0ff2004798808153ece9388b4b

## Setup and run

Make sure you have Rust and Cargo installed with the `default` toolchain.  
With rustup `curl https://sh.rustup.rs -sSf | sh -s`

1. Clone the repo and go in the directory,  
   `git clone https://github.com/Top-Summer-Hackers/Vayam-AI-backend-rs.git`.
2. Install all the project’s dependencies and build the project by running the command `cargo build`.
3. Start the Axum-Backend HTTP server by running `cargo run`.

4. If build fails, you might want to run `sudo apt install cmake` first.

## Test the API endpoints

Fetch all the client(provider or employee):

`curl http://0.0.0.0:8080/api/client --cookie auth-token={auth-token}`

Submit(register) a new client(provider or employee):

`curl -X POST http://localhost:8080/api/client -d '{"id": "0x546847854","user_name":"Scroll","description":"zk","password":"123"}' -H "content-type: application/json"`

Verify user(clients) credentials(login)

`curl --request POST \
 --url http://0.0.0.0:8080/api/login \
 --header 'Content-Type: application/json' \
 --data '{
"user_name": "Scroll",
"password": "123",
"role": "client"
}'`

Fetch all the freelancers:

`curl http://localhost:8080/api/freelancer --cookie auth-token={auth-token}`

Submit(Register) a new freelancer:

`curl -X POST http://localhost:8080/api/freelancer -d '{"id": "0x001546847854","user_name":"Medhi",	"description":"Auditor","password":"123", "skills": []}' -H "content-type: application/json"`

Verify user(freelancer) credentials(login)

`curl -v -X POST --url http://0.0.0.0:8080/api/login \
  --header 'Content-Type: application/json' \
  --data '{
    "user_name": "Medhi",
    "password": "123",
		"role": "freelancer"
}'`

Fetch all the tasks:

`curl http://localhost:8080/api/task --cookie auth-token={auth-token}`

Get task by skill:

`curl http://localhost:8080/api/task/{:skill} --cookie auth-token={auth-token}`

Submit a new task(Only by client) :

`curl -X POST http://localhost:8080/api/task -d '{
	"client_id": "0x418564867486","title":"Create bank-end","start_time":"22/01/2023","deadline":"29/10/2023","description":"Back end on rust", "skills":["Solidity","Rust"],"bounty":400 }' -H "content-type: application/json" --cookie auth-token={auth-token}`

Fetch all the proposals:

`curl http://localhost:8080/api/proposal --cookie auth-token={auth-token}`

Get proposal by id:

`curl http://localhost:8080/api/proposal/{proposal_id} --cookie auth-token={auth-token}`

Submit a new proposal(Only by freelancer):

`curl --request POST \
  --url http://localhost:8080/api/proposal \
  --header 'Content-Type: application/json' \
	--cookie auth-token={auth-token}\
  --data '{
	"client_id": "0x18549841886",
	"task_id": "2",
	"freelancer_id": "0x54e3544598798"
}'`

Fetch all the milestones:

`curl http://localhost:8080/api/milestone --cookie auth-token={auth-token}`

Add block of milestones:

`curl --request POST \
 --url http://localhost:8080/api/milestone \
 --header 'Content-Type: application/json' \
 --cookie auth-token={auth-token} \
 --data ' [{
"proposal_id": "1",
"description": "Dataset collection",
"deadline": "12-05-2023",
"price": 20
},
{
"proposal_id": "1",
"description": "Data cleanning",
"deadline": "12-05-2023",
"price": 22
},
{
"proposal_id": "1",
"description": "Data analysis",
"deadline": "12-05-2023",
"price": 2280
}]'`

Approve a proposal(Only by client):

`curl -X PATCH http://localhost:8080/api/proposal/{proposal_id} --cookie auth-token={auth-token}`

Fetch all the deals:

`curl http://localhost:8080/api/deal --cookie auth-token={auth-token}`

Submit milestone(link) (Only by freelancer)

`curl x PATCH http://localhost:8080/api/milestone/{proposal_id}/{milestone_id}/{link} --cookie auth-token={auth-token}`

Update deal transaction address (Only by client):

`curl -X PATCH http://localhost:8080/api/deal/{deal_id}/{transacction_id} --cookie auth-token={auth-token}`

Client submit deal review:

`curl --request POST --url http://0.0.0.0:8080/api/review --header 'Content-Type: application/json' --cookie auth-token={auth-token} --data '{
	"freelancer_id": "0x545649879823432",
	"client_id": "0x418564867486324",
	"deal_id": "1",
	"review": "Good",
	"stars": 4
}'`
