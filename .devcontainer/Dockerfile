FROM ubuntu:latest

WORKDIR /home/

COPY . .

RUN bash ./setup.sh

ENV PATH="/root/.cargo/bin:$PATH"

#RUN bash ./setup_cargo.sh