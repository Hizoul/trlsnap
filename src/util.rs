use serde::{Serialize,Deserialize, de::DeserializeOwned};
use hashbrown::HashMap;

pub const DATA_DIR: &str = "/local/mullermft/0data/mag";
pub const FILTERED_FILE: &str = "filtered_papers.json";
pub const AUTHOR_FILE: &str = "authors.json";
pub const FOS_FILE: &str = "fos.json";
pub const JOURNAL_FILE: &str = "journals.json";
pub const CONFERENCEINSTANCE_FILE: &str = "conference-instance.json";
pub const CONFERENCESERIES_FILE: &str = "conference-series.json";


pub type KeywordSearch = Vec<(Vec<String>, String)>;

pub type FilteredPaperMap = HashMap<usize, PaperEntry>;
pub type PaperList = Vec<PaperEntry>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaperEntry {
  pub id: usize,
  pub title: Option<String>,
  pub abst: Option<String>,
  pub url: Option<String>,
  pub lang: Option<String>,
  pub authors: Vec<usize>,
  pub fos: Vec<usize>,
  pub journals: Vec<usize>,
  pub conferences: Vec<usize>,
  pub conference_series: Vec<usize>,
  pub references: Vec<usize>,
  pub filter_matches: Vec<String>,
  pub rank: i64,
  pub citation_count: i64,
  pub estimated_citation_count: i64,
  pub publication_date: Option<String>,
  pub found_in: usize,
  pub transfer_experiment_type: Option<Vec<String>>,
  pub transfer_experiment_subtype: Option<Vec<String>>,
  pub transfer_data_type: Option<Vec<String>>,
  pub transfer_performance_metrics: Option<Vec<String>>,
  pub implementation: Option<Vec<String>>,
  pub policy_type: Option<Vec<String>>,
  pub task_mappings: Option<Vec<String>>,
  pub autonomous_transfer: Option<i64>,
  pub is_deep_rl: Option<i64>,
  pub tags: Option<Vec<String>>,
  pub allowed_learner: Option<Vec<String>>,
  pub country: Option<Vec<String>>,
  pub uni: Option<Vec<String>>,
  pub department: Option<Vec<String>>,
  pub source_task_selection: Option<Vec<String>>,
  pub was_in_survey: Option<Vec<String>>,
  pub in_title: Option<Vec<String>>,
  pub in_abs: Option<Vec<String>>,
  pub in_contet: Option<Vec<String>>,
  pub rejected_or_unpublished: Option<i64>,
  pub duplication_status: Option<i64>,
  pub paper_for_thesis: Option<i64>,
  pub paper_available: Option<i64>,
  pub behind_paywall: Option<i64>,
  pub paywall_circumventable: Option<i64>,
}

impl PaperEntry {
  pub fn new(id: usize, title: String, filter_matches: Vec<String>) -> PaperEntry {
    PaperEntry {
      id, title: Some(title), abst: None, url: None, lang: None,
      fos: Vec::new(), authors: Vec::new(), references: Vec::new(),
      journals: Vec::new(), conferences: Vec::new(), conference_series: Vec::new(),
      filter_matches,
      citation_count: -1, rank: -1, estimated_citation_count: -1, publication_date: None, found_in: 1,
      transfer_experiment_type: None, transfer_experiment_subtype: None, transfer_data_type: None,
      transfer_performance_metrics: None, implementation: None, policy_type: None,
      task_mappings: None, autonomous_transfer: None, is_deep_rl: None, tags: None,
      allowed_learner: None, country: None, uni: None, department: None, source_task_selection: None,
      was_in_survey: None, in_title: None, in_abs: None, in_contet: None, rejected_or_unpublished: None,
      paper_for_thesis: None, paper_available: None, behind_paywall: None, paywall_circumventable: None,
      duplication_status: None
    }
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
  pub id: usize,
  pub name: Option<String>,
  pub rank: i64,
  pub member_of: Vec<usize>,
  pub citation_count: i64,
  pub paper_count: i64,
  pub publication_date: Option<String>
}

impl Author {
  pub fn new(id: usize) -> Author {
    Author {id, name: None, rank: -1, member_of: Vec::new(), citation_count: -1, paper_count: -1, publication_date: None}
  }
}
pub type AuthorMap = HashMap<usize, Author>;
pub type AuthorList = Vec<Author>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOfStudy {
  pub id: usize,
  pub name: Option<String>,
  pub rank: i64,
  pub level: i64,
  pub citation_count: i64,
  pub paper_count: i64,
  pub publication_date: Option<String>
}

impl FieldOfStudy {
  pub fn new(id: usize) -> FieldOfStudy {
    FieldOfStudy {id, name: None, rank: -1, level: -1, citation_count: -1, paper_count: -1, publication_date: None}
  }
}

pub type FieldOfStudyMap = HashMap<usize, FieldOfStudy>;
pub type FieldOfStudyList = Vec<FieldOfStudy>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
  pub id: usize,
  pub name: Option<String>,
  pub issn: Option<String>,
  pub publisher: Option<String>,
  pub rank: i64,
  pub citation_count: i64,
  pub paper_count: i64,
  pub homepage: Option<String>,
  pub created: Option<String>
}

