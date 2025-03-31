import { Request, Response } from 'express'
import { BN_ZERO, Mangata } from 'gasp-sdk'

import { BadRequestException } from '../error/Exception.js'
import * as errorHandler from '../error/Handler.js'
import { mgxAirdropRepository } from '../repository/MgxAirdropRepository.js'
import {
  mgxAirdropEligibilitySchema,
  mgxAirdropLinkAddressSchema,
  mgxAirdropSignatureSchema,
} from '../schema/MgxAirdropSchema.js'
import {
  getEligibilityAtBlockN,
  verifySignature,
} from '../service/MgxAirdropService.js'

const SNAPSHOTS = process.env.MGX_AIRDROP_SNAPSHOTS.split(',')

export async function checkEligibility(req: Request, res: Response) {
  /*
    #swagger.tags = ['mgx-airdrop']
    #swagger.summary = 'Check eligibility for MGX airdrop'
    #swagger.description = 'Verifies if the provided address is eligible for the airdrop based on snapshots.'
    #swagger.parameters['address'] = {
     in: 'path',
     description: 'MangataX address',
     required: true,
     type: 'string'
   }
    #swagger.responses[200] = {
      description: 'Successful response',
      schema: {
        mangataXAddress: '5Gxkb...PDnnR',
        isEligible: true
      }
    }
    #swagger.responses[400] = {
      description: 'Validation error'
    }
    #swagger.responses[500] = {
      description: 'Internal Server Error'
    }
  */
  try {
    const { address } = req.params

    mgxAirdropEligibilitySchema.validateSync({ address })

    const MANGATA_API = await Mangata.instance([
      process.env.OLD_MANGATA_NODE_URL,
    ]).api()

    const snapshotPromises = SNAPSHOTS.map((snapshot) =>
      getEligibilityAtBlockN(MANGATA_API, snapshot, address)
    )

    const snapshots = await Promise.all(snapshotPromises)
    const weight = snapshots.reduce(
      (acc, snapshot) => acc.add(snapshot.weight),
      BN_ZERO
    )

    return res.json({
      mangataXAddress: address,
      isEligible: weight.gtn(0),
    })
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export async function linkAddress(req: Request, res: Response) {
  /*
    #swagger.tags = ['mgx-airdrop']
    #swagger.summary = 'Link Ethereum address to MangataX address'
    #swagger.description = 'Links the provided Ethereum address to the provided MangataX address.'
    #swagger.parameters['body'] = {
      in: 'body',
      description: 'JSON object containing the Ethereum and MangataX addresses to be linked',
      required: true,
      schema: {
        ethereumAddress: '0x...',
        mangataXAddress: '5Gxkb...PDnnR'
      }
    }
    #swagger.responses[200] = {
      description: 'Successful response',
      schema: {
        ethereumAddress: '0x...',
        mangataXAddress: '5Gxkb...PDnnR'
      }
    }
    #swagger.responses[400] = {
      description: 'Validation error'
    }
    #swagger.responses[500] = {
      description: 'Internal Server Error'
    }
  */
  try {
    const { mangataXAddress, ethereumAddress } = req.body

    mgxAirdropLinkAddressSchema.validateSync({
      mangataXAddress,
      ethereumAddress,
    })

    const { signature } = mgxAirdropSignatureSchema.validateSync({
      signature: req.headers['x-signature'],
    })

    const isSignatureValid = verifySignature(
      { mangataXAddress, ethereumAddress },
      signature
    )

    if (!isSignatureValid) {
      throw new BadRequestException('Invalid signature')
    }

    const record = await mgxAirdropRepository
      .search()
      .where('mangataXAddress')
      .eq(mangataXAddress)
      .returnFirst()

    if (record) {
      record.ethereumAddress = ethereumAddress
      await mgxAirdropRepository.save(record)

      return res.json({
        mangataXAddress,
        ethereumAddress,
      })
    }

    const MANGATA_API = await Mangata.instance([
      process.env.OLD_MANGATA_NODE_URL,
    ]).api()

    const snapshotPromises = SNAPSHOTS.map((snapshot) =>
      getEligibilityAtBlockN(MANGATA_API, snapshot, mangataXAddress)
    )
    const snapshots = await Promise.all(snapshotPromises)

    const mgxHoldings = snapshots.reduce((acc, snapshot) => {
      acc[snapshot.block] = snapshot.blockMgxHoldings.toString()
      return acc
    }, {})

    const mgxLpHoldings = snapshots.reduce((acc, snapshot) => {
      acc[snapshot.block] = snapshot.blockMgxLpHoldings.toString()
      return acc
    }, {})

    const weight = snapshots.reduce(
      (acc, snapshot) => acc.add(snapshot.weight),
      BN_ZERO
    )

    const isEligible = weight.gtn(0)

    if (!isEligible) {
      throw new Error('Address is not eligible for the airdrop')
    }

    await mgxAirdropRepository.save({
      mangataXAddress,
      ethereumAddress,
      isEligible,
      weight: weight.toString(),
      blockMgxHoldings: JSON.stringify(mgxHoldings),
      blockMgxLpHoldings: JSON.stringify(mgxLpHoldings),
    })

    return res.json({
      mangataXAddress,
      ethereumAddress,
    })
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
