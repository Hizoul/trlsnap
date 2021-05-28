import * as React from "react"
import { get } from "lodash"
import { FormStore } from "@xpfw/form"
import BulmaCard from "./card"
import { useList } from "@xpfw/data"
import { MdExpandLess, MdExpandMore } from "react-icons/md"
import { observer } from "mobx-react"
import { SCHEMA_DATA, PREFIX_FILTERS } from "../data"
import { toJS } from "mobx"

const AdditionalInfo: React.FunctionComponent<{data?: any[]}> = observer((props) => {
  const listData = useList(SCHEMA_DATA, PREFIX_FILTERS, undefined, undefined)
  let data: any = get(listData, "list.data", [])
  let count_deep = 0
  let count_nondeep = 0
  let no_experiments = 0
  let behind_paywall = 0
  let paywall_circumventable = 0
  let unreachable = 0
  let phd_theses = 0
  let master_theses = 0
  let paper_for_thesis = 0
  let two_success = 0
  let three_success = 0
  let four_success = 0
  let c = 0
  if (data != null && data.length > 0) {
    for (const entry of data) {
      if (entry.is_deep_rl === 2) {no_experiments++}
      else if (entry.is_deep_rl === 1) {count_deep++}
      else if (entry.is_deep_rl === 0) {count_nondeep++}
      else {console.log("DOESNT BELONG ANYWHERE", toJS(entry))}
      if (entry.behind_paywall == 1) {behind_paywall++}
      if (entry.paywall_circumventable == 1) {paywall_circumventable++}
      if (entry.behind_paywall == 1 && entry.paywall_circumventable == 0) {unreachable++}
      if (entry.behind_paywall == 0 && entry.paywall_circumventable == 1) {
        c++
        console.log("INVALID", c, toJS(entry))
      }
      if (entry.paper_for_thesis == 1) {phd_theses++}
      else if (entry.paper_for_thesis == 2) {master_theses++}
      else if (entry.paper_for_thesis > 2) {paper_for_thesis++}
      if (entry.transfer_performance_metrics != null) {
        if (entry.transfer_performance_metrics.length == 2) {
          two_success++
        } else if (entry.transfer_performance_metrics.length == 3) {
          three_success++
        } else if (entry.transfer_performance_metrics.length == 4) {
          four_success++
        }
      }
    }
  }
  return (
    <BulmaCard
      title="Additional Info"
      mapTo="adinf"
      defaultIsOpen={true}
    >
      <p>
        {count_deep} entries use Deep Neural Networks whereas {count_nondeep} do not require them.
        {no_experiments} are entries that do not contain experiments.
      </p>
      <p>Of the {data.length} entries {phd_theses} are PhD Theses and {master_theses} are Master Theses. {paper_for_thesis} of the entries are papers that belong to one of theses theses.</p>
      <p>When it comes to success metrics, {two_success} entries measured two, {three_success} three and {four_success} four metrics of success.</p>
    </BulmaCard>
  )
})

export default AdditionalInfo
