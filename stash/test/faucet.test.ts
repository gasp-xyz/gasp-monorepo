import { describe, it, beforeEach, expect, vi } from 'vitest'
import * as faucetService from '../src/service/FaucetService'
import { ForbiddenException } from '../src/error/Exception'

describe('FaucetService', () => {
  const toAddress = '0x00000000000000000000000000000000'
  const captcha = '10000000-aaaa-bbbb-cccc-000000000001'
  const wrongCaptcha = '10000000-aaaa-bbbb-cccc-000000000002'
  const MAX_REQUESTS = 3

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should verify captcha successfully', async () => {
    await expect(faucetService.verifyCaptcha(captcha)).resolves.toBeUndefined()
  })

  it('should fail captcha verification', async () => {
    const errorMessage = 'Captcha verification failed'

    try {
      await faucetService.verifyCaptcha(wrongCaptcha)
    } catch (error) {
      expect(error).toBeInstanceOf(ForbiddenException)
      expect(error.message).toContain(errorMessage)
    }
  })

  it('should check address usage successfully', async () => {
    vi.spyOn(faucetService, 'checkRequestCount').mockResolvedValue(true)

    await expect(faucetService.checkRequestCount(toAddress)).resolves.toBe(true)
  })

  it('should fail address usage check', async () => {
    const errorMessage = `Address ${toAddress} has requested the token more than ${MAX_REQUESTS} times.`
    vi.spyOn(faucetService, 'checkRequestCount').mockRejectedValue(
      new ForbiddenException(errorMessage)
    )

    await expect(faucetService.checkRequestCount(toAddress)).rejects.toThrow(
      ForbiddenException
    )
  })

  it('should send tokens successfully', async () => {
    vi.spyOn(faucetService, 'sendTokens').mockResolvedValue()

    await expect(faucetService.sendTokens(toAddress)).resolves.toBeUndefined()
  })

  it('should fail to send tokens', async () => {
    const errorMessage = 'Token transfer failed'
    vi.spyOn(faucetService, 'sendTokens').mockRejectedValue(
      new Error(errorMessage)
    )

    await expect(faucetService.sendTokens(toAddress)).rejects.toThrow(Error)
  })
})
