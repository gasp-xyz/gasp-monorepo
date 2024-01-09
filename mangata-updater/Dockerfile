FROM node:18.19

WORKDIR /app

# Add Tini, according to: https://github.com/nodejs/docker-node/blob/main/docs/BestPractices.md#handling-kernel-signals
RUN apt-get update && apt-get install -y tini && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["/usr/bin/tini", "--"]

COPY package.json package-lock.json /app/
RUN npm ci

COPY . .
RUN npm run build && npm prune --production

CMD ["node", "build/index.js"]