# syntax=docker/dockerfile:1
# Ref: https://github.com/nodejs/docker-node/blob/main/docs/BestPractices.md

ARG NODE_VERSION=18.20.7
FROM node:${NODE_VERSION} AS build
# # Install necessary build tools for native addons using apt-get for Debian-based image
# RUN apt-get update && \
#     apt-get install -y --no-install-recommends \
#     build-essential python3 \
#     g++ make python3 libssl-dev \
#     && rm -rf /var/lib/apt/lists/* # Clean up apt lists to reduce layer size 

WORKDIR /app
RUN corepack enable

COPY package.json yarn.lock .yarnrc.yml swagger.js swagger-output.json ./
COPY .yarn ./.yarn

# Install dependencies with cache for faster builds
RUN --mount=type=cache,target=/root/.yarn/berry/cache yarn install --immutable

COPY . .

RUN yarn run build && yarn workspaces focus --production

FROM node:${NODE_VERSION}-slim

WORKDIR /app
ENV NODE_ENV=production
RUN corepack enable

# Run the application as a non-root user.
USER node

COPY --chown=node:node package.json yarn.lock .yarnrc.yml swagger.js swagger-output.json ./
COPY --chown=node:node .yarn ./.yarn
COPY --from=build --chown=node:node /app/node_modules ./node_modules
COPY --from=build --chown=node:node /app/dist ./dist

EXPOSE 8080

# Enable source maps for better error stack traces in production environment
# https://stackoverflow.com/a/64721153/9645556
ENV NODE_OPTIONS="--enable-source-maps"

CMD ["node", "dist/server.js"]
