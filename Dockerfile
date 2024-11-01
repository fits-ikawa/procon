FROM python:3.8.10-slim

RUN apt-get update && apt-get install -y curl

RUN curl -LsSf https://astral.sh/uv/install.sh | sh && \
    echo '. $HOME/.cargo/env' >> $HOME/.bashrc

WORKDIR /app

COPY ./pyproject.toml /app/

RUN . $HOME/.bashrc && uv sync