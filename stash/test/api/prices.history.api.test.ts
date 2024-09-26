
import { describe, it , expect} from 'vitest'
import app from "../../src/app";
import supertest from "supertest";

const priceHistoryPath = "price-history"
describe('Prices', () => {

  describe('/GET prices', () => {
    it('[MGX-597] - pools should not be returned on prices', async() => {
      await supertest(app)
        .get(`/${priceHistoryPath}/GASPV2-L1Asset?interval=day&days=300`)
        .expect(500)
        .then((response) => {
          const invalidTokenNameResponse = response.body;
          expect(invalidTokenNameResponse.exceptionName).to.contain("ValidationError")
          expect(invalidTokenNameResponse.message).to.contain("this must be one of the following values: GASPV2, L1Asset, GASPV2-ETH, L1Asset-GASPV2")
        });
    });

    it('GET /price-history - Schema validation', async() => {
      await supertest(app)
        .get(`/${priceHistoryPath}/GASPV2`)         
        .expect(200)
        .then((response) => {
          expect(response.body).toMatchSnapshot("Prices")
          expect(response.body.error).toBeUndefined();
          expect(response.body.prices).toBeDefined();
          expect(response.body.prices).toBeInstanceOf(Array);
        });
    });
  });
});
