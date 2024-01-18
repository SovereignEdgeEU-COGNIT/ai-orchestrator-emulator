## TODO:

Make the ctrl-plane accessible from the host on some port, so you can send the updated files that way

I need to grab the IP from the container and the port for that container and send it to the registry - but how do I get the IP?
    Make sure the container has some way of getting the IP and being able to set TC
docker container inspect ...


https://tcconfig.readthedocs.io/en/latest/pages/usage/tcset/index.html#:~:text=Set%20traffic%20control%20to%20a,to%20specify%20source%2Fdestination%20container.

4.1.4.2. Set traffic control within a docker container
You need to run a container with --cap-add NET_ADMIN option if you you would like to set a tc rule within a container:

docker run -d --cap-add NET_ADMIN -t <docker image>
