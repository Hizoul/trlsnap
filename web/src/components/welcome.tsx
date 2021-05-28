import * as React from "react"

import "./form"
import { SharedField } from "@xpfw/form"
import { SCHEMA_DATA, PREFIX_FILTERS } from "../data"
import { get } from "lodash"

const Welcome: React.FunctionComponent<{isDetail?: boolean}> = () => {
  return (
    <>
      <section className="hero is-primary has-text-centered">
        <div className="hero-body">
          <p className="title is-size-1">Transfer in Reinforcement Learning</p>
          <p className="subtitle is-size-3">An interactive snapshot of the field</p>
        </div>
      </section>
      <div className="content">
        This website offers an interative look into the data that is analyzed in the paper "PCG: Better Benchmarks for Transfer in Reinforcement Learning".
      </div>
    </>
  )
}

export default Welcome
