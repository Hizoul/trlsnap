import React from "react"
import { get, isArray } from "lodash"
import { DbStore, useList, toJS } from "@xpfw/data"
import { ExtendedJSONSchema, FormStore, getMapTo } from "@xpfw/form"
import { observer, PropTypes } from "mobx-react"
import { authors, SCHEMA_DATA } from "../data"

const convertDate = (date: any, key: any) => {
  if (key == "authors") {
    let names = ""
    for (const author_id of date[key]) {
      if (authors[author_id] != null) {
        names += `${authors[author_id].name}, `
      }
    }
    return names.substr(0, names.length-2)
  } else {
    let labelConversion = get(SCHEMA_DATA.properties, `${key}.selectOptions`)
    if (labelConversion != null && labelConversion.length > 0) {
      let label = ""
      let vals = date[key]
      vals = isArray(vals) ? vals : [vals]
      for (const k of vals) {
        for (const convert of labelConversion) {
          if (convert.value == k) {
            label += `${convert.label}, `
          }
        }
      }
      if (label.length > 0) {
        return label.substr(0, label.length-2)
      }
    }
  }

  let vals = date[key]
  if (isArray(vals)) {
    let label = ""
    for (const k of vals) {
      label += `${k}, `
    }
    return label.substr(0, label.length-2)
  }
  return date[key]
}

const DataTable: React.FunctionComponent<{
  data: any[],
  keys?: Array<string>,
  header?: Array<{key: string, label: string}>
  limitHeight?: boolean
}> = (props) => {
  const renderedItems: any = []
  let renderedHeader: any = undefined
  let keysToUse: any = props.keys
  if (keysToUse == null && props.data.length > 0) {
    keysToUse = Object.keys(props.data[0])
  }
  for (const listEntry of props.data) {
    renderedItems.push((
      <tr key={listEntry.id}>
        {keysToUse.map((k: any) => <td key={k}>{convertDate(listEntry, k)}</td>)}
      </tr>
    ));
  }
  if (props.header != null) {
    renderedHeader = props.header.map((o) => (
      <th key={o.label}>
        {o.label}
      </th>))
  }
  return (
    <div
      className={`table-container maxtable`}
    >
      <table className="table is-bordered is-striped is-hoverable is-fullwidth ">
        <tbody>
          <tr key="header">
            {renderedHeader}
          </tr>
          {renderedItems}
        </tbody>
      </table>
    </div>
  )
}


const XpfwTable: React.FunctionComponent<{
  schema: ExtendedJSONSchema,
  mapTo?: string,
  prefix?: string,
  keys?: Array<string>,
  container?: React.FunctionComponent<{amount: number, data: any[]}>
}> = observer((props) => {
  FormStore.setValue(`${getMapTo(props.schema, props.mapTo)}.$limit`, 0, props.prefix)
  const listData = useList(props.schema, props.mapTo, props.prefix, undefined)
  let keysToUse: any = props.keys
  if (keysToUse == null && props.schema.properties != null) {
    keysToUse = Object.keys(props.schema.properties)
  }
  let data: any = []
  if (listData.list != null && listData.list.data != null && listData.list.data.length > 0) {
    data = listData.list.data
  }
  const header = keysToUse.map((k: string) => {
    return {key: k, label: get(props.schema.properties, `${k}.label`)}
  })
  let toRet = (
    <DataTable data={data} keys={keysToUse} header={header} />
  )
  if (props.container != null) {
    let Ele = props.container
    return (
      <Ele amount={data.length} data={data}>
        {toRet}
      </Ele>
    )
  }
  return toRet
})

export {
  XpfwTable
}
export default DataTable