impl Journal {
  pub fn new(id: usize) -> Journal {
    Journal {id, name: None, issn: None, publisher: None, rank: -1, citation_count: -1, paper_count: -1, homepage: None, created: None}
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConferenceSeries {
  pub id: usize,
  pub name: Option<String>,
  pub rank: i64,
  pub citation_count: i64,
  pub paper_count: i64,
  pub created: Option<String>
}

impl ConferenceSeries {
  pub fn new(id: usize) -> ConferenceSeries {
    ConferenceSeries {id, name: None, rank: -1, citation_count: -1, paper_count: -1, created: None}
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConferenceInstance {
  pub id: usize,
  pub name: Option<String>,
  pub rank: i64,
  pub citation_count: i64,
  pub paper_count: i64,
  pub location: Option<String>,
  pub created: Option<String>
}

impl ConferenceInstance {
  pub fn new(id: usize) -> ConferenceInstance {
    ConferenceInstance {id, name: None, location: None, rank: -1, citation_count: -1, paper_count: -1, created: None}
  }
}

pub type ConferenceInstanceList = Vec<ConferenceInstance>;
pub type ConferenceSeriesList = Vec<ConferenceSeries>;
pub type JournalList = Vec<Journal>;
pub type JournalMap = HashMap<usize, Journal>;
pub type ConferenceSeriesMap = HashMap<usize, ConferenceSeries>;
pub type ConferenceInstanceMap = HashMap<usize, ConferenceInstance>;

pub fn save_serde_file<T: Serialize>(file_name: &str, filtered_list: &T) {
  std::fs::write(file_name, serde_json::to_string(filtered_list).unwrap().as_bytes()).unwrap();
}
pub fn save_serde_file_pretty<T: Serialize>(file_name: &str, filtered_list: &T) {
  std::fs::write(file_name, serde_json::to_string_pretty(filtered_list).unwrap().as_bytes()).unwrap();
}
pub fn load_serde_file<T: DeserializeOwned>(file_name: &str) -> T {
  let file_content = std::fs::read_to_string(file_name).unwrap();
  serde_json::from_str(file_content.as_str()).unwrap()
}

pub const EXPERIMENT_TYPE_STATE_SPACES: &str = "stp";
pub const EXPERIMENT_TYPE_START_STATES: &str = "s_i";
pub const EXPERIMENT_TYPE_GOAL_STATES: &str = "s_f";
pub const EXPERIMENT_TYPE_STATE_VARIABLES: &str = "v";
pub const EXPERIMENT_TYPE_REWARD_FUNC: &str = "r";
pub const EXPERIMENT_TYPE_ACTION_SETS: &str = "a";
pub const EXPERIMENT_TYPE_TRANSITION_FUNCTIONS: &str = "t";
pub const EXPERIMENT_TYPE_AGENTINC: &str = "#";
pub const EXPERIMENT_TYPE_PCG: &str = "pcg";

pub const TRANSFER_SOURCE_TASK_ALL: &str = "all";
pub const TRANSFER_SOURCE_TASK_ONE_HUMAN_SELECTED: &str = "h";
pub const TRANSFER_SOURCE_TASK_LIBRARY: &str = "lib";
pub const TRANSFER_SOURCE_TASK_MODIFIED_FROM_HUMAN: &str = "mod";

pub const TRANSFER_TYPE_ACTION_VALUE_FUNC: &str = "Q";
pub const TRANSFER_TYPE_POLICY: &str = "pi";
pub const TRANSFER_TYPE_TASK_MODEL: &str = "model";
pub const TRANSFER_TYPE_PRIOR_DISTRIBUTIONS: &str = "pri";
pub const TRANSFER_TYPE_PARTIAL_POLICY: &str = "pi_p";
pub const TRANSFER_TYPE_GENERATED_POLICY: &str = "pi_gen";
pub const TRANSFER_TYPE_RULES_OR_ADVICE: &str = "ra";
pub const TRANSFER_TYPE_FEATURES_FOR_LEARNING: &str = "fea";
pub const TRANSFER_TYPE_PROTO_VALUE_FUNCTIONS: &str = "pvf";
pub const TRANSFER_TYPE_REWARD_SHAPING: &str = "R";
pub const TRANSFER_TYPE_SUBTASK_DEF: &str = "sub";
pub const TRANSFER_TYPE_ACTION_SET: &str = "A";
pub const TRANSFER_TYPE_EXPERIENCE_INSTANCES: &str = "I";
pub const TRANSFER_TYPE_WEIGHT: &str = "WT";
pub const TRANSFER_TYPE_WEIGHT_FILTERED: &str = "WTF";
pub const TRANSFER_TYPE_KNOWLEDGE_MATRIX: &str = "KM";
pub const TRANSFER_TYPE_KNOWLEDGE_REPOSITRY: &str = "KR";
pub const TRANSFER_TYPE_ADVISOR: &str = "advisor";
pub const TRANSFER_TYPE_NN_WEIGHT_LINKS: &str = "nn_weight_links";
pub const TRANSFER_TYPE_MEMORY: &str = "RNN/LSTM";
pub const TRANSFER_TYPE_VAE: &str = "VAE";
pub const TRANSFER_TYPE_KNN: &str = "KNN"; // similar to knowledge matrix?
pub const TRANSFER_TYPE_SRL: &str = "SRL";
pub const TRANSFER_TYPE_GAN: &str = "GAN";
pub const TRANSFER_TYPE_SPARSE_CODING: &str = "sparse_coding";


pub const TRANSFER_METRIC_JUMPSTART: &str = "j";
pub const TRANSFER_METRIC_ASYMPTOTIC: &str = "ap";
pub const TRANSFER_METRIC_TOTAL_REWARD: &str = "tr";
pub const TRANSFER_METRIC_TRANSFER_RATIO: &str = "ra";
pub const TRANSFER_METRIC_LEARNING_QUICKER: &str = "tt";
pub const TRANSFER_METRIC_TIME_TO_THRESHOLD: &str = "ttt";

pub const LEARNER_BAYESIAN: &str = "B";
pub const LEARNER_BATCH: &str = "Batch";
pub const LEARNER_CASE_BASED: &str = "CBR";
pub const LEARNER_HIERARCHICAL_VALUE: &str = "H";
pub const LEARNER_LINEAR_PROGRAMMING: &str = "LP";
pub const LEARNER_MODEL_BASED: &str = "MB";
pub const LEARNER_POLICY_SEARCH: &str = "PS";
pub const LEARNER_RELATIONAL_REINFORCEMENT_LEARNING: &str = "RRL";
pub const LEARNER_TEMPORAL_DIFFERENCE: &str = "TD";

pub const INTERTASK_MAP_EXPERIENCE: &str = "exp";
pub const INTERTASK_MAP_NONE: &str = "N/A";
pub const INTERTASK_MAP_ACTION_MAPPING: &str = "Ma";
pub const INTERTASK_MAP_HUMAN_MADE: &str = "sup";
pub const INTERTASK_MAP_GROUPING_OF_STATE: &str = "svg";
pub const INTERTASK_MAP_HIGHER_LEVEL: &str = "T"; // improve


// Paper for thesis: ID of Parent, -2 = Thesis for known Paper, -3 = Thesis but not duplicate for paper
// possible new keywords for paper search: Skill, Knowledge
