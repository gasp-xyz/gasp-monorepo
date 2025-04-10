# syntax=docker/dockerfile:1
ARG NODE_VERSION=18.20.7
FROM node:${NODE_VERSION} AS build

WORKDIR /app

RUN corepack enable && corepack prepare yarn@4.7.0 --activate

# # Use bind mounts for package.json and yarn.lock, and cache mounts for yarn install
# RUN --mount=type=bind,source=package.json,target=package.json \
#     --mount=type=bind,source=yarn.lock,target=yarn.lock \
#     --mount=type=bind,source=.yarnrc.yml,target=.yarnrc.yml \
#     --mount=type=cache,target=/usr/local/share/.cache,sharing=locked \
#     yarn install --immutable
# Use bind mounts for package.json and yarn.lock, and cache mounts for yarn install

COPY package.json yarn.lock .yarnrc.yml ./
COPY .yarn ./.yarn
RUN yarn install --immutable

COPY . .

RUN yarn run build && yarn workspaces focus --production

FROM node:${NODE_VERSION}-slim

ENV NODE_ENV=production

# Run the application as a non-root user.
USER node

WORKDIR /app

COPY package.json yarn.lock ./
COPY --from=build /app/build /app/build
COPY --from=build /app/node_modules /app/node_modules

CMD ["node", "build/src/main.js"]
