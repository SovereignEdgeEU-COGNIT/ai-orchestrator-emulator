# What is this?
Simon wanted to try Ansible as an alternative to Terraform
Why:
    Terraform recently went BSL instead of Open source
    The following description lead me to believe that re-configuring the network topology and app/client behaviours would be easier in Ansible:
        **Terraform** might be a better fit if your primary focus is on provisioning and managing the lifecycle of OpenNebula infrastructure with a declarative approach.**Ansible** would be more suitable if you are looking at configuring and managing the state of already provisioned resources, or if you need a tool for a broader range of automation tasks beyond just infrastructure provisioning.
    

# Ansible commands
ansible-playbook -i hosts docker_playbook.yml
ansible infra -m win_ping
ansible-galaxy collection list

# How to on Windows
**WARNING**: Windows install w. Docker and Ansible in WSL is hard to get to work:
    The networking between the Ansible WSL and the Docker containers (also running in WSL) is tricky to get to work. Prob. can be solved but haven't figured out yet
Start-service winrm
Set-Item -Path WSMan:\localhost\Service\Auth\Basic -Value $true
winrm set winrm/config/service '@{AllowUnencrypted="true"}'
winrm set winrm/config/client/auth '@{Basic="true"}'
winrm set winrm/config/service/auth '@{Basic="true"}'

If connection still fails (e.g. WSL can't ping Win host):
* Win firewall naturally blocks on that connection
* Try ping from Win to WSL
* Make sure WinRM is on
* Username/Pw is the local username but the Microsoft account Pw

