Lancer l'application :
cargo run

Tester l'application avec des champs prédéfinis :

curl -v -H "Content-Type: application/json"  -X POST -d '{"last_name": "GAUDE","first_name":"MATHIEU","genre":"M","birth_date":"02112000","birth_country":"FR","diploma_degree":"7","diploma_domain":"IN","diploma_mention":"","diploma_speciality":"","diploma_type":"IN"}' 0.0.0.0:4000/2ddoc

Le 2D-doc est généré à la racine du projet
