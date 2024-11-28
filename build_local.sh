!#/usr/bin/bash

docker buildx build --platform linux/amd64 --push -t localhost:5000/purplerag_db_api .
