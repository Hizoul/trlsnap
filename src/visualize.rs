use crate::util::*;
use crate::mine::*;
use hashbrown::HashMap;

pub fn generate_gefx(edges: Vec<(usize, usize)>, nodes: Vec<(usize, String)>) -> String {
  let mut edges_string = String::new();
  let mut nodes_string = String::new();
  for node in nodes {
    nodes_string.push_str(format!("<node id=\"{}\" label=\"{}\" />", node.0, node.1).as_str());
  }
  for (index, edge) in edges.iter().enumerate() {
    edges_string.push_str(format!("<edge id=\"{}\" source=\"{}\" target=\"{}\" />", index, edge.0, edge.1).as_str());
  }
  format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<gexf xmlns="http://www.gexf.net/1.2draft" version="1.2">
    <meta lastmodifieddate="2009-03-20">
        <creator>Hizoul</creator>
        <description>GEFX File created by tlsurvey tool</description>
    </meta>
    <graph mode="static" defaultedgetype="directed">
        <attributes class="node">
            <attribute id="0" title="url" type="string"/>
            <attribute id="1" title="indegree" type="float"/>
            <attribute id="2" title="frog" type="boolean">
                <default>true</default>
            </attribute>
        </attributes>
        <nodes>
            {}
        </nodes>
        <edges>
            {}
        </edges>
    </graph>
</gexf>  
"#, nodes_string, edges_string)
}

pub fn gefx_references(paper_list: &PaperList) {
  let mut edges: Vec<(usize, usize)> = Vec::new();
  let mut nodes: Vec<(usize, String)> = Vec::new();
  for paper in paper_list {
    if let Some(title) = &paper.title {
      nodes.push((paper.id, title.clone()));
    }
    for reference in paper.references.iter() {
      let mut is_in_dataset = false;
      'find_in_set: for second_paper in paper_list {
        if second_paper.id == *reference {
          is_in_dataset = true;
          break 'find_in_set;
        }
      }
      if is_in_dataset {
        edges.push((paper.id, *reference));
      }
    }
  }
  std::fs::write(format!("{}/paper_network.gexf", DATA_DIR), generate_gefx(edges, nodes).as_bytes()).unwrap();
}

pub fn gefx_references_tags(paper_list: &PaperList) {
  let mut edges: Vec<(usize, usize)> = Vec::new();
  let mut nodes: Vec<(usize, String)> = Vec::new();
  for paper in paper_list {
    if let Some(tags) = &paper.tags {
      nodes.push((paper.id, format!("{:?}", tags).replace("\"", "")));
    }
    for reference in paper.references.iter() {
      let mut is_in_dataset = false;
      'find_in_set: for second_paper in paper_list {
        if second_paper.id == *reference {
          is_in_dataset = true;
          break 'find_in_set;
        }
      }
      if is_in_dataset {
        edges.push((paper.id, *reference));
      }
    }
  }
  std::fs::write(format!("{}/paper_tags_network.gexf", DATA_DIR), generate_gefx(edges, nodes).as_bytes()).unwrap();
}

pub fn gefx_authors(paper_list: &[PaperEntry], author_list: &[Author]) {
  let mut only_include_these_authors: Vec<usize> = Vec::new();
  let mut paper_name: HashMap<usize, String> = HashMap::new();
  let mut edges: Vec<(usize, usize)> = Vec::new();
  let mut nodes: Vec<(usize, String)> = Vec::new();
  for paper in paper_list {
    for reference in paper.references.iter() {
      let mut second_paper_opt: Option<&PaperEntry> = None;
      'find_in_set: for second_paper in paper_list {
        if second_paper.id == *reference {
          second_paper_opt = Some(second_paper);
          break 'find_in_set;
        }
      }
      if let Some(second_paper) = second_paper_opt {
        for author_id in paper.authors.iter() {
          if !only_include_these_authors.contains(author_id) {
            only_include_these_authors.push(*author_id);
            paper_name.insert(*author_id, paper.title.as_ref().unwrap_or(&"".to_owned()).clone());
          }
          for sec_author_id in second_paper.authors.iter() {
            edges.push((*author_id, *sec_author_id));
          }
        }
      }
    }
  }
  for author in author_list {
    if only_include_these_authors.contains(&author.id) {
      if let Some(name) = author.name.as_ref() {
        nodes.push((author.id, format!("{}\n;{}", name, paper_name.get(&author.id).unwrap_or(&"".to_owned()))));
      }
    }
  }
  std::fs::write(format!("{}/author_network.gexf", DATA_DIR), generate_gefx(edges, nodes).as_bytes()).unwrap();
}

