# pow_server
To run docker build:
```
docker build -t server -f Dockerfile_server .
docker build -t client -f Dockerfile_client .
```
Alorithm for PoW is hashcash, cause it's very popular and well-known algorithm and the solution check alorithm is pretty fast. 
The downside is it's hard to find good difficulty parameter, cause we can have clients with very large perfomance gap.
