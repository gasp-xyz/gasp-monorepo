#!/usr/bin/env bash
set -Eeo pipefail

ROLLUP_NODE_CUSTOM_CHAINSPEC_PATH=/data/chainspec.json
ROLLUP_NODE_CUSTOM_CHAINSPEC_URL=${ROLLUP_NODE_CUSTOM_CHAINSPEC_URL}



if [ -n "${ROLLUP_NODE_CUSTOM_CHAINSPEC_URL}" ]; then
  if [ ! -f ${ROLLUP_NODE_CUSTOM_CHAINSPEC_PATH} ]; then
    echo "[entrypoint.sh] Downloading chainspec file from ${ROLLUP_NODE_CUSTOM_CHAINSPEC_URL}"
    wget -O ${ROLLUP_NODE_CUSTOM_CHAINSPEC_PATH} ${ROLLUP_NODE_CUSTOM_CHAINSPEC_URL}
    chown appuser:appuser ${ROLLUP_NODE_CUSTOM_CHAINSPEC_PATH}
  elif [ "${ROLLUP_NODE_FORCE_DOWNLOAD_CHAINSPEC}" = "true" ]; then
    echo "[entrypoint.sh] Force downloading chainspec file from ${ROLLUP_NODE_CUSTOM_CHAINSPEC_URL}"
    wget -O ${ROLLUP_NODE_CUSTOM_CHAINSPEC_PATH} ${ROLLUP_NODE_CUSTOM_CHAINSPEC_URL}
    chown appuser:appuser ${ROLLUP_NODE_CUSTOM_CHAINSPEC_PATH}
  else
    echo "[entrypoint.sh] Using existing downloaded chainspec file at ${ROLLUP_NODE_CUSTOM_CHAINSPEC_PATH}"
  fi
fi

exec /usr/bin/supervisord -c /etc/supervisor/conf.d/supervisord.conf