pub fn gefx_fos(paper_list: &[PaperEntry], fos_list: &[FieldOfStudy]) {
  let mut only_include_these_fos: Vec<usize> = Vec::new();
  let mut edges: Vec<(usize, usize)> = Vec::new();
  let mut nodes: Vec<(usize, String)> = Vec::new();
  for paper in paper_list {
    for reference in paper.references.iter() {
      let mut second_paper_opt: Option<&PaperEntry> = None;
      'find_in_set: for second_paper in paper_list {
        if second_paper.id == *reference {
          second_paper_opt = Some(second_paper);
          break 'find_in_set;
        }
      }
      if let Some(second_paper) = second_paper_opt {
        for fos_id in paper.fos.iter() {
          if !only_include_these_fos.contains(fos_id) {
            only_include_these_fos.push(*fos_id);
          }
          for sec_fos_id in second_paper.fos.iter() {
            edges.push((*fos_id, *sec_fos_id));
          }
        }
      }
    }
  }
  for fos in fos_list {
    if only_include_these_fos.contains(&fos.id) {
      if let Some(name) = fos.name.as_ref() {
        nodes.push((fos.id, name.clone()));
      }
    }
  }
  std::fs::write(format!("{}/fos_network.gexf", DATA_DIR), generate_gefx(edges, nodes).as_bytes()).unwrap();
}

use fasthash::{metro, MetroHasher};

pub fn gefx_country(paper_list: &[PaperEntry]) {
  let mut edges: Vec<(usize, usize)> = Vec::new();
  let mut nodes: Vec<(usize, String)> = Vec::new();
  for paper in paper_list {
    if let Some(countries) = &paper.country {
      for country in countries.iter() {
        let country_id = metro::hash64(country.as_bytes()) as usize;
        let node_entry = (country_id as usize, country.clone());
        if !nodes.contains(&node_entry) {
          nodes.push(node_entry.clone());
        }

        for reference in paper.references.iter() {
          let mut second_paper_opt: Option<&PaperEntry> = None;
          'find_in_set: for second_paper in paper_list {
            if second_paper.id == *reference {
              second_paper_opt = Some(second_paper);
              break 'find_in_set;
            }
          }
          if let Some(second_paper) = second_paper_opt {
            if let Some(second_countries) = &second_paper.country {
              for second_country in second_countries.iter() {
                let second_country_id = metro::hash64(second_country.as_bytes());
                edges.push((country_id, second_country_id as usize));
              }
            }
          }
        }
      }
    }
  }
  std::fs::write(format!("{}/country_network.gexf", DATA_DIR), generate_gefx(edges, nodes).as_bytes()).unwrap();
}

