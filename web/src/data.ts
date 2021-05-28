import NedbClient from "@xpfw/data-nedb"
import { BackendClient } from "@xpfw/data"
import { ExtendedJSONSchema, executeForMethods } from "@xpfw/form"
import nedb from "nedb"
NedbClient.instanceCreator = nedb
BackendClient.client = NedbClient

const CURRENT_DATA_VERSION = 5;
const COL_VERSION = "version";
const COL_DATA = "data";
const PREFIX_FILTERS = "data_filters";
const authors = require("./authors.json")

const do_import = async () => {
  const data = require("./sorted.json")
  let promises = []
  for (const entry of data) {
    let currentEntry = await BackendClient.client.get(COL_DATA, entry.id)
    entry._id = entry.id
    if (currentEntry == null) {
      promises.push(BackendClient.client.create(COL_DATA, entry))
    } else {
      promises.push(BackendClient.client.patch(COL_DATA, entry.id, entry))
    }
  }
  await Promise.all(promises)
}

const data_to_nedb = async () => {
  const version = await BackendClient.client.get(COL_VERSION, COL_VERSION);
  if (version == null || version._v != CURRENT_DATA_VERSION) {
    console.log("Importing data")
    await do_import()
    let versionObj = {"_id": COL_VERSION, "_v": CURRENT_DATA_VERSION};
    if (version == null) {
      console.log("Creating versionObj")
      BackendClient.client.create(COL_VERSION, versionObj)
    } else {
      console.log("Patching versionObj")
      BackendClient.client.patch(COL_VERSION, COL_VERSION, versionObj)
    }
  } else {
    console.log("Skipping data reimport")
  }
}

const SCHEMA_NUM = {"type": "number"}
const SCHEMA_STR = {"type": "string"}
const SCHEMA_ARR_NUM = {"type": "array", items: SCHEMA_NUM}
const SCHEMA_ARR_STR = {"type": "array", items: SCHEMA_STR}

const valToRegex = (val: any) => {
  if (val == null || val.length === 0) {
    return undefined
  }
  return {
    $regex: new RegExp(`(.*?)${val}(.*?)`, "ig")
  }
}

const toModify = [
  {name: "title", type: 0},
  {name: "author_names", type: 0},
  {name: "tags", type: 0},
  {name: "implementation", type: 0},
  {name: "is_deep_rl", type: 1}
]
const fullModifier = () => {
  
  return executeForMethods((value: any) => {
    for (const m of toModify) {
      if (value[m.name] != null) {
        switch (m.type) {
          case 0: {
            value[m.name] = valToRegex(value[m.name])
            if (value[m.name] == null) {
              delete value[m.name]
            }
            break
          }
          case 1: {
            let v = value[m.name]
            if (v != null && v !== -1 && v !== "-1") {
              value[m.name] = Number(v)
            }
          }
        }
      }
    }
    return Promise.resolve(value)
  })
}

