import axios from 'axios'
import * as process from 'node:process'

const VERIFY_URL = 'https://api.hcaptcha.com/siteverify'
//the two enums below you can use with token 10000000-aaaa-bbbb-cccc-000000000001 to have a test example of success
const TEST_SECRET = '0x0000000000000000000000000000000000000000'
const TEST_SITEKEY = '10000000-ffff-ffff-ffff-000000000001'

export const verifyCaptcha = async (captchaToken: string): Promise<boolean> => {
  try {
    console.log('THIS IS CAPTCHA TOKEN', captchaToken)
    // Build payload with secret key and token
    const payload = {
      secret: process.env.CAPTCHA_SECRET, //comment out this, and uncomment the next line to test with the test token
      // secret: TEST_SECRET,
      sitekey: process.env.CAPTCHA_SITEKEY, //comment out this, and uncomment the next line to test with the test token
      // sitekey: TEST_SITEKEY,
      response: captchaToken,
    }
    // Make POST request with data payload to hCaptcha API endpoint
    const response = await axios.post(VERIFY_URL, payload, {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
    })
    if (!response.data.success) {
      console.log('Captcha verification failed', response.data)
      return false
    }
    return true
  } catch (error) {
    console.error('Error verifying captcha:', error)
    return false
  }
}
