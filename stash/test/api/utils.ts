import { GenericContainer, StartedTestContainer, Wait } from "testcontainers";

export const TIMESERIES_HOST_DOCKER_IMAGE_NAME = "mangatasolutions/redis-test-stash_ts:v2";
export const REDIS_HOST_DOCKER_IMAGE_NAME = "mangatasolutions/redis-test-stash:latest";
export const MAX_DAYS="max";
export const MAX_INTERVAL="day";
let redisContainer: StartedTestContainer;
let timeSeriesContainer: StartedTestContainer;

export async function startContainer(image: string) {
    console.warn("Starting container: " + image);
    if(image === TIMESERIES_HOST_DOCKER_IMAGE_NAME) {
        return await new GenericContainer(image)
            .withWorkingDir("/")
            .withEntrypoint(["/entrypoint.sh"])
            .withExposedPorts({container: 6379, host: 6379})
            .withWaitStrategy(Wait.forLogMessage("Ready to accept connections"))
            .start();
    } else {
        return await new GenericContainer(image)
            .withWorkingDir('/')
            .withEntrypoint(['redis-server'])
            .withExposedPorts({container: 6379, host: 6380})
            .withWaitStrategy(Wait.forLogMessage('Ready to accept connections'))
            .start()
    }
}
export async function tearDownBothContainers() {
    console.warn("Tearing down containers");
    if (redisContainer) {
        await redisContainer.stop();
    }
    if (timeSeriesContainer) {
        await timeSeriesContainer.stop();
    }
}
export async function initBothContainers() {
    try {
        redisContainer = await startContainer(REDIS_HOST_DOCKER_IMAGE_NAME);
        timeSeriesContainer = await startContainer(TIMESERIES_HOST_DOCKER_IMAGE_NAME);
    } catch (e) {
        console.error(e);
        throw e;
    }
}
