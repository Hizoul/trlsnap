import React from "react"
import { get } from "lodash"
import { DbStore, useList, toJS } from "@xpfw/data"
import { ExtendedJSONSchema, FormStore, getMapTo } from "@xpfw/form"
import { observer, PropTypes } from "mobx-react"
import { Vega } from "react-vega"
import useWindowSize from "./useWindowSize"
import { MdFullscreen, MdFullscreenExit } from "react-icons/md"
import { FaChartBar, FaChartPie, FaTable } from "react-icons/fa"
import DataTable from "./table"
import { authors } from "../data"
import { splitAccessPath } from "vega"

const spec: any = {
  "$schema": "https://vega.github.io/schema/vega-lite/v5.json",
  "description": "A simple bar chart with embedded data.",
  "data": {"name": "values"},
  "mark": "bar",
  "encoding": {
  }
}



const XpfwChart: React.FunctionComponent<{
  schema: ExtendedJSONSchema,
  chartOpt: {type: number, subkey: string, defaultType?: number, labelConversion?: any},
  mapTo?: string,
  prefix?: string,
  keys?: Array<string>
}> = observer((props) => {
  FormStore.setValue(`${getMapTo(props.schema, props.mapTo)}.$limit`, 0, props.prefix)
  const visualizationType = FormStore.getValue(`ui.${props.chartOpt.subkey}${props.chartOpt.type}.vizType`, props.prefix, get(props, "chartOpt.defaultType", 0))
  const isFullscreen = FormStore.getValue(`ui.${props.chartOpt.subkey}${props.chartOpt.type}.fullScreen`, props.prefix, false)
  const listData = useList(props.schema, props.mapTo, props.prefix, undefined)
  let keysToUse: any = props.keys
  if (keysToUse == null && props.schema.properties != null) {
    keysToUse = Object.keys(props.schema.properties)
  }
  let data: any = []
  if (listData.list != null && listData.list.data != null && listData.list.data.length > 0) {
    switch (props.chartOpt.type) {
      case 0: {
        const categories: any = {}
        for (const e of listData.list.data) {
          let val = e[props.chartOpt.subkey];
          if (val != null) {
            for (const v of val) {
              if (categories[v] == null) {
                categories[v] = 1
              } else {
                categories[v] += 1
              }
            }
          }
        }
        data = Object.keys(categories).map((k) => {
          let label = k
          if (props.chartOpt.labelConversion != null) {
            for (const convert of props.chartOpt.labelConversion) {
              if (convert.value == k) {
                label = convert.label
              }
            }
          }
          return {label, value: categories[k]}
        })
        break
      }
      case 1: {
        const categories: any = {}
        for (const e of listData.list.data) {
          let val = e.authors;
          if (val != null) {
            for (const v of val) {
              if (categories[v] == null) {
                categories[v] = 1
              } else {
                categories[v] += 1
              }
            }
          }
        }
        data = Object.keys(categories).map((k) => {
          return {label: get(authors, `${k}.name`, "Unknown"), value: categories[k]}
        })
        break
      }
      case 2: {
        data = listData.list.data.map((k: any) => {
          return {date: k.publication_date.substr(0, 7), deep: k.is_deep_rl === 1 ? "Deep Neural Networks" : "Regular Algorithms"}
        })
        data = data.sort((a: any, b:any) => {
            if ( a.date < b.date ){
              return -1;
            }
            if ( a.date > b.date ){
              return 1;
            }
            return 0;
          })
        break
      }
    }
  }
  data.sort((a: any, b: any) => b.value - a.value)
  spec.data.values = data
  const size = useWindowSize()
  let multiply_width_with = isFullscreen ? 1.0 : 0.25 + Math.random() * 3
  let multiply_height_with = isFullscreen ? 0.2 : 0.25 + Math.random() * 3
  spec.width = get(size, "width", 500) - 100
  spec.height = get(size, "height", 250) - 200

  // if (props.chartOpt.type == 2) {
  //   spec.width = get(size, "width", 500) * 0.8
  // }
  let footeritems = [
  ]
  let rendered_data = null
  if (visualizationType != 0) {
    footeritems.push((
      <a
        key="bar"
        className="card-footer-item"
        onClick={() => {
          FormStore.setValue(`ui.${props.chartOpt.subkey}${props.chartOpt.type}.vizType`, 0, props.prefix)
        }}
      >
        <FaChartBar />&nbsp;Bar Chart
      </a>
    ))
    spec.mark = undefined
    spec.layer =  [{
      "mark": {"type": "arc", "outerRadius": 80}
    }, {
      "mark": {"type": "text", "radius": 90},
      "encoding": {
        "text": {"field": "value", "type": "quantitative"}
      }
    }]
    spec.view = {"stroke": null}
    spec.encoding = {
      "color": {"field": "label", "type": "nominal", "sort": ["-theta"], "title": "Type"},
      "theta": {"field": "value", "type": "quantitative", "stack": true, "title": "Count"}
    }
  }
  if (visualizationType != 1) {
    footeritems.push((
      <a
        key="pie"
        className="card-footer-item"
        onClick={() => {
          FormStore.setValue(`ui.${props.chartOpt.subkey}${props.chartOpt.type}.vizType`, 1, props.prefix)
        }}
      ><FaChartPie />&nbsp;Pie Chart</a>
    ))
    spec.mark = "bar"
    spec.layer = [{
      "mark": {"type": "bar"}
    }]
    spec.view = undefined
    spec.encoding = {
      "x": {"field": "label", "type": "nominal", "sort": "-y", "title": "Type"},
      "y": {"field": "value", "type": "quantitative", "title": "Count"},
    }
    if (props.chartOpt.type == 2) {
      spec.encoding = {
        "x": {
          "field": "date",
          "type": "temporal",
          "title": "Date"
        },
        "y": {
          "aggregate": "count",
          "type": "quantitative"
        },
        "color": {
          "field": "deep",
          "type": "nominal",
          "scale": {
            "domain": ["Regular Algorithms", "Deep Neural Networks"],
            "range": ["#45ef74", "#83b4f5"]
          },
          "title": "Algorithm type"
        }
      }
    }
  }
  if (visualizationType != 2) {
    footeritems.push((
      <a
        key="table"
        className="card-footer-item"
        onClick={() => {
          FormStore.setValue(`ui.${props.chartOpt.subkey}${props.chartOpt.type}.vizType`, 2, props.prefix)
        }}
      ><FaTable />&nbsp;Table</a>
    ))
  }
  
  if (visualizationType == 1) {
    rendered_data = (<Vega spec={spec} data={{values: data}} renderer="svg" downloadFileName={`${props.chartOpt.subkey}`} />)
  } else if (visualizationType == 0) {
    rendered_data = <Vega key="bar" spec={spec} data={{values: data}} renderer="svg" downloadFileName={`${props.chartOpt.subkey}`} />
  } else if (visualizationType == 2) {
    rendered_data = <DataTable data={data} keys={["value", "label"]} />
  }

  // footeritems.push((
  //   <a
  //     key="fullscreen"
  //     className="card-footer-item"
  //     onClick={() => {
  //       FormStore.setValue(`ui.${props.chartOpt.subkey}${props.chartOpt.type}.fullScreen`, !isFullscreen, props.prefix)
  //       return true
  //     }}
  //   >{isFullscreen ? <MdFullscreenExit /> : <MdFullscreen />}&nbsp;{isFullscreen ? "Exit Fullscreen" : "Fullscreen"}</a>
  // ))
  return (
    <>
      <div className={`card`}>
        {
          props.chartOpt.type == 3 ? null : (
            <footer className="card-footer">
            {footeritems}
          </footer>
          )
        }
      </div>
      {rendered_data}
    </>
  )
})

export { XpfwChart }