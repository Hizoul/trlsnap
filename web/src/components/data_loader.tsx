import * as React from "react"
import { get } from "lodash"
import { FormStore } from "@xpfw/form"
import { MdExpandLess, MdExpandMore } from "react-icons/md"
import { observer } from "mobx-react"
import { useGet } from "@xpfw/data"
import { COL_VERSION } from "../data"
import { VscLoading } from "react-icons/vsc"

const DataLoader: React.FunctionComponent<{
  toRender: React.FunctionComponent<{}>
}> = observer((props) => {
  let data = useGet(COL_VERSION, COL_VERSION)
  if (data.item == null) {
    return (
      <div className="columns is-vcentered fullColumn">
        <div className="column">
        Loading Dataset
        <br />
        <VscLoading className="spinIcon" />
        <br />
        This may take a few minutes.
        </div>
      </div>
    )
  }
  let Comp = props.toRender
  return (
    <Comp />
  )
})

export default DataLoader
