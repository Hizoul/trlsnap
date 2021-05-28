import * as React from "react"
import { get } from "lodash"
import { FormStore } from "@xpfw/form"
import { MdExpandLess, MdExpandMore } from "react-icons/md"
import { observer } from "mobx-react"

const BulmaCard: React.FunctionComponent<{
  title?: string
  footer?: React.ReactElement<any>,
  mapTo: string
  defaultIsOpen?: boolean
}> = observer((props) => {
  const showCardContent = FormStore.getValue(`ui.isCardOpen.${props.mapTo}`, undefined, get(props, "defaultIsOpen", false))
  let header = props.title ? (
    <div
      className="card-header"
      onClick={() => {
        FormStore.setValue(`ui.isCardOpen.${props.mapTo}`, !showCardContent)
      }}
    >
        <p className="card-header-title">
          {props.title}
        </p>
        <div className="switchIcon">
          {showCardContent ? <MdExpandLess /> : <MdExpandMore />}
        </div>
      </div>
  ) : null

  let footer = props.footer ? (
    <footer className="card-footer">
      {props.footer}
    </footer>
  ) : null
  return (
    <div className="card miniInset">
      {header}
      {showCardContent ? (
        <>
          {props.children}
          {footer}
        </>
      ) : null}
      
    </div>
  )
})

export default BulmaCard
