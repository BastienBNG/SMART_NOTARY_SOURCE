SMART NOTARY WEB INTERFACE :

To run this project : cargo run

Go to http://0.0.0.0:8000/welcome -- You will see the landing page

You can register on SIGN UP 
You can login on SIGN IN

You can go on Certifier un nouveau Document and add all the informations to Generate the 2DDOC

YOU CAN'T REGISTER AND LOGIN WHEN ONLY THIS POD IS RUNNING !


TO DEPLOY :

kubectl apply -f k8s-rdms.yaml --> Deploy document Management Pod
kubectl apply -f k8s-rqs.yaml --> Deploy 2DDOC Generator Pod
kubectl apply -f k8s-rws.yaml --> Deploy Web interface Pod
kubectl apply -f minio.yaml --> Deploy MinIO Pod
