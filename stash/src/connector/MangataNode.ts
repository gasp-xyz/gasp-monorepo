import { Mangata } from '@mangata-finance/sdk'

const MangataClient = Mangata.instance([process.env.MANGATA_NODE_URL])

export default MangataClient
