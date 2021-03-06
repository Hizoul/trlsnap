import { get } from "lodash"
import * as React from "react"
import holder from "./labelRenderer"
import { getLabelFromProps } from "@xpfw/form"

const FieldContainer: React.FunctionComponent<any> = (props) => {
  const err = props.error && props.error.ok !== true ?
    <p className="help is-danger">{JSON.stringify(props.error)}</p> : null
  const isSlider = get(props, "schema.format") === "slider"
  const labelText = `${getLabelFromProps(props)}${isSlider && props.value ? `: ${props.value}` : ``}`
  const Label = holder.label
  return (
    <div className="field">
      <Label text={labelText} />
      {props.children}
      {err}
    </div>
  )
}

export default FieldContainer
