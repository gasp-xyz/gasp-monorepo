FROM node:18.19 AS build

WORKDIR /app

COPY package.json package-lock.json /app/
RUN npm ci

COPY . .
# RUN npm run build && npm prune --production
RUN npm run build
RUN sh -c "md5sum src/RollDown.json > abi.md5sum"
CMD ["npm", "start"]

# FROM node:18.19-slim
#
# # Add Tini, according to: https://github.com/nodejs/docker-node/blob/main/docs/BestPractices.md#handling-kernel-signals
# RUN apt-get update && apt-get install -y tini && rm -rf /var/lib/apt/lists/*
# ENTRYPOINT ["/usr/bin/tini", "--"]
#
# ENV NODE_ENV=production
#
# WORKDIR /app
#
# run
# COPY package*.json ./
# COPY --from=build /app/abi.md5sum /
# COPY --from=build /app/build /app/build
# COPY --from=build /app/node_modules /app/node_modules
#
# CMD ["node", "build/index.js"]
