SMART NOTARY AUTHENTICATION APP :

To run and test this project alone : 

# SSL libs + Clang/LLVM
sudo apt install openssl libssl-dev clang llvm-dev libclang-dev

# SQLX CLI
cargo install --git https://github.com/launchbadge/sqlx sqlx-cli

# Run docker-compose (postgres database)
docker-compose up -d

# Run migrations
sqlx mig run

# Run the server (http://localhost:3000)
cargo run

You need to change the IP address of the postgres Database in .env file.


ALL THE COMMAND TO TEST WITH A JSON :

# Register :

curl --request POST \
  --url 0.0.0.0:3000/signup \
  --header 'content-type: application/json' \
  --data '{
      "username": "user1",
      "email": "user1@example.com",
      "password": "user1"
  }'

# Login :

curl --request POST \
  --url 0.0.0.0:3000/auth \
  --user user1

# Get Informations :

curl --request GET \
--url 0.0.0.0:3000/me \
--header 'authorization: Bearer <jwt_token>'



TO DEPLOY :

kubectl apply -f k8s-ras.yaml --> Deploy Authentication Pod
kubectl apply -f postgres.yaml --> Deploy posgres database Pod

