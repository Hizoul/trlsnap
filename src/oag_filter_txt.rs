use std::fs::{File, OpenOptions, metadata};
use std::io::{Result, Write, BufRead, BufReader};
use serde_json::{Value, from_str};
const FOLDER: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/";

const FILE_TRANSFER: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/t";
const FILE_R: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/r";
const FILE_RL: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/rl";
const FILE_TR: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/tr";
const FILE_TRL: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/trl";
const FILE_TRANSFER_A: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/t_a";
const FILE_R_A: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/r_a";
const FILE_RL_A: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/rl_a";
const FILE_TR_A: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/tr_a";
const FILE_TRL_A: &str = "/run/media/mmb/Storage/5-Use/openacademicgraph/trl_a";

const ALL_FILES: [&str;10] = [FILE_TRANSFER, FILE_R, FILE_TR, FILE_RL, FILE_TRL, FILE_TRANSFER_A, FILE_R_A, FILE_TR_A, FILE_RL_A, FILE_TRL_A];

pub fn append(path: &str, content: &[u8]) -> std::io::Result<()> {
  OpenOptions::new()
  .write(true)
  .append(true)
  .open(path)?
  .write(content)?;
  Ok(())
}


pub type LineProcessor = dyn Fn(Result<String>);

pub fn oag_filter_keywords(file_name: &str, start_fresh_opt: Option<bool>) -> Result<()> {
  let processor = Box::new(|line_res: Result<String>| {
    let line: String = line_res.unwrap();
    let line_with_newline = format!("{}\n", line);
    if line.contains("\"abstract\"") || line.contains("\"keywords\"") {
      let parsed_line: Value = from_str(&line).unwrap();
      let mut to_scan: Vec<String> = Vec::with_capacity(2);
      let abstract_opt = parsed_line.get("abstract");
      if abstract_opt.is_some() {
        to_scan.push(abstract_opt.unwrap().as_str().unwrap().to_owned());
      }
      let keywords_opt = parsed_line.get("keywords");
      if keywords_opt.is_some() {
        let keywords = keywords_opt.unwrap();
        to_scan.push(keywords.to_string());
      }
      let mut i = 0;
      for to_analyze in to_scan.iter() {
        let (f_t, f_r, f_tr, f_rl, f_trl) = if i == 1 {
          (FILE_TRANSFER, FILE_R, FILE_TR, FILE_RL, FILE_TRL)
        } else {
          (FILE_TRANSFER_A, FILE_R_A, FILE_TR_A, FILE_RL_A, FILE_TRL_A)
        };
        i += 1;
        let mut contained_transfer = false;
        let contained_learning = to_analyze.contains("learning");
        if to_analyze.contains("transfer") {
          append(f_t, line_with_newline.as_bytes()).unwrap();
          contained_transfer = true;
        }
        if to_analyze.contains("reinforcement") {
          append(f_r, line_with_newline.as_bytes()).unwrap();
          if contained_learning {
            append(f_rl, line_with_newline.as_bytes()).unwrap();
          }
          if contained_transfer {
            append(f_tr, line_with_newline.as_bytes()).unwrap();
            if contained_learning {
              append(f_trl, line_with_newline.as_bytes()).unwrap();
            }
          }
        }
      }
    }
  });
  oag_reader(file_name, start_fresh_opt, processor)
}

pub fn oag_reader(file_name: &str, start_fresh_opt: Option<bool>, processor: Box<LineProcessor>) -> Result<()> {
  let start_fresh = start_fresh_opt.unwrap_or(false);
  for file_name in ALL_FILES.iter() {
    let file_exists = metadata(file_name).is_ok();
    if !file_exists || start_fresh {
      File::create(file_name).unwrap();
    }
  }
  let file = File::open(format!("{}{}", FOLDER, file_name))?;
  let reader = BufReader::new(file);
  reader.lines().for_each(processor);
  Ok(())
}

pub fn count_lines(file_name: &str) -> Result<usize> {
  let file = File::open(format!("{}{}", FOLDER, file_name))?;
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