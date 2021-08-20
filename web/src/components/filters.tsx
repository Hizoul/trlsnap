import * as React from "react"

import "./form"
import { ExtendedJSONSchema, FormStore, getMapTo, SharedField } from "@xpfw/form"
import { SCHEMA_DATA, PREFIX_FILTERS } from "../data"
import { get, cloneDeep } from "lodash"
import { VscDebugRestart } from "react-icons/vsc"
import BulmaCard from "./card"
import { useList } from "@xpfw/data"
import { observer } from "mobx-react"

const makeToTextFilter = (schema: ExtendedJSONSchema) => {
  const newSchema = cloneDeep(schema)
  newSchema.type = "string"
  return newSchema
}

const DataFilters: React.FunctionComponent<{}> = observer(() => {
  FormStore.setValue(`${getMapTo(SCHEMA_DATA, PREFIX_FILTERS)}.$limit`, 0, undefined)
  const listData = useList(SCHEMA_DATA, PREFIX_FILTERS, undefined, undefined)
  let data: any = []
  if (listData.list != null && listData.list.data != null && listData.list.data.length > 0) {
    data = listData.list.data
  }
  let find_amount = data.length
  return (
    <BulmaCard
      title={`Click / Tap to apply filters to only view a subset of the data\nCurrently filters result in ${find_amount} of 270 entries`}
      mapTo="filters"
      defaultIsOpen={true}
      footer={(
        <a
          className="card-footer-item"
          onClick={() => {FormStore.setValue(PREFIX_FILTERS, {})}}
        >
          <VscDebugRestart />&nbsp;Reset filters
        </a>
      )}
    >
      <div className="columns flex-wrap">
        <div className="column is-one-third-desktop is-full-mobile">
          <SharedField schema={makeToTextFilter(get(SCHEMA_DATA.properties, "author_names"))} mapTo="author_names" prefix={PREFIX_FILTERS} />
        </div>
        <div className="column is-one-third-desktop is-full-mobile">
          <SharedField schema={get(SCHEMA_DATA.properties, "title")} mapTo="title" prefix={PREFIX_FILTERS} />
        </div>
        <div className="column is-one-third-desktop is-full-mobile">
          <SharedField schema={makeToTextFilter(get(SCHEMA_DATA.properties, "implementation"))} mapTo="implementation" prefix={PREFIX_FILTERS} />
        </div>
        <div className="column is-one-third-desktop is-full-mobile">
          <SharedField schema={makeToTextFilter(get(SCHEMA_DATA.properties, "tags"))} mapTo="tags" prefix={PREFIX_FILTERS} />
        </div>
        <div className="column is-one-third-desktop is-full-mobile">
          <SharedField schema={makeToTextFilter(get(SCHEMA_DATA.properties, "transfer_experiment_type"))} mapTo="transfer_experiment_type" prefix={PREFIX_FILTERS} />
        </div>
        <div className="column is-one-third-desktop is-full-mobile">
          <SharedField schema={makeToTextFilter(get(SCHEMA_DATA.properties, "transfer_experiment_subtype"))} mapTo="transfer_experiment_subtype" prefix={PREFIX_FILTERS} />
        </div>
        <div className="column is-one-third-desktop is-full-mobile">
          <SharedField schema={makeToTextFilter(get(SCHEMA_DATA.properties, "transfer_performance_metrics"))} mapTo="transfer_performance_metrics" prefix={PREFIX_FILTERS} />
        </div>
        <div className="column is-one-third-desktop is-full-mobile">
          <SharedField schema={makeToTextFilter(get(SCHEMA_DATA.properties, "allowed_learner"))} mapTo="allowed_learner" prefix={PREFIX_FILTERS} />
        </div>
        <div className="column is-one-third-desktop is-full-mobile">
          <SharedField schema={makeToTextFilter(get(SCHEMA_DATA.properties, "transfer_data_type"))} mapTo="transfer_data_type" prefix={PREFIX_FILTERS} />
        </div>
        <div className="column is-one-third-desktop is-full-mobile">
          <SharedField schema={makeToTextFilter(get(SCHEMA_DATA.properties, "is_deep_rl"))} mapTo="is_deep_rl" prefix={PREFIX_FILTERS} />
        </div>
      </div>
    </BulmaCard>
  )
})

export default DataFilters
