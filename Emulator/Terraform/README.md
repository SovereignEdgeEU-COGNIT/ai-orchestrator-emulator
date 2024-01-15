# How to run
Load terraform providers (modules/packages/..):
    terraform init
See how the depl. will change when running the script
    terraform plan
Apply changes
    terraform apply

# Notes:

### Automatic container rebuilding:
[Link](https://registry.terraform.io/providers/kreuzwerker/docker/latest/docs/resources/image)
You can rebuild the containers using triggers of e.g. source code change.
Is this a better approach than pushing the info from some source instead?

### Multiple network interfaces 
[Link](https://stackoverflow.com/questions/34110416/start-container-with-multiple-network-interfaces)