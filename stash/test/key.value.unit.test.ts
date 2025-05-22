import { store } from '../src/service/KeyValueService.js';
  import { vi, describe, beforeEach, afterEach, it, expect } from 'vitest';
import { redis } from '../src/connector/RedisConnector.js';

  describe('store', () => {
    beforeEach(() => {
      vi.spyOn(redis.client, 'hset').mockResolvedValue(1);
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should store the key-value pair with a timestamp', async () => {
      const key = 'testKey';
      const value = 'testValue';
      const result = await store(key, value);

      expect(result.value).toBe(`${key}:${value}`);
      expect(typeof result.timestamp).toBe('number');
      expect(redis.client.hset).toHaveBeenCalledWith(`${key}:${value}`, 'timestamp', result.timestamp);
    });
  });