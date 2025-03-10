import logger from './Logger.js'
import moment, { Duration, Moment } from 'moment'

export class Metrics {
  static SEC = 1000
  total: number
  done: number
  ticks: number
  checkpoint: Moment
  init: Moment
  tag: string

  constructor(tag: string) {
    this.ticks = 0
    this.done = 0
    this.checkpoint = moment()
    this.init = moment()
    this.tag = tag
  }

  setBlocks(start: number, end: number) {
    this.total = end - start
    logger.debug(`Metrics: parsing from block ${start} until ${end}`)
  }

  tick() {
    this.ticks++
    this.done++
    const current = moment()
    if (current.diff(this.checkpoint, 'seconds') > 60) {
      const elapsed = moment.duration(current.diff(this.init))
      const av = this.done / elapsed.asSeconds()
      const eta = moment.duration((this.total - this.done) / av, 'seconds')
      const completed = `completed: ${((this.done / this.total) * 100).toFixed(
        2
      )}% (${this.done}/${this.total})`
      logger.info(
        `${this.tag}: current: ${this.ticks / 60}b/s, average: ${av.toFixed(
          2
        )}, eta: ${this.eta(eta)}, ${completed}`
      )

      this.ticks = 0
      this.checkpoint = current
    }
  }

  eta(elapsed: Duration) {
    return `${elapsed.days()}d ${elapsed.hours()}:${elapsed.minutes()}:${elapsed.seconds()}`
  }
}
