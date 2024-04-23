import { type AppConfig, appConfigSchema } from "./types";

export function defineConfig(config: AppConfig) {
    return appConfigSchema.parse(config);
}