pub fn chart_papers_per_date_vega(paper_list: &PaperList) -> String {
  let mut date_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
  for paper in paper_list {
    if let Some(paper_date) = paper.publication_date.as_ref() {
      let date = paper_date[0..7].to_owned();
      let current_value = {
        let map_entry = date_map.get(&date);
        if map_entry.is_none() {
          0
        } else {
          *map_entry.unwrap()
        }
      };
      date_map.insert(date, current_value + 1);
    }
  }
  let mut value_string = String::new();
  for (k, v) in date_map {
    value_string.push_str(&format!(r##"{{"name": "{}", "value": {}}},"##, k, v))
  }
  let spec = format!(r##"
  {{
    "$schema": "https://vega.github.io/schema/vega-lite/v4.json",
    "title": "Amount of TRL Publications over time",
    "selection": {{
      "highlight": {{"type": "single", "empty": "none", "on": "mouseover"}},
      "select": {{"type": "multi"}},
      "grid": {{
        "type": "interval", "bind": "scales"
      }}
    }},
    "mark": {{
      "type": "bar",
      "fill": "#4C78A8",
      "cursor": "pointer"
    }},
    "background": "white",
    "width": 500,
    "data": {{
      "values": [
        {}
      ]
    }},
    "encoding": {{
      "x": {{"field": "name", "type": "temporal", "title": "Year"}},
      "y": {{"field": "value", "type": "quantitative", "title": "Papers published"}},
      "fillOpacity": {{
        "condition": {{"selection": "select", "value": 1}},
        "value": 0.3
      }}
    }}
  }}
  "##, value_string[0..value_string.len()-1].to_owned());
  spec
}

pub fn chart_pie_vega(title: &str, pie_values: &[(String, usize)]) -> String {
  let mut value_string = String::new();
  for (k, v) in pie_values {
    value_string.push_str(&format!(r##"{{"name": "{}", "value": {}}},"##, k, v))
  }
  let spec = format!(r##"
  {{
    "$schema": "https://vega.github.io/schema/vega-lite/v4.json",
    "title": "{}",
    "selection": {{
      "highlight": {{"type": "single", "empty": "none", "on": "mouseover"}},
      "select": {{"type": "multi"}}
    }},
    "width": 500,
    "data": {{
      "values": [
        {}
      ]
    }},
    "encoding": {{
      "theta": {{"field": "value", "type": "quantitative", "stack": true}},
      "color": {{
        "field": "name", "type": "nominal"
      }}
    }},
    "layer": [{{
      "mark": {{"type": "arc", "outerRadius": 170}}
    }}, {{
      "mark": {{"type": "text", "radius": 185}},
      "encoding": {{
        "text": {{"field": "value", "type": "nominal"}}
      }}
    }}],
    "view": {{"stroke": null}}
  }}
  "##, title, value_string[0..value_string.len()-1].to_owned());
  spec
}

pub fn chart_bar_vega(title: &str, pie_values: &[(String, usize)]) -> String {
  let mut value_string = String::new();
  for (k, v) in pie_values {
    value_string.push_str(&format!(r##"{{"name": "{}", "value": {}}},"##, k, v))
  }
  let spec = format!(r##"
  {{
    "$schema": "https://vega.github.io/schema/vega-lite/v4.json",
    "title": "{}",
    "selection": {{
      "highlight": {{"type": "single", "empty": "none", "on": "mouseover"}},
      "select": {{"type": "multi"}}
    }},
    "width": 500,
    "data": {{
      "values": [
        {}
      ]
    }},
    "encoding": {{
      "x": {{"field": "name", "type": "nominal", "sort": "-y"}},
      "y": {{
        "field": "value", "type": "quantitative"
      }}
    }},
    "mark": {{
      "type": "bar",
      "fill": "#4C78A8",
      "cursor": "pointer"
    }},
    "view": {{"stroke": null}}
  }}
  "##, title, value_string[0..value_string.len()-1].to_owned());
  spec
}
pub fn category_to_sorted_tuple(map: &HashMap<String, usize>) -> Vec<(String, usize)> {
  let mut tuples: Vec<(String, usize)> = map.iter().map(|x| (x.0.clone(), *x.1)).collect();
  tuples.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
  tuples
}

pub fn summarize_others(list: &mut HashMap<String, usize>, limit: usize) {
  let mut others_total = 0;
  let mut to_delete = Vec::new();
  for (key, value) in list.iter() {
    if *value <= limit {
      to_delete.push(key.clone());
      others_total += 1;
    }
  }
  for delete in to_delete {
    list.remove(&delete);
  }
  list.insert("Others".to_owned(), others_total);
}

pub fn clean_tags(list: Vec<String>) -> Vec<String> {
  list.iter().map(|x| x.replace("-", "").replace(" ", "").to_lowercase()).collect()
}

pub fn filter_str_val_map(list: &mut HashMap<String, usize>, to_filter: &[&str], must_contain: bool) {
  let mut to_remove = Vec::new();
  for key in list.keys() {
    let lower = key.to_lowercase();
    let contains = to_filter.contains(&lower.as_str());
    if must_contain && contains || !must_contain && !contains {
      to_remove.push(key.to_owned());
    }
  }
  for r in to_remove {
    list.remove(&r).expect("REMOVED");
  }
}

pub const CLASSIC_CONTROL_TAGS: [&str; 2] = ["", ""];

pub fn chart_paper_tags(paper_list: &[PaperEntry]) {

  let mut tag_counts = count_to_category(paper_list, Box::new(move |x| {
    if let Some(tags) = x.tags.as_ref() {
      clean_tags(tags.clone())
    } else {
      vec![]
    }
  }));
  std::fs::write(format!("{}/vega-tags-all.json", DATA_DIR),
    chart_pie_vega("Publications by Tag", &category_to_sorted_tuple(&tag_counts))).unwrap();
  let mut t2 = tag_counts.clone();
  let to_filter_for_applications = vec![
    "energysaving", "imitation", "wifi", "lifelong", "collision", "demonstration",
    "adaptive", "uav", "successor", "avoidance", "autonomous", "2v1", "survey",
    "human", "collect", "obstacle", "smartenergy", "montecarlo", "psychology",
    "4v3", "2dto3d", "theory", "3v2", "cnn", "maze", "3d", "multiagent", "simulation", "realworld",
    "grid", "drone", "network", "2d", "efficiency", "Others", "uci", "datatransfer",
    "knowledgegraph", "memorymatrix", "joints", "textbased", "networks","icub",
    "trading", "theory", "kilobot", "rccar", "homing", "roadnetwork", "siamese", "target", "theoremproving",
    "target", "cost", "lrf", "ddpg", "legs", "deadline", "episodicmemory", "ntu",
    "climbing", "hose", "rogue", "influence", "firstorderlogic", "indoornetworkselection", "heat",
    "palpation", "inputabstraction", "drawing", "kinematic", "boat", "car", "guidance",
    "reallife",
    // counts to games
    "flappybird",
    // robotics
    "icub",
  ];
  let filtered_t2 = t2.iter().filter(|x| {
    !to_filter_for_applications.contains(&x.0.as_str())
  }).map(|x| (x.0.clone(), *x.1)).collect();
  std::fs::write(format!("{}/vega-tags-applications.json", DATA_DIR),
    chart_bar_vega("Practical Application Fields", &category_to_sorted_tuple(&filtered_t2))).unwrap();
  

  fn make_sub_counts(paper_list: &[PaperEntry], must_contain: &[String]) -> HashMap<String, usize> {
    let to_contain = must_contain.to_vec();
    count_to_category(paper_list, Box::new(move |x| {
      if let Some(tags) = x.tags.as_ref() {
        let mut contains = 0;
        for tag in tags {
          for sub_tag in to_contain.iter() {
            if &tag.to_owned().to_lowercase() == sub_tag {
              contains += 1;
            }
          }
          if contains == to_contain.len() {
            return tags.clone();
          }
        }
      }
      vec![]
    }))
  }
  let robotics_apps = make_sub_counts(paper_list, &["robotics".to_owned()]);
  let robotics_nav = make_sub_counts(paper_list, &["robotics".to_owned(), "navigation".to_owned()]);

  println!("Got robotics tags {:?} robonav {:?}", robotics_apps.len(), robotics_nav.len());

  std::fs::write(format!("{}/vega-tags-robotics.json", DATA_DIR),
    chart_bar_vega("Robotics Tags", &category_to_sorted_tuple(&robotics_apps))).unwrap();
    std::fs::write(format!("{}/vega-tags-robocup.json", DATA_DIR),
    chart_bar_vega("Robocup Tags", &category_to_sorted_tuple(&make_sub_counts(paper_list, &["robocup".to_owned()])))).unwrap();
  std::fs::write(format!("{}/vega-tags-navigation.json", DATA_DIR),
  chart_bar_vega("Navigation Tags", &category_to_sorted_tuple(&make_sub_counts(paper_list, &["navigation".to_owned()])))).unwrap();
  std::fs::write(format!("{}/vega-tags-games.json", DATA_DIR),
  chart_bar_vega("Games Tags", &category_to_sorted_tuple(&make_sub_counts(paper_list, &["games".to_owned()])))).unwrap();
}

pub fn chart_paper_categories(paper_list: &[PaperEntry]) {
  use crate::mine::count_to_category;
  let fos_map: FieldOfStudyMap = load_serde_file(&format!("{}/{}", DATA_DIR, FOS_FILE));
  let fos_counts = count_to_category(paper_list, Box::new(move |x| x.fos.iter().map(|y| fos_map.get(y).unwrap().name.as_ref().unwrap().clone()).collect()));
  // let fos_tuples = category_to_sorted_tuple(&fos_counts);
  // println!("FOS COUNTS {:?}", fos_tuples);
  let lang_counts = count_to_category(paper_list, Box::new(move |x| vec![x.lang.as_ref().unwrap_or(&"Unknown".to_owned()).clone()]));
  // let lang_tuples = category_to_sorted_tuple(&lang_counts);
  // println!("lang COUNTS {:?}", lang_tuples);
  let journal_map: JournalMap = load_serde_file(&format!("{}/{}", DATA_DIR, JOURNAL_FILE));
  let journal_counts = count_to_category(paper_list, Box::new(move |x: &PaperEntry| x.journals.iter().map(|y| {
    journal_map.get(y).unwrap_or(&Journal::new(*y)).name.as_ref().unwrap_or(&"undefined".to_owned()).clone()
  }).collect()));
  let journal_tuples = category_to_sorted_tuple(&journal_counts);
  let journals_more_than_one: Vec<(String, usize)> = journal_tuples.iter().filter(|x| x.1 > 1).map(|x| x.clone()).collect::<Vec<(String, usize)>>();
  let journals_exactly_one: Vec<(String, usize)> = journal_tuples.iter().filter(|x| x.1 == 1).map(|x| x.clone()).collect::<Vec<(String, usize)>>();
  


  let tet_tuples = count_to_category(paper_list, Box::new(move |x| x.transfer_experiment_type.as_ref().unwrap_or(&vec![]).clone()));
  std::fs::write(format!("{}/vega-transfer-experiment-type.json", DATA_DIR),
    chart_pie_vega("Experiment Types", &category_to_sorted_tuple(&tet_tuples))).unwrap();
    
  let test_tuples = count_to_category(paper_list, Box::new(move |x| x.transfer_experiment_subtype.as_ref().unwrap_or(&vec![]).clone()));
  std::fs::write(format!("{}/vega-transfer-experiment-subtype.json", DATA_DIR),
    chart_pie_vega("Experiment Subtypes", &category_to_sorted_tuple(&test_tuples))).unwrap();
  let tdt_tuples = count_to_category(paper_list, Box::new(move |x| x.transfer_data_type.as_ref().unwrap_or(&vec![]).clone()));
  
  std::fs::write(format!("{}/vega-transfer-data-type.json", DATA_DIR),
    chart_pie_vega("Transferred Data Types", &category_to_sorted_tuple(&tdt_tuples))).unwrap();
  let tpm_tuples = count_to_category(paper_list, Box::new(move |x| x.transfer_performance_metrics.as_ref().unwrap_or(&vec![]).clone()));
  std::fs::write(format!("{}/vega-transfer-performance-metric.json", DATA_DIR),
    chart_pie_vega("Experiment Types", &category_to_sorted_tuple(&tpm_tuples))).unwrap();
  let tpt_tuples = count_to_category(paper_list, Box::new(move |x| x.policy_type.as_ref().unwrap_or(&vec![]).clone()));
  std::fs::write(format!("{}/vega-policy-type.json", DATA_DIR),
    chart_pie_vega("RL Policy Types", &category_to_sorted_tuple(&tpt_tuples))).unwrap();
  let tm_tuples = count_to_category(paper_list, Box::new(move |x| x.task_mappings.as_ref().unwrap_or(&vec![]).clone()));
  std::fs::write(format!("{}/vega-task-mappings.json", DATA_DIR),
    chart_pie_vega("Task Mappings", &category_to_sorted_tuple(&tm_tuples))).unwrap();
  let sts_tuples = count_to_category(paper_list, Box::new(move |x| x.source_task_selection.as_ref().unwrap_or(&vec![]).clone()));
  std::fs::write(format!("{}/vega-transfer-sour-task-selection.json", DATA_DIR),
    chart_pie_vega("Source task selection type", &category_to_sorted_tuple(&sts_tuples))).unwrap();
  let al_tuples = count_to_category(paper_list, Box::new(move |x| x.allowed_learner.as_ref().unwrap_or(&vec![]).clone()));
  std::fs::write(format!("{}/vega-allowed-learner.json", DATA_DIR),
    chart_pie_vega("Experiment Types", &category_to_sorted_tuple(&al_tuples))).unwrap();
  
  let mut imp_tuples = count_to_category(paper_list, Box::new(move |x| x.implementation.as_ref().unwrap_or(&vec![]).clone()));
  let imp_filters = vec!["custom", "cs", "oss", "figures", "figure", "formula", "formulas", "pseudo", "table", "tables", "video", "videos", "theorem", "lemma", "dead", "web", "github", "github/haarnoja/sac", "jetsontx2", "fakeoss"];
  let mut filteredimp = imp_tuples.clone();
  filter_str_val_map(&mut filteredimp, &imp_filters, true);
  std::fs::write(format!("{}/vega-implementation.json", DATA_DIR),
    chart_pie_vega("Experiment Types", &category_to_sorted_tuple(&filteredimp))).unwrap();
    let portray_filters = vec!["figures", "formulas", "pseudo","tables", "videos", "theorem", "lemma", "dead", "web"];
  let mut portray_papers = imp_tuples.clone();
  filter_str_val_map(&mut portray_papers, &portray_filters, false);
  std::fs::write(format!("{}/vega-paper-portray.json", DATA_DIR),
    chart_pie_vega("Experiment Types", &category_to_sorted_tuple(&portray_papers))).unwrap();



  let uni_tuples = count_to_category(paper_list, Box::new(move |x| x.uni.as_ref().unwrap_or(&vec![]).clone()));
  std::fs::write(format!("{}/vega-unis.json", DATA_DIR),
    chart_pie_vega("Universities", &category_to_sorted_tuple(&uni_tuples))).unwrap();
  let dep_tuples = count_to_category(paper_list, Box::new(move |x| x.department.as_ref().unwrap_or(&vec![]).clone()));
  std::fs::write(format!("{}/vega-departments.json", DATA_DIR),
    chart_pie_vega("Departments", &category_to_sorted_tuple(&dep_tuples))).unwrap();
  let country_tuples = count_to_category(paper_list, Box::new(move |x| x.country.as_ref().unwrap_or(&vec![]).clone()));
  std::fs::write(format!("{}/vega-countries.json", DATA_DIR),
    chart_pie_vega("Countries", &category_to_sorted_tuple(&country_tuples))).unwrap();
  println!("Got {} Countries with {} Unis and {} Departments.", country_tuples.len(), uni_tuples.len(), dep_tuples.len());


  let cs_papers = only_with_source(&paper_list, 0);
  let oss_papers = only_with_source(&paper_list, 1);
  let fakeoss_papers = only_with_source(&paper_list, 3);
  println!("cs {}, oss {}, NONE OF BOTH {}", cs_papers.len(), oss_papers.len(), fakeoss_papers.len());

  chart_paper_tags(paper_list);

  std::fs::write(format!("{}/vega-langs.json", DATA_DIR),
    chart_pie_vega("Publications by Language", &category_to_sorted_tuple(&lang_counts))).unwrap();
  std::fs::write(format!("{}/vega-fos.json", DATA_DIR),
    chart_pie_vega("Publications by Field of Study", &category_to_sorted_tuple(&fos_counts))).unwrap();
  std::fs::write(format!("{}/vega-journals.json", DATA_DIR),
    chart_pie_vega("Publications by Journals with more than one Publication", &journal_tuples)).unwrap();
  
  std::fs::write(format!("{}/vega-journals-bigger1.json", DATA_DIR),
    chart_pie_vega("Publications by Journals with more than one Publication", &journals_more_than_one)).unwrap();


  let fos_more_than_one: Vec<(String, usize)> = category_to_sorted_tuple(&fos_counts).iter().filter(|x| x.1 > 3).map(|x| x.clone()).collect::<Vec<(String, usize)>>();  
  let fos_only_one: Vec<(String, usize)> = category_to_sorted_tuple(&fos_counts).iter().filter(|x| x.1 == 1).map(|x| x.clone()).collect::<Vec<(String, usize)>>();
  
  std::fs::write(format!("{}/vega-fos-bigger1.json", DATA_DIR),
    chart_pie_vega("Publications by Field of Sty with more than one Publication", &fos_more_than_one)).unwrap();

  println!("{} out of {} Fields of Study only have one publication", fos_only_one.len(), fos_counts.len());
  println!("{} out of {} Journals only have one publication", journals_exactly_one.len(), journal_tuples.len());
}