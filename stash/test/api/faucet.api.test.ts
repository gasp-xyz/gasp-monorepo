import { describe, it, beforeEach, expect, vi } from 'vitest'
import supertest from 'supertest';
import app from '../../src/app';
import * as faucetService from '../../src/service/FaucetService.js'
import { ForbiddenException } from '../../src/error/Exception.js'


describe('FaucetController', () => {
  const toAddress = '0x00000000000000000000000000000000'
  const captcha = '10000000-aaaa-bbbb-cccc-000000000001'
  const MAX_REQUESTS = 3;
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should return 500 when toAddress does not start with 0x', async () => {
    const invalidAddress = 'E9F0315FA5DAAEBB503A6D9A85C20D6BF93EA7F5';
    const errorMessage = 'toAddress must start with 0x';

    await supertest(app)
      .get(`/faucet/requestTokens/${invalidAddress}/captcha/${captcha}`)
      .expect(500)
      .then((response) => {
        const invalidAddressResponse = response.body;
        expect(invalidAddressResponse.exceptionName).toContain("ValidationError");
        expect(invalidAddressResponse.message).toContain(errorMessage);
      });
  });

  it('should return 200 on successful captcha verification, address used <= 3 times, and tokens transferred', async () => {

    vi.spyOn(faucetService, 'sendTokens').mockResolvedValue();

    await supertest(app)
      .get("/faucet/requestTokens/" + toAddress + "/captcha/" + captcha)
      .expect(200)
      .then((response) => {
        expect(response.body).toEqual({});
      });
  });

  it('should return 403 when captcha verification fails', async () => {
    const errorMessage = 'Captcha verification failed. Reason: invalid-input-response';

    await supertest(app)
      .get(`/faucet/requestTokens/${toAddress}/captcha/foo`)
      .expect(403)
      .then((response) => {
        const responseInvalidCaptcha = response.body;
        expect(responseInvalidCaptcha.exceptionName).toContain("ForbiddenException")
        expect(responseInvalidCaptcha.message).toContain(errorMessage)
      });
  });

  it('should return 403 when address is used more than 3 times', async () => {
    const errorMessage = `Address ${toAddress} has requested the token more than ${MAX_REQUESTS} times.`;

    vi.spyOn(faucetService, 'verifyCaptcha').mockResolvedValue();
    vi.spyOn(faucetService, 'sendTokens').mockRejectedValue(new ForbiddenException(
      errorMessage
    ));

    await supertest(app)
      .get(`/faucet/requestTokens/${toAddress}/captcha/${captcha}`)
      .expect(403)
      .then((response) => {
        const responseAddressUsage = response.body;
        expect(responseAddressUsage.exceptionName).toContain("ForbiddenException");
        expect(responseAddressUsage.message).toContain(errorMessage);
      });
  });

  it('should return 500 when tokens transfer fails', async () => {
    const errorMessage = 'Token transfer failed (service call error)';

    vi.spyOn(faucetService, 'verifyCaptcha').mockResolvedValue();
    vi.spyOn(faucetService, 'sendTokens').mockRejectedValue(new Error(errorMessage));

    await supertest(app)
      .get(`/faucet/requestTokens/${toAddress}/captcha/${captcha}`)
      .expect(500)
      .then((response) => {
        const tokenTransferServiceFailResponse = response.body;
        expect(tokenTransferServiceFailResponse.exceptionName).toContain("Error");
        expect(tokenTransferServiceFailResponse.message).toContain(errorMessage);
      });
  });

  it('should return 500 when captcha service call fails', async () => {
    const errorMessage = 'Captcha verification failed (service call error)';

    vi.spyOn(faucetService, 'verifyCaptcha').mockRejectedValue(new Error(errorMessage));

    await supertest(app)
      .get(`/faucet/requestTokens/${toAddress}/captcha/${captcha}`)
      .expect(500)
      .then((response) => {
        const captchaServiceFailResponse = response.body;
        expect(captchaServiceFailResponse.exceptionName).toContain("Error");
        expect(captchaServiceFailResponse.message).toContain(errorMessage);
      });
  });
});
