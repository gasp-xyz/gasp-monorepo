
import { describe, it , expect} from 'vitest'
import chaiHttp from 'chai-http';
import chai from 'chai';
import app from "../../src/app";
import supertest from "supertest";

chai.use(chaiHttp);
chai.should();
const priceHistoryPath = "price-history"
describe('Prices', () => {

  describe('/GET prices', () => {
      it('[MGX-597] - pools should not be returned on prices', async() => {
          await supertest(app)
              .get(`/${priceHistoryPath}/MGX-TUR?interval=day&days=300`)
              .expect(500)
              .then((response) => {
                  const invalidTokenNameResponse = response.body;
                  expect(invalidTokenNameResponse.exceptionName).to.contain("ValidationError")
                  expect(invalidTokenNameResponse.message).to.contain("this must be one of the following values: MGX, KSM,")
              });
        });
  });

});
