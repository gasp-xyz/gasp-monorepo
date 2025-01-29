FROM node:18.20.4 AS build

WORKDIR /app

RUN corepack enable && corepack prepare yarn@4.6.0 --activate

COPY package.json yarn.lock .yarnrc.yml /app/
RUN yarn install --immutable

COPY . .
RUN yarn run build && yarn workspaces focus --production

FROM node:18.20.4-slim

# Add Tini, according to: https://github.com/nodejs/docker-node/blob/main/docs/BestPractices.md#handling-kernel-signals
RUN apt-get update && apt-get install -y tini && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["/usr/bin/tini", "--"]

ENV NODE_ENV=production

WORKDIR /app

COPY package.json yarn.lock ./
COPY --from=build /app/build /app/build
COPY --from=build /app/node_modules /app/node_modules

CMD ["node", "build/src/main.js"]