const SCHEMA_DATA: any = {
  collection: COL_DATA,
  properties: {
    id: {...SCHEMA_NUM, label: "ID"},
    title: {...SCHEMA_STR, label: "Title"},
    abst: {...SCHEMA_STR, label: "Abstract"},
    url: {...SCHEMA_STR, label: "URL"},
    lang: {...SCHEMA_STR, label: "Language"},
    authors: {...SCHEMA_ARR_NUM, label: "Authors"},
    author_names: {...SCHEMA_ARR_STR, label: "Author"},
    fos: {...SCHEMA_ARR_NUM, label: "Field of Study"},
    journals: {...SCHEMA_ARR_NUM, label: "Journals"},
    conferences: {...SCHEMA_ARR_NUM, label: "Conferences"},
    conference_series: {...SCHEMA_ARR_NUM, label: "Conference Series"},
    references: {...SCHEMA_ARR_NUM, label: "References"},
    filter_matches: {...SCHEMA_ARR_STR, label: "Filter Matches"},
    rank: {...SCHEMA_NUM, label: "MAG-Rank"},
    citation_count: {...SCHEMA_NUM, label: "Citation Count"},
    estimated_citation_count: {...SCHEMA_NUM, label: "Estimated Citation Count"},
    publication_date: {...SCHEMA_STR, label: "Publication Date"},
    found_in: {...SCHEMA_NUM, label: "Found in"},
    transfer_experiment_type: {...SCHEMA_ARR_STR, label: "Transfer Experiment Type", format: "select", selectOptions: [
      {label: "No filter", value: ""},
      {label: "Transition Dynamics", value: "t"},
      {label: "Goal position", value: "s_f"},
      {label: "Starting position", value: "s_i"},
      {label: "State", value: "s"},
      {label: "Levels", value: "levels"},
      {label: "Different State Varaibles", value: "v"},
      {label: "Action set", value: "a"},
      {label: "Different number of objects in State", value: "#"},
      {label: "Reward", value: "r"},
      {label: "Different Problem Space", value: "p"},
      {label: "Datasets", value: "datasets"},
    ]},
    transfer_experiment_subtype: {...SCHEMA_ARR_STR, label: "Subtype", format: "select", selectOptions: [
      {label: "No filter", value: ""},
      {label: "Multitask", value: "multi"},
      {label: "Same All", value: "same_all"},
      {label: "Learning intertask transfer", value: "lit"},
      {label: "Different without Mapping", value: "diff-no"},
      {label: "Different with Intertask-Mapping", value: "diff-it"},
      {label: "Simulation to Reality", value: "sim2real"},
      {label: "Pure Reinforcement Learning", value: "pure"},
      {label: "Multiagent", value: "multiagent"},
      {label: "Theory", value: "theory"},
      {label: "Curriculum Learning", value: "curriculum"}
    ]},
    transfer_data_type: {...SCHEMA_ARR_STR, label: "Transferred Data", format: "select", selectOptions: [
      {label: "No filter", value: ""},
      {label: "Action set", value: "a"},
      {label: "Features", value: "fea"},
      {label: "Experience Instances", value: "I"},
      {label: "Task model", value: "model"},
      {label: "Policies", value: "pi"},
      {label: "Partial Policies", value: "pi_p"},
      {label: "Modified / Generated Policies", value: "pi_gen"},
      {label: "Distribution Priors", value: "pri"},
      {label: "Proto-value function", value: "pvf"},
      {label: "Action-value function", value: "Q"},
      {label: "Reward shaping", value: "r"},
      {label: "Rules", value: "rule"},
      {label: "Subtask definitions", value: "sub"},
      {label: "Knowledge Matrices", value: "KM"},
      {label: "Advice", value: "advice"},
      {label: "Rules and Advice", value: "ra"},
      {label: "Options", value: "options"},
      {label: "Advisor", value: "advisor"},
      {label: "Policy Distillation", value: "distil"}
    ]},
    transfer_performance_metrics: {...SCHEMA_ARR_STR, label: "Performance Metrics", format: "select", selectOptions: [
      {label: "No filter", value: ""},
      {label: "Time to Threshold", value: "tt"},
      {label: "Jumpstart", value: "j"},
      {label: "Total reward", value: "tr"},
      {label: "Asympotic Performance", value: "ap"},
    ]},
    implementation: {...SCHEMA_ARR_STR, label: "Implementation"},
    task_mappings: {...SCHEMA_ARR_STR, label: "Task Mappings"},
    tags: {...SCHEMA_ARR_STR, label: "Tags"},
    autonomous_transfer: {...SCHEMA_NUM, label: "Autonomous Transfer"},
    is_deep_rl: {...SCHEMA_STR, label: "Algorithm Type", format: "select", selectOptions: [
      {label: "No filter", value: -1},
      {label: "Regular Algorithms", value: 0},
      {label: "Deep Neural Networks", value: 1},
      {label: "Unknown / Not Applicable", value: 1},
    ]},
    policy_type: {...SCHEMA_ARR_STR, label: "Policy Type"},
    allowed_learner: {...SCHEMA_ARR_STR, label: "Allowed Learners", format: "select", selectOptions: [
      {label: "No filter", value: ""},
      {label: "Temporal Difference", value: "TD"},
      {label: "Hierarchical", value: "H"},
      {label: "Model Based", value: "MB"},
      {label: "Bayesian", value: "B"},
      {label: "Batch", value: "batch"},
      {label: "Relational Reinforcement Learning", value: "RRL"},
      {label: "Policy Search", value: "PS"},
      {label: "Case based reasoning", value: "CBR"},
      {label: "All", value: "all"},
      {label: "Linear Programming", value: "LP"},
      {label: "Any", value: "any"},
    ]},
    country: {...SCHEMA_ARR_STR, label: "Coutry"},
    uni: {...SCHEMA_ARR_STR, label: "University"},
    department: {...SCHEMA_ARR_STR, label: "Department"},
    source_task_selection: {...SCHEMA_ARR_STR, label: "Source Task Selection"},
    was_in_survey: {...SCHEMA_ARR_STR, label: "Was in Survey"},
    in_title: {...SCHEMA_ARR_STR, label: "in title"},
    in_abs: {...SCHEMA_ARR_STR, label: "in abs"},
    in_contet: {...SCHEMA_ARR_STR, label: "in content"},
    rejected_or_unpublished: {...SCHEMA_NUM, label: "rejected or unpublished"},
    duplication_status: {...SCHEMA_NUM, label: "duplication status"},
    paper_for_thesis: {...SCHEMA_NUM, label: "paper for thesis"},
    paper_available: {...SCHEMA_NUM, label: "paper available"},
    behind_paywall: {...SCHEMA_NUM, label: "behind paywall"},
    paywall_circumventable: {...SCHEMA_NUM, label: "paywall circumventable"}
  },
  modify: fullModifier()
}

export {
  data_to_nedb, SCHEMA_DATA, authors, PREFIX_FILTERS, COL_VERSION
}