pub mod util;
pub use util::*;
pub mod oag_filter_rdf;
pub use oag_filter_rdf::*;
pub mod mine;
pub use mine::*;
pub mod visualize;
pub use visualize::*;


fn main() {
  println!("Starting main task of oag");
  // filter_trl();
  // fill_data();
  // fill_extra_data();
  // let mut paper_map: FilteredPaperMap = load_serde_file(format!("{}/{}", DATA_DIR, FILTERED_FILE).as_str());
  // count_hits_per_keyword(&mut paper_map);
  // find_id_entries(format!("{}/Papers.nt", DATA_DIR).as_str(), 2097381042);
  
  add_data_from_csv();
  process_data();


}

pub fn filter_trl() {
  println!("STARTING TO Filter data");
  let search_for = vec![
    (vec!["transfer".to_owned(), "reinforcement".to_owned()], "tr".to_owned()),
    (vec!["transfer".to_owned(), "learning".to_owned()], "tl".to_owned()),
    (vec!["transfer".to_owned(), "reinforcement".to_owned(), "learning".to_owned()], "trl".to_owned()),
    (vec!["reinforcement".to_owned(), "learning".to_owned()], "rl".to_owned()),
    (vec!["reinforcement".to_owned(), "learning".to_owned(), "reward".to_owned()], "rlreward".to_owned()),
    (vec!["transfer".to_owned(), "reinforcement".to_owned(), "reward".to_owned()], "trreward".to_owned()),
    (vec!["transfer".to_owned(), "learning".to_owned(), "reward".to_owned()], "tlrreward".to_owned()),
  ];
  oag_filter_keywords(DATA_DIR, search_for.clone()).unwrap();
  let mut paper_map: FilteredPaperMap = load_serde_file(format!("{}/{}", DATA_DIR, FILTERED_FILE).as_str());
  oag_filter_abstracts(DATA_DIR, search_for, &mut paper_map).unwrap();
}

pub fn fill_data() {
  let mut paper_map: FilteredPaperMap = load_serde_file(format!("{}/{}", DATA_DIR, FILTERED_FILE).as_str());
  println!("Filling from Papers.nt");
  oag_fill_from_paperfile(DATA_DIR, &mut paper_map).unwrap();
  println!("Filling Abstracts");
  oag_fill_abstract(DATA_DIR, &mut paper_map).unwrap();
  println!("Filling References");
  oag_fill_references(DATA_DIR, &mut paper_map).unwrap();
  println!("Filling Authors");
  oag_fill_author(DATA_DIR, &mut paper_map).unwrap();
  println!("Filling Field of Study");
  oag_fill_fos(DATA_DIR, &mut paper_map).unwrap();
  println!("Filling URL");
  oag_fill_url(DATA_DIR, &mut paper_map).unwrap();
  println!("Filling LANG");
  oag_fill_lang(DATA_DIR, &mut paper_map).unwrap();
}

pub fn fill_extra_data() {
  let mut paper_map: FilteredPaperMap = load_serde_file(format!("{}/{}", DATA_DIR, FILTERED_FILE).as_str());
  // println!("Creating authors.json");
  // oag_build_author_file(DATA_DIR, &mut paper_map).unwrap();
  // println!("Creating fos.json");
  // oag_build_fos_file(DATA_DIR, &mut paper_map).unwrap();
  println!("Creating journals.json");
  oag_build_journal_file(DATA_DIR, &mut paper_map).unwrap();
  println!("Creating conference-instance.json");
  oag_build_conference_instance_file(DATA_DIR, &mut paper_map).unwrap();
  println!("Creating conference-series.json");
  oag_build_conference_series_file(DATA_DIR, &mut paper_map).unwrap();
}

