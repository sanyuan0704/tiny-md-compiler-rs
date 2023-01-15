use std::{
  fs::File,
  io::{BufReader, Read, Write},
  path::Path,
  thread::{self, JoinHandle},
  time::Instant,
};

fn match_title() -> impl Fn(&str) -> bool {
  let mut title_set: Vec<String> = vec![];
  for i in 0..6 {
    title_set.push("#".repeat(i + 1));
  }
  move |word: &str| title_set.contains(&String::from(word))
}

#[inline]
pub fn md_to_html(md: &str) -> String {
  let mut html_fragments: Vec<String> = Vec::new();
  let title_matcher = match_title();
  md.lines()
    .into_iter()
    .enumerate()
    .for_each(|(index, line)| {
      // 寻找开头的第一个单词
      let mut words = line.split_whitespace();
      let first_word = words.next();
      match first_word {
        Some(word) if title_matcher(word) => {
          // 处理标题的情况
          let mut tag = "h".to_string();
          tag.push_str(&word.len().to_string());
          let content = words.collect::<Vec<&str>>().join(" ");

          html_fragments.push(format!("<{}>{}</{}>", tag, content, tag));
        }
        _ => {
          // 处理普通文本的情况
          if !line.is_empty() {
            html_fragments.push(format!("<p>{}</p>", line));
          }
        }
      };
    });
  html_fragments.join("\n")
}

pub fn read_file(path: &str) -> String {
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

pub fn write_file(path: &str, content: &str) {
  let mut file = File::create(path).unwrap();
  file.write_all(content.as_bytes()).unwrap();
}

#[test]
fn match_title_test() {
  let matcher = match_title();
  assert_eq!(true, matcher("#"));
  assert_eq!(true, matcher("##"));
  assert_eq!(true, matcher("###"));
  assert_eq!(true, matcher("####"));
  assert_eq!(true, matcher("#####"));
  assert_eq!(false, matcher("##123##"));
}

#[test]
fn read_md_file_test() {
  fn compile(content_str: String) -> JoinHandle<()> {
    thread::spawn(move || {
      md_to_html(&content_str);
    })
  }
  let start = Instant::now();
  let input_file = "hello.md";
  // 1. 读取文件内容
  let contents = read_file(input_file);
  // 2. 解析 Markdown 文本，生成 HTML
  let mut threads = Vec::with_capacity(2000);
  for _ in 0..2000 {
    threads.push(compile(contents.clone()));
  }

  for thread in threads {
    thread.join().unwrap();
  }
  let result = md_to_html(&contents);
  // 3. 输出 HTML 到磁盘
  let mut output_file = String::new();
  output_file.push_str(&input_file[..input_file.len() - 3]);
  output_file.push_str(".html");
  let end = start.elapsed();
  print!("{}\n", end.as_nanos());
  write_file(output_file.as_str(), result.as_str());
}
