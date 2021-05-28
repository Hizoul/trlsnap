use std::fs::{File, OpenOptions};
use std::io::{Result, Write, BufRead, BufReader};
use std::time::{Instant};
use crate::util::*;
use hashbrown::HashMap;

const ENTITY_START: &str = "<http://ma-graph.org/entity/";
const TAG_CLOSE: &str = ">";
const LINE_END: &str = "> .";
const URL_START: &str = "<http://purl.org/spar/fabio/hasURL> <";
const RDF_VALUE_START: &str = "\"";
const RDF_VALUE_END: &str = "\"^^<";

pub fn append(path: &str, content: &[u8]) -> std::io::Result<()> {
  OpenOptions::new()
  .write(true)
  .append(true)
  .open(path)?
  .write(content)?;
  Ok(())
}


pub type LineProcessor = dyn Fn(Result<String>);


pub fn check_line_for_keywords(full_line: &str, search_for: &KeywordSearch, paper_map: &mut FilteredPaperMap, found_in: usize) {
  let line_id = rdf_get_entity_id(full_line, false);
  let (start, end) = rdf_get_value_indices(&full_line);
  let to_analyze = &full_line[start..end];
  for searching_for in search_for.iter() {
    let mut does_contain = 0;
    for keyword in searching_for.0.iter() {
      if to_analyze.to_lowercase().contains(keyword) {
        does_contain += 1;
      }
    }
    if does_contain == searching_for.0.len() {
      if paper_map.get(&line_id).is_none() {
        let title = if found_in == 1 {to_analyze.to_owned()} else {String::new()};
        let mut paper_entry = PaperEntry::new(line_id, title, vec![searching_for.1.clone()]);
        paper_entry.found_in = found_in;
        if found_in == 3 {
          paper_entry.abst = Some(to_analyze.to_owned());
        }
        paper_map.insert(line_id, paper_entry);
      } else {
        let paper_entry = paper_map.get_mut(&line_id).unwrap();
        paper_entry.filter_matches.push(searching_for.1.clone());
      }
    }
  }
}

pub fn rdf_get_value_indices(line: &str) -> (usize, usize) {
  let start = line.find(RDF_VALUE_START).expect(&format!("RDF values need to start with a \", not the case for {}", line));
  let end = line.find(RDF_VALUE_END).expect("RDF values need to end with a \"^^<");
  (start + 1, end)
}

pub fn rdf_get_entity_id(line: &str, get_second_id: bool) -> usize {
  let line_to_use = if get_second_id {
    let first_shave_off = &line[ENTITY_START.len()..line.len()];
    let next_start = first_shave_off.find(ENTITY_START).expect("RDF lines need to start with an entity ID \"");
    &first_shave_off[next_start..first_shave_off.len()]
  } else {line};
  let start = line_to_use.find(ENTITY_START).expect("RDF lines need to start with an entity ID \"");
  let end = line_to_use.find(TAG_CLOSE).expect("RDF lines need to start with an entity ID");
  line_to_use[start + ENTITY_START.len() .. end].parse().unwrap()
}

pub fn oag_filter_keywords(dir_path: &str, search_for: KeywordSearch) -> Result<()> {
  let mut paper_map: FilteredPaperMap = HashMap::new();
  let start_process = Instant::now();
  let file = File::open(format!("{}/{}", dir_path, "Papers.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      if line.contains("<http://purl.org/dc/terms/title>") {
        check_line_for_keywords(&line, &search_for, &mut paper_map, 1);
      }
      line.clear();
    }
  }
  save_serde_file(format!("{}/{}", dir_path, FILTERED_FILE).as_str(), &paper_map);
  println!("Spent {} seconds filtering keywords in titles", start_process.elapsed().as_secs());
  Ok(())
}