pub fn process_data() {
  let mut paper_map: FilteredPaperMap = load_serde_file(format!("{}/{}", DATA_DIR, FILTERED_FILE).as_str());
  let mut paper_list: PaperList = map_to_list(&mut paper_map);
  // uncomment when fresh file
  let mut secondary_matches = only_secondary(&paper_list);
  println!("Got {} secondary matches", secondary_matches.len());
  paper_list = only_matches_of(&paper_list, "trl".to_owned());
  let without_citation = only_without_citations(&paper_list);
  let without_ref_data = only_without_refs(&paper_list);
  let duplicates = only_duplicates(&paper_list);
  save_serde_file_pretty(format!("{}/duplicates.json", DATA_DIR).as_str(), &duplicates);
  

  println!("before filtering duplicates: total {}; Without citations {}; Without reference data {}; Duplicates {};", paper_list.len(), without_citation.len(), without_ref_data.len(), duplicates.len());
  paper_list = only_without_duplicates(&paper_list);
  let without_citation = only_without_citations(&paper_list);
  let without_ref_data = only_without_refs(&paper_list);
  let duplicates = only_duplicates(&paper_list);
  println!("after filtering duplicates: total {}; Without citations {}; Without reference data {}; Duplicates {};", paper_list.len(), without_citation.len(), without_ref_data.len(), duplicates.len());
  
  let filtered_out = only_unrleated(&paper_list);
  println!("before last filter {}", paper_list.len());
  paper_list = filter_out(&paper_list, &filtered_out);
  println!("after unrelated filter {}", paper_list.len());
  let uncircumventable = only_paywalled(&paper_list);
  paper_list = filter_out(&paper_list, &uncircumventable);
  println!("after uncircumventable filter {}", paper_list.len());
  let empty_tags = only_empty_tags(&paper_list);
  paper_list = filter_out(&paper_list, &empty_tags);
  println!("after emptytag filter {}", paper_list.len());
  let purerl = only_pure(&paper_list);
  paper_list = filter_out(&paper_list, &purerl);
  println!("after pure RL filter {}", paper_list.len());

  paper_list.append(&mut secondary_matches);

  let without_ref_data = only_without_refs(&paper_list);
  println!("After adding taylor survey entries {} without ref data {}", paper_list.len(), without_ref_data.len());
  let rejected = count_rejected(&paper_list);
  let unpublished = count_unpublished(&paper_list);
  let uniquetosurvey = count_unique_to_this_survey(&paper_list);
  println!("Papers unpublished: {}, Papers rejected: {}, Unique to this survey: {} ({}%)", unpublished, rejected, uniquetosurvey, uniquetosurvey / paper_list.len());
  
  sort_by_rank(&mut paper_list);
  save_serde_file_pretty(format!("{}/sorted.json", DATA_DIR).as_str(), &paper_list);
  save_serde_file_pretty(format!("{}/without_citation.json", DATA_DIR).as_str(), &without_citation);
  save_serde_file_pretty(format!("{}/without_references.json", DATA_DIR).as_str(), &without_ref_data);
  
  let without_ref = filter_out(&paper_list, &without_ref_data);


  gefx_references(&without_ref);
  gefx_references_tags(&without_ref);
  let author_map: AuthorMap = load_serde_file(format!("{}/{}", DATA_DIR, AUTHOR_FILE).as_str());
  let author_list: Vec<Author> = author_map.values().map(|x| x.clone()).collect();
  gefx_authors(&paper_list, &author_list);
  let author_map: FieldOfStudyMap = load_serde_file(format!("{}/{}", DATA_DIR, FOS_FILE).as_str());
  let fos_list: Vec<FieldOfStudy> = author_map.values().map(|x| x.clone()).collect();
  gefx_fos(&paper_list, &fos_list);
  gefx_country(&paper_list);
  std::fs::write(format!("{}/pub_date_vega.json", DATA_DIR), chart_papers_per_date_vega(&paper_list)).unwrap();
  chart_paper_categories(&paper_list);

  // prepare_transfer_type_csv(&paper_list);
}



