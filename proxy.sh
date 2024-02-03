#!/bin/bash

set -xe

sudo docker build -t tranx-proxy -f proxy.Dockerfile . 
sudo docker run --net=host -it -p 7000:7000 tranx-proxy