pub fn oag_filter_abstracts(dir_path: &str, search_for: KeywordSearch, mut paper_map: &mut FilteredPaperMap) -> Result<()> {
  let start_process = Instant::now();
  let file = File::open(format!("{}/{}", dir_path, "PaperAbstractsInvertedIndex.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      if line.contains("<purl.org/dc/terms/abstract>") {
        check_line_for_keywords(&line, &search_for, &mut paper_map, 3);
      }
      line.clear();
    }
  }
  save_serde_file(format!("{}/{}", dir_path, FILTERED_FILE).as_str(), &paper_map);
  println!("Spent {} seconds filtering keywords in abstract", start_process.elapsed().as_secs());
  Ok(())
}

pub fn oag_fill_from_paperfile(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<()> {
  let start_process = Instant::now();
  let file = File::open(format!("{}/{}", dir_path, "Papers.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      if line.contains("<http://purl.org/spar/fabio/hasDiscipline>") {
        let line_id = rdf_get_entity_id(&line, false);
        let paper_opt = paper_map.get_mut(&line_id);
        if paper_opt.is_some() {
          let paper = paper_opt.unwrap();
          let fos_id = rdf_get_entity_id(&line, true);
          if !paper.fos.contains(&fos_id) {
            paper.fos.push(fos_id);
          }
        }
      } else if line.contains("<http://purl.org/dc/terms/creator>") {
        let line_id = rdf_get_entity_id(&line, false);
        let paper_opt = paper_map.get_mut(&line_id);
        if paper_opt.is_some() {
          let paper = paper_opt.unwrap();
          let author_id = rdf_get_entity_id(&line, true);
          if !paper.authors.contains(&author_id) {
            paper.authors.push(author_id);
          }
        }
      } else if line.contains("<http://ma-graph.org/property/citationCount>") {
        let line_id = rdf_get_entity_id(&line, false);
        let paper_opt = paper_map.get_mut(&line_id);
        if paper_opt.is_some() {
          let paper = paper_opt.unwrap();
          let (start, end) = rdf_get_value_indices(&line);
          paper.citation_count = line[start..end].parse().unwrap();
        }
      } else if line.contains("<http://ma-graph.org/property/estimatedCitationCount>") {
        let line_id = rdf_get_entity_id(&line, false);
        let paper_opt = paper_map.get_mut(&line_id);
        if paper_opt.is_some() {
          let paper = paper_opt.unwrap();
          let (start, end) = rdf_get_value_indices(&line);
          paper.estimated_citation_count = line[start..end].parse().unwrap();
        }
      } else if line.contains("<http://ma-graph.org/property/rank>") {
        let line_id = rdf_get_entity_id(&line, false);
        let paper_opt = paper_map.get_mut(&line_id);
        if paper_opt.is_some() {
          let paper = paper_opt.unwrap();
          let (start, end) = rdf_get_value_indices(&line);
          paper.rank = line[start..end].parse().unwrap();
        }
      } else if line.contains("<http://prismstandard.org/namespaces/1.2/basic/publicationDate>") {
        let line_id = rdf_get_entity_id(&line, false);
        let paper_opt = paper_map.get_mut(&line_id);
        if paper_opt.is_some() {
          let paper = paper_opt.unwrap();
          let (start, end) = rdf_get_value_indices(&line);
          paper.publication_date = Some(line[start..end].to_owned());
        }
      } else if line.contains("<http://purl.org/dc/terms/title>") {
        let line_id = rdf_get_entity_id(&line, false);
        let paper_opt = paper_map.get_mut(&line_id);
        if paper_opt.is_some() {
          let paper = paper_opt.unwrap();
          let (start, end) = rdf_get_value_indices(&line);
          paper.title = Some(line[start..end].to_owned());
        }
      } else if line.contains("<http://ma-graph.org/property/appearsInJournal>") {
        let line_id = rdf_get_entity_id(&line, false);
        let paper_opt = paper_map.get_mut(&line_id);
        if paper_opt.is_some() {
          let paper = paper_opt.unwrap();
          let journal_id = rdf_get_entity_id(&line, true);
          if !paper.journals.contains(&journal_id) {
            paper.journals.push(journal_id);
          }
        }
      } else if line.contains("<http://ma-graph.org/property/appearsInConferenceInstance>") {
        let line_id = rdf_get_entity_id(&line, false);
        let paper_opt = paper_map.get_mut(&line_id);
        if paper_opt.is_some() {
          let paper = paper_opt.unwrap();
          let conf_id = rdf_get_entity_id(&line, true);
          if !paper.conferences.contains(&conf_id) {
            paper.conferences.push(conf_id);
          }
        }
      } else if line.contains("<http://ma-graph.org/property/appearsInConferenceSeries>") {
        let line_id = rdf_get_entity_id(&line, false);
        let paper_opt = paper_map.get_mut(&line_id);
        if paper_opt.is_some() {
          let paper = paper_opt.unwrap();
          let conf_series_id = rdf_get_entity_id(&line, true);
          if !paper.conference_series.contains(&conf_series_id) {
            paper.conference_series.push(conf_series_id);
          }
        }
      }
      line.clear();
    }
  }
  save_serde_file(format!("{}/{}", dir_path, FILTERED_FILE).as_str(), paper_map);
  println!("Spent {} seconds filling in Field of Studies", start_process.elapsed().as_secs());
  Ok(())
}

