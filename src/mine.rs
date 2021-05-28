use crate::util::*;
use hashbrown::HashMap;

pub fn count_hits_per_keyword(paper_map: &mut FilteredPaperMap) {
  let mut counts: HashMap<String, usize> = HashMap::new();
  for paper in paper_map.values() {
    for keyword in paper.filter_matches.iter() {
      let mut filter_count = *counts.get(keyword).unwrap_or(&0);
      filter_count += 1;
      counts.insert(keyword.clone(), filter_count);
    }
  }
  println!("Hits are {:?}", counts);
}


pub fn map_to_list(paper_map: &mut FilteredPaperMap) -> PaperList {
  paper_map.values().map(|v| v.clone()).collect()
}

pub fn count_rejected(paper_list: &[PaperEntry]) -> usize {
  let mut counter = 0;
  for paper in paper_list {
    if let Some(a) = paper.rejected_or_unpublished {
      if a == 1 {
        counter += 1;
      }
    }
  }
  counter
}
pub fn count_unpublished(paper_list: &[PaperEntry]) -> usize {
  let mut counter = 0;
  for paper in paper_list {
    if let Some(a) = paper.rejected_or_unpublished {
      if a == 2 {
        counter += 1;
      }
    }
  }
  counter
}

pub fn count_unique_to_this_survey(paper_list: &[PaperEntry]) -> usize {
  let mut counter = 0;
  for paper in paper_list {
    if let Some(surveys) = paper.was_in_survey.as_ref() {
      if surveys.len() == 1 && surveys.contains(&"mag".to_string()) {
        counter += 1;
      }
    }
  }
  counter
}



pub fn only_matches_of(paper_list: &[PaperEntry], to_match: String) -> PaperList {
  paper_list.iter().filter(|w| w.filter_matches.contains(&to_match)).map(|v| v.clone()).collect()
}


pub fn only_secondary(paper_list: &[PaperEntry]) -> PaperList {
  let to_find = "taylor".to_owned();
  paper_list.iter().filter(|w| {
    if let Some(surveys) = w.was_in_survey.as_ref()  {
      if surveys.len() == 1 && surveys.contains(&to_find) {
        return true;
      }
    }
    false
  }).map(|v| v.clone()).collect()
}

pub fn only_without_refs(paper_list: &[PaperEntry]) -> PaperList {
  paper_list.iter().filter(|w| w.references.len() == 0).map(|v| v.clone()).collect()
}

pub fn only_without_citations(paper_list: &[PaperEntry]) -> PaperList {
  paper_list.iter().filter(|w| w.citation_count == 0).map(|v| v.clone()).collect()
}

pub fn only_without_duplicates(paper_list: &[PaperEntry]) -> PaperList {
  let duplicates = only_duplicates(paper_list);
  filter_out(paper_list, &duplicates)
}

pub fn filter_out(paper_list: &[PaperEntry], remove_these: &[PaperEntry])  -> PaperList  {
  paper_list.iter().filter(|w| !remove_these.contains(w)).map(|v| v.clone()).collect()
}

pub fn only_unrleated(paper_list: &[PaperEntry]) -> PaperList {
  let mut psychology = 0;
  let mut patent = 0;
  let mut theory = 0;
  let mut neuro = 0;
  let mut unrelated = 0;
  let res: PaperList = paper_list.iter().filter(|w| {
    if let Some(tags) = w.tags.as_ref() {
      for tag in tags {
        if &tag.to_lowercase() == "psychology" {
          psychology += 1;
          return true;
        }
        if &tag.to_lowercase() == "patent" {
          patent += 1;
          return true;
        }
        if &tag.to_lowercase() == "theory" {
          theory += 1;
          return true;
        }
        if &tag.to_lowercase() == "neurology" {
          neuro += 1;
          return true;
        }
        if &tag.to_lowercase() == "unrelated" {
          unrelated += 1;
          return true;
        }
      }
    }
    false
  }).map(|v| v.clone()).collect();
  println!("Additionally filtered {} psychology {} patents {} theory and {} neuro {} unrelated in total {}", psychology, patent, theory, neuro, unrelated, unrelated +psychology + patent + theory + neuro);
  res
}

pub fn only_paywalled(paper_list: &[PaperEntry]) -> PaperList {
  let mut empty = 0;
  let res: PaperList = paper_list.iter().filter(|w| {
    if let Some(behind_paywall) = w.behind_paywall {
      if let Some(paywall_circumventable) = w.paywall_circumventable {
        return behind_paywall == 1 && paywall_circumventable == 0;
      }
    }
    false
  }).map(|v| v.clone()).collect();

  let res2: PaperList = paper_list.iter().filter(|w| {
    if let Some(tags) = w.tags.as_ref() {
      return tags.len() == 0;
    }
    false
  }).map(|v| v.clone()).collect();
  println!("paywalled and unavailable {} paywalled and no data {}", res.len(), res2.len());
  res2
}

