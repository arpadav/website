# `docker` force removal of container

The amount of times I have encountered a situation where a container refuses to be killed, using sudo/root, using `docker kill` and a bunch of `--force` flags. This is the end-all-be-all command if all other `docker` api's fail. 

```bash
kill -9 $(docker inspect -f '{{.State.Pid}}' <CONTAINER-NAME-OR-ID>)
```

Thank you for coming to my ted talk
