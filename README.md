<div id="top"></div>




<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="images/logo.jpg" alt="Logo" width="800" height="200">
  </a>

  <h3 align="center">SMART NOTARY</h3>

  <p align="center">
    How to test our pods and how to deploy them !
    <br />
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#alone-pods-tests">Alone pods tests</a>
      <ul>
        <li><a href="#authentication-pod">Authentication pod</a></li>
        <li><a href="#web-interface-pod">Web Interface pod</a></li>
        <li><a href="#2d-doc-pod">2D-Doc pod</a></li>
        <li><a href="#doc-management-pod">Doc Management pod</a></li>
      </ul>
    </li>
    <li><a href="#deployment">Deployment</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project




<p align="right">(<a href="#top">back to top</a>)</p>


Smart notary is a project that allows you to authenticate and prove the authenticity of all types of documents



<br><br>

<!-- GETTING STARTED -->
## Alone pods tests

To try each pods separately
<br><br>
### Authentication pod





SMART NOTARY AUTHENTICATION APP :

To run and test this project alone : 
<br><br>
#### SSL libs + Clang/LLVM
sudo apt install openssl libssl-dev clang llvm-dev libclang-dev
<br><br>
#### SQLX CLI
cargo install --git https://github.com/launchbadge/sqlx sqlx-cli
<br><br>
#### Run docker-compose (postgres database)
docker-compose up -d

#### Run migrations
sqlx mig run
<br><br>
#### Run the server (http://localhost:3000)
cargo run

You need to change the IP address of the postgres Database in .env file.


ALL THE COMMAND TO TEST WITH A JSON :
<br><br>
#### Register :

curl --request POST \
  --url 0.0.0.0:3000/signup \
  --header 'content-type: application/json' \
  --data '{
      "username": "user1",
      "email": "user1@example.com",
      "password": "user1"
  }'
<br><br>
#### Login :

curl --request POST \
  --url 0.0.0.0:3000/auth \
  --user user1
<br><br>
#### Get Informations :

curl --request GET \
--url 0.0.0.0:3000/me \
--header 'authorization: Bearer <jwt_token>'




<br><br>

### Web Interface pod


SMART NOTARY WEB INTERFACE :

To run this project : cargo run

Go to http://0.0.0.0:8000/welcome -- You will see the landing page

You can register on SIGN UP 
You can login on SIGN IN

You can go on Certifier un nouveau Document and add all the informations to Generate the 2DDOC

YOU CAN'T REGISTER AND LOGIN WHEN ONLY THIS POD IS RUNNING !


<br><br>
### 2D-Doc pod



To run the app :
cargo run

Test the application to generate a 2D-Doc :

curl -v -H "Content-Type: application/json"  -X POST -d '{"last_name": "GAUDE","first_name":"MATHIEU","genre":"M","birth_date":"02112000","birth_country":"FR","diploma_degree":"7","diploma_domain":"IN","diploma_mention":"","diploma_speciality":"","diploma_type":"IN"}' 0.0.0.0:4000/2ddoc

2D-Doc is generated


<br><br>
### Doc Management pod


run the app :

cargo run 

another terminal:

curl --request POST \
  --url 0.0.0.0:5100/miniopush \
  --header 'content-type: application/json' \
  --data '{
      "uid": "tress",
      "doc_name": "nameofmydoc",
      "content_doc": "000000098765432232123456789"
  }'

  => push content doc on minio at the port 9000

Work only if the MinIO pod is deploy
<br><br>
## Deployment


kubectl apply -f k8s-rdms.yaml --> Deploy document Management Pod
<br><br />

kubectl apply -f k8s-rqs.yaml --> Deploy 2DDOC Generator Pod
<br><br />

kubectl apply -f k8s-rws.yaml --> Deploy Web interface Pod
<br><br />

kubectl apply -f k8s-ras.yaml --> Deploy Authentication Pod
<br><br />

kubectl apply -f minio.yaml --> Deploy MinIO Pod
<br><br />

kubectl apply -f ingress.yaml --> Deploy Ingress
<br><br />

kubectl apply -f secret.yaml --> Deploy the Secret
<br><br />

kubectl apply -f postgres.yaml --> Deploy the Database pod postgresql




