import React from "react"
import { XpfwTable } from "./components/table"
import { XpfwChart } from "./components/chart"
import DataFilters from "./components/filters"
import { FaDownload, FaGithub, FaInfo, FaTable, FaDatabase, FaRobot, FaTag, FaMapMarkedAlt, FaUniversity, FaFilter } from "react-icons/fa"
import { AiFillExperiment, AiOutlineCloudSync, AiOutlineFieldTime } from "react-icons/ai"
import { CgOrganisation, CgPerformance } from "react-icons/cg"
import { VscCode, VscListOrdered, VscReferences, VscSourceControl } from "react-icons/vsc"
import { BiTransfer } from "react-icons/bi"
import { IoMdArrowRoundBack } from "react-icons/io"
import { ImTable } from "react-icons/im"
import { SCHEMA_DATA, PREFIX_FILTERS } from "./data"
import { get } from "lodash"
import Credits from "./components/credits"
import AdditionalInfo from "./components/additionalInfo"
import DataLoader from "./components/data_loader"
import "./customBulma.scss"
import "./customStyle.scss"
import { observable } from "mobx"
import { observer } from "mobx-react"
import { MdPerson } from "react-icons/md"


const charts_to_show = [
  {
    type: 0, subkey: "transfer_experiment_type", defaultType: 0, icon: VscListOrdered,
    labelConversion: get(SCHEMA_DATA.properties, "transfer_experiment_type.selectOptions", [])
  },
  {
    type: 0, subkey: "transfer_performance_metrics", defaultType: 0, icon: CgPerformance,
    labelConversion: get(SCHEMA_DATA.properties, "transfer_performance_metrics.selectOptions", [])
  },
  {
    type: 0, subkey: "source_task_selection", defaultType: 0, icon: FaDatabase,
    labelConversion: get(SCHEMA_DATA.properties, "source_task_selection.selectOptions", [])
  },
  {
    type: 0, subkey: "task_mappings", defaultType: 0, icon: BiTransfer,
    labelConversion: get(SCHEMA_DATA.properties, "task_mappings.selectOptions", [])
  },
  {
    type: 0, subkey: "allowed_learner", defaultType: 0, icon: FaFilter,
    labelConversion: get(SCHEMA_DATA.properties, "allowed_learner.selectOptions", [])
  },
  {
    type: 0, subkey: "transfer_experiment_subtype", defaultType: 0, icon: AiFillExperiment,
    labelConversion: get(SCHEMA_DATA.properties, "transfer_experiment_subtype.selectOptions", [])
  },
  {
    type: 0, subkey: "transfer_data_type", defaultType: 2, icon: AiOutlineCloudSync,
    labelConversion: get(SCHEMA_DATA.properties, "transfer_data_type.selectOptions", [])
  },
  {type: 0, subkey: "tags", defaultType: 2, icon: FaTag,},
  {type: 0, subkey: "implementation", defaultType: 2, icon: VscCode,},
  {type: 0, subkey: "policy_type", defaultType: 2, icon: FaRobot,},
  {type: 0, subkey: "country", defaultType: 2, icon: FaMapMarkedAlt,},
  {type: 0, subkey: "uni", defaultType: 2, icon: FaUniversity,},
  {type: 2, subkey: "publication_date", defaultType: 0, fullWidth: 1, icon: AiOutlineFieldTime,},
  {type: 0, subkey: "department", defaultType: 2, icon: CgOrganisation,},
  {type: 1, subkey: "authors", defaultType: 2, icon: MdPerson,},
]

const selection = observable.box("home")

const goHome = () => {selection.set("home")}
const Welcome: React.FunctionComponent<{title?: string}> = (props) => {
  if (props.title != null) {
    return (
      <>
        <section className="hero is-primary has-text-centered">
          <div className="hero-body">
            <p className="subtitle is-size-3"><span onClick={goHome}><IoMdArrowRoundBack/></span> {props.title}</p>
          </div>
        </section>
      </>
    )
  }
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

const Footer = () => {
  return (
    <footer className="card-footer">
      <a href="https://github.com/hizoul/tlsnap" className="card-footer-item"><FaDownload />&nbsp;Dataset</a>
      <a href="https://github.com/hizoul/tlsnap" className="card-footer-item"><VscSourceControl />&nbsp;Source code</a>
    </footer>
  )
}

const BoxLink: React.FunctionComponent<{text: string, icon: any, link: string}> = (props) => {
  return (
    <a
      className="box btn-box column"
      onClick={() => {
        selection.set(props.link)
      }}
    >
      <div className="bigicn">{props.icon}</div>
      {props.text}
    </a>
  )
}

const ReactApp: React.FunctionComponent<{}> = observer(() => {
  if (selection.get() == "table") {
    return (
      <XpfwTable
        schema={SCHEMA_DATA}
        mapTo={PREFIX_FILTERS}
        keys={[ "tags",
          "title", "authors", "publication-date", "transfer_experiment_type",
          "transfer_experiment_subtype", "transfer_data_type", "transfer_performance_metrics",
          "implementation", "task_mappings",  "policy_type", "allowed_learner",
          "source_task_selection",
          "is_deep_rl", "country", "uni", "department", 
          "rejected_or_unpublished",   "lang"]}
        container={(props) => (
          <>
            <Welcome title={"Spreadsheet"} />
            <div className="card">
              <div className="card-header">
                <p className="card-header-title">
                  Found {props.amount} entries for the selected filters
                </p>
              </div>
              {props.children}
              <Footer />
            </div>
          </>
        )}
      />
    )
  } else if (selection.get() == "info") {
    return (
      <>
        <Welcome title="Info" />
        <AdditionalInfo />
      </>
    )
  } else if (selection.get() == "credits") {
    return (
      <>
        <Welcome title="Credits" />
        <Credits />
      </>
    )
  }
  let current_selection = selection.get()
  for (const chart_config of charts_to_show) {
    if (current_selection == chart_config.subkey) {
      return (
        <>
          <Welcome title={chart_config.subkey} />
          <XpfwChart
            schema={SCHEMA_DATA}
            mapTo={PREFIX_FILTERS}
            chartOpt={chart_config}
          />
        </>
      )
    }
  }

  let chart_links = []
  
  for (const chart_config of charts_to_show) {
    let Icon = chart_config.icon
    chart_links.push(
      <BoxLink text={chart_config.subkey} icon={<Icon />} link={chart_config.subkey} />
    )
  }

  return (
    <>
      <Welcome />
      <p>&nbsp;</p>
      <p>&nbsp;</p>
      <div className="columns is-multiline is-mobile">
        <a
          href="https://github.com/hizoul/tlsnap"
          target="blank"
          className="box btn-box column is-one-third-desktop is-half-mobile"
        >
          <div className="bigicn"><FaGithub /></div>
          Source Code
        </a>
        <BoxLink text="Spreadsheet" icon={<ImTable />} link="table" />
        <BoxLink text="Credits" icon={<VscReferences />} link="credits" />
        <BoxLink text="Information" icon={<FaInfo />} link="info" />
        {chart_links}
        <div className="column is-full">&nbsp;</div>
      </div>
    </>
  )
})

const AppWrapper: React.FunctionComponent<{}> = () => {
  return (
    <>
      <DataLoader toRender={ReactApp} />
    </>
  )
}

export default AppWrapper