pub fn only_empty_tags(paper_list: &[PaperEntry]) -> PaperList {
  let mut empty = 0;
  let res: PaperList = paper_list.iter().filter(|w| {
    if let Some(tags) = w.tags.as_ref() {
      return tags.len() == 0;
    }
    false
  }).map(|v| v.clone()).collect();
  println!("got empty tags {}", res.len());
  res
}

pub fn only_with_source(paper_list: &[PaperEntry], src_type: u8) -> PaperList {
  let a = "cs".to_owned();
  let b = "oss".to_owned();
  let c = "fake".to_owned();
  paper_list.iter().filter(|w| {
    if let Some(imp) = w.implementation.as_ref() {
      match src_type {
        3 => {
          for im in imp {
            if im.to_lowercase().contains(&c) {
              return true;
            }
          }
        },
        _ => {
          let to_look_for = match src_type {
            1 => &b,
            _ => &a
          };
          for im in imp {
            if &im.to_lowercase() == to_look_for {
              return true;
            }
          }
        }
      }
    }
    false
  }).map(|v| v.clone()).collect()
}

pub fn only_pure(paper_list: &[PaperEntry]) -> PaperList {
  paper_list.iter().filter(|w| {
    if let Some(tags) = w.transfer_experiment_subtype.as_ref() {
      return tags.len() == 1 && tags[0].to_lowercase() == "pure".to_owned();
    }
    false
  }).map(|v| v.clone()).collect()
}

pub fn sort_by_rank(paper_list: &mut PaperList) {
  paper_list.sort_by(|a, b| a.rank.partial_cmp(&b.rank).unwrap())
}

pub fn make_comparable(to_convert: &String) -> String {
  to_convert.to_lowercase().replace(".", "").replace(":", "").replace("-", " ")
}


pub fn only_duplicates(paper_list: &[PaperEntry]) -> PaperList {
  let mut duplicates: PaperList = Vec::new();
  for paper_1 in paper_list {
    for paper_2 in paper_list {
      if let Some(t1) = paper_1.title.as_ref() {
        if let Some(t2) = paper_2.title.as_ref() {
          if make_comparable(&t1) == make_comparable(&t2) {
            if paper_1.id != paper_2.id {
              let duplicate_paper = if paper_1.citation_count > paper_2.citation_count {paper_1.clone()} else {paper_2.clone()};
              if !duplicates.contains(&duplicate_paper) {
                duplicates.push(duplicate_paper);
              }
            }
          }
        }
      }
    }
  }
  let same_title = duplicates.len();
  for paper in paper_list {
    if let Some(a) = paper.duplication_status {
      if a == 3 || a == 4 || a == 7 {
        duplicates.push(paper.clone());
      }
    }
  }
  println!("of {} papers, {} have the exact same title and {} have been found to be duplicated through manual search", paper_list.len(), same_title, duplicates.len() - same_title);
  duplicates
}

pub fn count_without_citations(paper_list: &[PaperEntry]) -> usize {
  let mut citation_count = 0;
  for paper in paper_list {
    if paper.citation_count == 0 { citation_count += 1;}
  }
  citation_count
}

pub fn count_without_reference_data(paper_list: &[PaperEntry]) -> usize {
  let mut without_ref_count = 0;
  for paper in paper_list {
    if paper.references.len() == 0 { without_ref_count += 1;}
  }
  without_ref_count
}


pub fn prepare_transfer_type_csv(paper_list: &[PaperEntry]) {
  let mut transfer_csv = "ID,Name,URL,TransferExperimentType,TransferExperimentSubType,TransferDataType,TransferPerformanceMetrics,RL Implementation,RL Policy Type,Inter-Task Mappings,Autonomous Transfer?,Is Deep RL?,Special Novelty?\n".to_owned();
  for paper in paper_list {
    if let Some(title) = paper.title.as_ref() {
      transfer_csv.push_str(&format!("{},{:?},{:?},[-1],[-1],[-1],[-1],Unknown,-1,-1,-1,Not read yet, Unknown\n", paper.id, title.replace(",", " "), paper.url));
    }
  }
  std::fs::write(format!("{}/transfer_type.csv", DATA_DIR), transfer_csv).unwrap();
}

pub fn count_to_category(paper_list: &[PaperEntry], category_getter: Box<dyn Fn(&PaperEntry) -> Vec<String>>) -> HashMap<String, usize> {
  let mut category_map = HashMap::new();
  for paper in paper_list {
    let paper_cats = category_getter(paper);
    for paper_cat in paper_cats {
      let current_value = if let Some(val) = category_map.get(&paper_cat) {*val} else {0};
      category_map.insert(paper_cat.clone(), current_value + 1);
    }
  }
  category_map
}