pub fn oag_fill_abstract(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<()> {
  let start_process = Instant::now();
  let file = File::open(format!("{}/{}", dir_path, "PaperAbstractsInvertedIndex.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      let paper_opt = paper_map.get_mut(&line_id);
      if paper_opt.is_some() {
        let paper = paper_opt.unwrap();
        let (start, end) = rdf_get_value_indices(&line);
        paper.abst = Some(line[start..end].to_owned());
      }
      line.clear();
    }
  }
  save_serde_file(format!("{}/{}", dir_path, FILTERED_FILE).as_str(), paper_map);
  println!("Spent {} seconds filling in Abstract", start_process.elapsed().as_secs());
  Ok(())
}

pub fn oag_fill_url(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<()> {
  let start_process = Instant::now();
  let file = File::open(format!("{}/{}", dir_path, "PaperUrls.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      let paper_opt = paper_map.get_mut(&line_id);
      if paper_opt.is_some() {
        let paper = paper_opt.unwrap();
        if let Some(start) = line.find(URL_START) {
          paper.url = Some(line[start + URL_START.len()..line.len()-3].to_owned());
        }
      }
      line.clear();
    }
  }
  save_serde_file(format!("{}/{}", dir_path, FILTERED_FILE).as_str(), paper_map);
  println!("Spent {} seconds filling in Abstract", start_process.elapsed().as_secs());
  Ok(())
}

pub fn oag_build_author_file(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<AuthorMap> {
  let start_process = Instant::now();
  let mut author_map: AuthorMap = HashMap::new();
  paper_map.values().for_each(|paper| {
    for author_id in paper.authors.iter() {
      if author_map.get(author_id).is_none() {
        author_map.insert(*author_id, Author::new(*author_id));
      }
    }
  });
  let file = File::open(format!("{}/{}", dir_path, "Authors.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      let author_opt = author_map.get_mut(&line_id);
      if author_opt.is_some() {
        let author = author_opt.unwrap();
        if line.contains("<http://xmlns.com/foaf/0.1/name>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.name = Some(line[start..end].to_owned());
        } else if line.contains("<http://purl.org/dc/terms/created>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.publication_date = Some(line[start..end].to_owned());
        } else if line.contains("<http://ma-graph.org/property/paperCount>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.paper_count = line[start..end].parse().unwrap();
        } else if line.contains("<http://ma-graph.org/property/citationCount>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.citation_count = line[start..end].parse().unwrap();
        } else if line.contains("<http://ma-graph.org/property/rank>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.rank = line[start..end].parse().unwrap();
        } else if line.contains("<http://www.w3.org/ns/org#memberOf>") {
          let member_id = rdf_get_entity_id(&line, true);
          if !author.member_of.contains(&member_id) {
            author.member_of.push(member_id);
          }
        }
      }
      line.clear();
    }
  }
  std::fs::write(format!("{}/{}", DATA_DIR, AUTHOR_FILE), serde_json::to_string_pretty(&author_map).unwrap().as_bytes()).unwrap();
  println!("Spent {} seconds creating authors.json", start_process.elapsed().as_secs());
  Ok(author_map)
}

pub fn oag_build_fos_file(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<FieldOfStudyMap> {
  let start_process = Instant::now();
  let mut fos_map: FieldOfStudyMap = HashMap::new();
  paper_map.values().for_each(|paper| {
    for fos_id in paper.fos.iter() {
      if fos_map.get(fos_id).is_none() {
        fos_map.insert(*fos_id, FieldOfStudy::new(*fos_id));
      }
    }
  });
  let file = File::open(format!("{}/{}", dir_path, "FieldsOfStudy.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      if line.contains(RDF_VALUE_END) {
        let line_id = rdf_get_entity_id(&line, false);
        if let Some(fos) = fos_map.get_mut(&line_id) {
          let (start, end) = rdf_get_value_indices(&line);
          if line.contains("<http://xmlns.com/foaf/0.1/name>") {
            fos.name = Some(line[start..end].to_owned());
          } else if line.contains("<http://purl.org/dc/terms/created>") {
            fos.publication_date = Some(line[start..end].to_owned());
          } else if line.contains("<http://ma-graph.org/property/paperCount>") {
            fos.paper_count = line[start..end].parse().unwrap();
          } else if line.contains("<http://ma-graph.org/property/citationCount>") {
            fos.citation_count = line[start..end].parse().unwrap();
          } else if line.contains("<http://ma-graph.org/property/rank>") {
            fos.rank = line[start..end].parse().unwrap();
          } else if line.contains("<http://ma-graph.org/property/level>") {
            fos.level = line[start..end].parse().unwrap();
          }
        }
      }
      line.clear();
    }
  }
  std::fs::write(format!("{}/{}", DATA_DIR, FOS_FILE), serde_json::to_string_pretty(&fos_map).unwrap().as_bytes()).unwrap();
  println!("Spent {} seconds creating fos.json", start_process.elapsed().as_secs());
  Ok(fos_map)
}

pub fn oag_build_journal_file(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<JournalMap> {
  let start_process = Instant::now();
  let mut journal_map: JournalMap = HashMap::new();
  paper_map.values().for_each(|paper| {
    for journal_id in paper.journals.iter() {
      if journal_map.get(journal_id).is_none() {
        journal_map.insert(*journal_id, Journal::new(*journal_id));
      }
    }
  });
  let file = File::open(format!("{}/{}", dir_path, "Journals.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      let author_opt = journal_map.get_mut(&line_id);
      if author_opt.is_some() {
        let author = author_opt.unwrap();
        if line.contains("<http://xmlns.com/foaf/0.1/name>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.name = Some(line[start..end].to_owned());
        } else if line.contains("<http://purl.org/dc/terms/created>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.created = Some(line[start..end].to_owned());
        } else if line.contains("<http://ma-graph.org/property/paperCount>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.paper_count = line[start..end].parse().unwrap();
        } else if line.contains("<http://ma-graph.org/property/citationCount>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.citation_count = line[start..end].parse().unwrap();
        } else if line.contains("<http://ma-graph.org/property/rank>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.rank = line[start..end].parse().unwrap();
        } else if line.contains("<http://id.loc.gov/vocabulary/identifiers/issn>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.issn = Some(line[start..end].to_owned());
        } else if line.contains("<http://purl.org/dc/terms/publisher>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.publisher = Some(line[start..end].to_owned());
        }
      }
      line.clear();
    }
  }
  std::fs::write(format!("{}/{}", DATA_DIR, JOURNAL_FILE), serde_json::to_string_pretty(&journal_map).unwrap().as_bytes()).unwrap();
  println!("Spent {} seconds creating journals.json", start_process.elapsed().as_secs());
  Ok(journal_map)
}

pub fn oag_build_conference_instance_file(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<ConferenceInstanceMap> {
  let start_process = Instant::now();
  let mut conf_map: ConferenceInstanceMap = HashMap::new();
  paper_map.values().for_each(|paper| {
    for conf_id in paper.conferences.iter() {
      if conf_map.get(conf_id).is_none() {
        conf_map.insert(*conf_id, ConferenceInstance::new(*conf_id));
      }
    }
  });
  let file = File::open(format!("{}/{}", dir_path, "ConferenceInstances.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      let author_opt = conf_map.get_mut(&line_id);
      if author_opt.is_some() {
        let author = author_opt.unwrap();
        if line.contains("<http://xmlns.com/foaf/0.1/name>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.name = Some(line[start..end].to_owned());
        } else if line.contains("<http://purl.org/dc/terms/created>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.created = Some(line[start..end].to_owned());
        } else if line.contains("<http://ma-graph.org/property/paperCount>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.paper_count = line[start..end].parse().unwrap();
        } else if line.contains("<http://ma-graph.org/property/citationCount>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.citation_count = line[start..end].parse().unwrap();
        } else if line.contains("<http://ma-graph.org/property/rank>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.rank = line[start..end].parse().unwrap();
        }
      }
      line.clear();
    }
  }
  std::fs::write(format!("{}/{}", DATA_DIR, CONFERENCEINSTANCE_FILE), serde_json::to_string_pretty(&conf_map).unwrap().as_bytes()).unwrap();
  println!("Spent {} seconds creating confsinstance.json", start_process.elapsed().as_secs());
  Ok(conf_map)
}

pub fn oag_build_conference_series_file(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<ConferenceSeriesMap> {
  let start_process = Instant::now();
  let mut confs_map: ConferenceSeriesMap = HashMap::new();
  paper_map.values().for_each(|paper| {
    for conf_id in paper.conference_series.iter() {
      if confs_map.get(conf_id).is_none() {
        confs_map.insert(*conf_id, ConferenceSeries::new(*conf_id));
      }
    }
  });
  let file = File::open(format!("{}/{}", dir_path, "ConferenceSeries.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      let author_opt = confs_map.get_mut(&line_id);
      if author_opt.is_some() {
        let author = author_opt.unwrap();
        if line.contains("<http://xmlns.com/foaf/0.1/name>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.name = Some(line[start..end].to_owned());
        } else if line.contains("<http://purl.org/dc/terms/created>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.created = Some(line[start..end].to_owned());
        } else if line.contains("<http://ma-graph.org/property/paperCount>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.paper_count = line[start..end].parse().unwrap();
        } else if line.contains("<http://ma-graph.org/property/citationCount>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.citation_count = line[start..end].parse().unwrap();
        } else if line.contains("<http://ma-graph.org/property/rank>") {
          let (start, end) = rdf_get_value_indices(&line);
          author.rank = line[start..end].parse().unwrap();
        }
      }
      line.clear();
    }
  }
  std::fs::write(format!("{}/{}", DATA_DIR, CONFERENCESERIES_FILE), serde_json::to_string_pretty(&confs_map).unwrap().as_bytes()).unwrap();
  println!("Spent {} seconds creating confss.json", start_process.elapsed().as_secs());
  Ok(confs_map)
}



pub fn oag_fill_references(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<()> {
  let start_process = Instant::now();
  let file = File::open(format!("{}/{}", dir_path, "PaperReferences.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      let paper_opt = paper_map.get_mut(&line_id);
      if paper_opt.is_some() && line.contains("<http://purl.org/spar/cito/cites>") {
        let paper = paper_opt.unwrap();
        let ref_id = rdf_get_entity_id(&line, true);
        if !paper.references.contains(&ref_id) {
          paper.references.push(ref_id);
        }
      }
      line.clear();
    }
  }
  save_serde_file(format!("{}/{}", dir_path, FILTERED_FILE).as_str(), paper_map);
  println!("Spent {} seconds filling in references", start_process.elapsed().as_secs());
  Ok(())
}

pub fn oag_fill_author(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<()> {
  let start_process = Instant::now();
  let file = File::open(format!("{}/{}", dir_path, "PaperAuthorAffiliations.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      let paper_opt = paper_map.get_mut(&line_id);
      if paper_opt.is_some() && line.contains("<http://purl.org/dc/terms/creator>") {
        let paper = paper_opt.unwrap();
        let ref_id = rdf_get_entity_id(&line, true);
        if !paper.authors.contains(&ref_id) {
          paper.authors.push(ref_id);
        }
      }
      line.clear();
    }
  }
  save_serde_file(format!("{}/{}", dir_path, FILTERED_FILE).as_str(), paper_map);
  println!("Spent {} seconds filling in references", start_process.elapsed().as_secs());
  Ok(())
}

pub fn oag_fill_fos(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<()> {
  let start_process = Instant::now();
  let file = File::open(format!("{}/{}", dir_path, "PaperFieldsOfStudy.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      let paper_opt = paper_map.get_mut(&line_id);
      if paper_opt.is_some() && line.contains("<http://purl.org/spar/fabio/hasDiscipline>") {
        let paper = paper_opt.unwrap();
        let ref_id = rdf_get_entity_id(&line, true);
        if !paper.fos.contains(&ref_id) {
          paper.fos.push(ref_id);
        }
      }
      line.clear();
    }
  }
  save_serde_file(format!("{}/{}", dir_path, FILTERED_FILE).as_str(), paper_map);
  println!("Spent {} seconds filling in references", start_process.elapsed().as_secs());
  Ok(())
}

pub fn oag_fill_lang(dir_path: &str, paper_map: &mut FilteredPaperMap) -> Result<()> {
  let start_process = Instant::now();
  let file = File::open(format!("{}/{}", dir_path, "PaperLanguages.nt"))?;
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      let paper_opt = paper_map.get_mut(&line_id);
      if paper_opt.is_some() && line.contains("<http://purl.org/dc/terms/language>") {
        let paper = paper_opt.unwrap();
        let (start, end) = rdf_get_value_indices(&line);
        paper.lang = Some(line[start..end].to_owned());
      }
      line.clear();
    }
  }
  save_serde_file(format!("{}/{}", dir_path, FILTERED_FILE).as_str(), paper_map);
  println!("Spent {} seconds filling in references", start_process.elapsed().as_secs());
  Ok(())
}

pub fn count_lines(file_name: &str) -> Result<usize> {
  let file = File::open(file_name)?;
  let reader = BufReader::new(file);
  let mut line_counter = 0;
  for line_res in reader.lines() {
    if line_res.is_ok() {
      line_counter += 1;
    } else {
      println!("UNABEL TO READ LINE {:?}", line_res);
    }
  }
  Ok(line_counter)
}

pub fn find_id_entries(file_name: &str, id: usize) {
  let start_process = Instant::now();
  let file = File::open(file_name).unwrap();
  let reader = BufReader::new(file);
  let mut line = String::new();
  for line_res in reader.lines() {
    let line_part: String = line_res.unwrap();
    line.push_str(line_part.as_str());
    if line.ends_with(LINE_END) {
      let line_id = rdf_get_entity_id(&line, false);
      if line_id == id {
        println!("{}", line);
      }
      line.clear();
    }
  }
  println!("Spent {} seconds finding entries for ID", start_process.elapsed().as_secs());
}