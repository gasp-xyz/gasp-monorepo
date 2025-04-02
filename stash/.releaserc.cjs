/**
 * @type {import('semantic-release').GlobalConfig}
 */

const SERVICE_PREFIX = 'stash-'
const DOCKER_IMAGE_REPOSITORY = 'gaspxyz/stash'

module.exports = {
  extends: 'semantic-release-monorepo',
  branches: [
    'main',
    `(${SERVICE_PREFIX})+([0-9])?(.{+([0-9]),x}).x`,
    { name: 'release/next', prerelease: 'rc' },
  ],
  plugins: [
    '@semantic-release/commit-analyzer',
    [
      '@semantic-release/release-notes-generator',
      {
        writerOpts: {
          transform: (commit, context) => {
            // Clean the version for the template
            if (context.version) {
              context.cleanVersion = context.version.replace(
                `${SERVICE_PREFIX}v`,
                ''
              )
            }
            return commit
          },
          footerPartial: `
          {{#if noteGroups}}
          {{#each noteGroups}}
          ### {{title}}
          {{#each notes}}
          * {{text}}
          {{/each}}
          {{/each}}
          {{/if}}

          ---

          ### Docker Images
          - Standard: \`${DOCKER_IMAGE_REPOSITORY}:{{cleanVersion}}\`
        `,
        },
      },
    ],
    [
      '@semantic-release/github',
      {
        assets: ['build/**/*'],
      },
    ],
    [
      '@semantic-release/exec',
      {
        shell: 'bash',
        publishCmd: `
        docker buildx imagetools create -t ${DOCKER_IMAGE_REPOSITORY}:\${nextRelease.version} ${DOCKER_IMAGE_REPOSITORY}:\${process.env.IMAGE_TAG}
      `,
      },
    ],
  ],
}
