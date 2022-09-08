FROM ubuntu:latest

RUN apt-get update \
  && apt-get install -y curl ca-certificates pkg-config libssl-dev git unzip build-essential --no-install-recommends \
  && curl -sL https://deb.nodesource.com/setup_18.x | bash - \
  && curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - \
  && echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list \
  && apt-get update \
  && apt-get install -y nodejs yarn --no-install-recommends \
  && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add wasm32-unknown-unknown \
  && cargo install cargo-generate trunk \
  && npm cache verify \
  && yarn cache clean --all \
  && rm -rf ${CARGO_HOME}/git/* \
  && rm -rf ${CARGO_HOME}/registry/* \
  && rm -rf /var/lib/apt/lists/* 

RUN cargo install wasm-pack

ENV PATH="/root/.cache/.wasm-pack/.wasm-bindgen-cargo-install-0.2.82/bin:${PATH}"

WORKDIR /app

COPY package.json .

RUN yarn

COPY frontend/package.json ./frontend/

RUN yarn deps-install

COPY . .

RUN printf 'n\n' | yarn deploy

EXPOSE 3000

CMD [ "yarn", "start" ]