pub fn add_data_from_csv() {
  use serde_json::from_str;
  let mut paper_map: FilteredPaperMap = load_serde_file(format!("{}/{}", DATA_DIR, FILTERED_FILE).as_str());
  let csv_file = std::fs::read_to_string(format!("{}/papers_transfer_type.csv", DATA_DIR)).expect("CSV-File is readable");
  for line in csv_file.split("\n") {
    let parts_ref = line.split("\\");
    let parts: Vec<&str> = parts_ref.collect();
    let paper_id_opt = parts[0].parse::<usize>();
    if let Ok(paper_id) = paper_id_opt {
      if let Some(paper) = paper_map.get_mut(&paper_id) {
        
        paper.transfer_experiment_type = Some(from_str(parts[3]).expect(&format!("Able to parse transferexperimenttype for Paper {}", paper_id)));
        paper.transfer_experiment_subtype = Some(from_str(parts[4]).expect(&format!("Able to parse transfer_experiment_subtype for Paper {}", paper_id)));
        paper.transfer_data_type = Some(from_str(parts[5]).expect(&format!("Able to parse transfer_data_type for Paper {}", paper_id)));
        paper.transfer_performance_metrics = Some(from_str(parts[6]).expect(&format!("Able to parse transfer_performance_metrics for Paper {}", paper_id)));
        paper.implementation = Some(from_str(parts[8]).expect(&format!("Able to parse implementation for Paper {}", paper_id)));
        paper.policy_type = Some(from_str(parts[9]).expect(&format!("Able to parse policy_type for Paper {}", paper_id)));
        paper.task_mappings = Some(from_str(parts[10]).expect(&format!("Able to parse task_mappings for Paper {}", paper_id)));
        paper.autonomous_transfer = Some(from_str(parts[11]).expect(&format!("Able to parse autonomous_transfer for Paper {}", paper_id)));
        paper.is_deep_rl = Some(from_str(parts[12]).expect(&format!("Able to parse is_deep_rl for Paper {}", paper_id)));
        paper.tags = Some(from_str(parts[14]).expect(&format!("Able to parse tags for Paper {}", paper_id)));
        paper.allowed_learner = Some(from_str(parts[15]).expect(&format!("Able to parse allowed_learner for Paper {}", paper_id)));
        paper.paper_available = Some(from_str(parts[16]).expect(&format!("Able to parse paper_available for Paper {}", paper_id)));
        paper.behind_paywall = Some(from_str(parts[17]).expect(&format!("Able to parse behind_paywall for Paper {}", paper_id)));
        paper.paywall_circumventable = Some(from_str(parts[18]).expect(&format!("Able to parse paywall_circumventable for Paper {}", paper_id)));
        paper.country = Some(from_str(parts[19]).expect(&format!("Able to parse country for Paper {}", paper_id)));
        paper.uni = Some(from_str(parts[20]).expect(&format!("Able to parse uni for Paper {}", paper_id)));
        paper.department = Some(from_str(parts[21]).expect(&format!("Able to parse department for Paper {}", paper_id)));
        paper.source_task_selection = Some(from_str(parts[23]).expect(&format!("Able to parse source_task_selection for Paper {}", paper_id)));
        paper.paper_for_thesis = Some(from_str(parts[22]).expect(&format!("Able to parse paper_for_thesis for Paper {}", paper_id)));
        paper.was_in_survey = Some(from_str(parts[24]).expect(&format!("Able to parse was_in_survey for Paper {}", paper_id)));
        paper.in_title = Some(from_str(parts[25]).expect(&format!("Able to parse in_title for Paper {}", paper_id)));
        paper.in_abs = Some(from_str(parts[26]).expect(&format!("Able to parse in_abs for Paper {}", paper_id)));
        paper.in_contet = Some(from_str(parts[27]).expect(&format!("Able to parse in_contet for Paper {}", paper_id)));

        if parts[30].len() > 0 {
          paper.rejected_or_unpublished = Some(from_str(parts[30]).expect(&format!("Able to parse rejected_or_unpublished for Paper {}", paper_id)));
        }
        if parts[31].len() > 0 {
          paper.duplication_status = Some(from_str(parts[31]).expect(&format!("Able to parse duplicationstatus for Paper {}", paper_id)));
        }
      
      }
    }
  }

  save_serde_file(format!("{}/{}", DATA_DIR, FILTERED_FILE).as_str(), &paper_map);
}