# syntax=docker/dockerfile:1
FROM ubuntu 
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y tzdata
RUN unlink /etc/localtime
RUN ln -s /usr/share/zoneinfo/America/New_York /etc/localt
RUN apt update
RUN apt install php-cli -y
RUN apt install golang-go -y
RUN apt install tree unzip git  -y
RUN git clone https://github.com/peteblank/directory2todolist.git
WORKDIR directory2todolist
EXPOSE 80
