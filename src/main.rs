use std::{
  sync::Arc,
  thread::{self, JoinHandle},
  time::Instant,
};

use md_compiler::*;
fn main() {
  fn compile(content_str: Arc<String>) -> JoinHandle<()> {
    thread::spawn(move || {
      md_to_html(&content_str);
    })
  }
  let start = Instant::now();
  let input_file = "hello.md";
  // 1. 读取文件内容
  let contents = Arc::new(read_file(input_file));
  // 2. 解析 Markdown 文本，生成 HTML
  let mut threads = Vec::with_capacity(2000);
  for _ in 0..10 {
    threads.push(compile(Arc::clone(&contents)));
  }

  for thread in threads {
    thread.join().unwrap();
  }
  // 3. 输出 HTML 到磁盘
  let mut output_file = String::new();
  output_file.push_str(&input_file[..input_file.len() - 3]);
  output_file.push_str(".html");
  let end = start.elapsed();
  print!("耗时 {} ms\n", end.as_millis());
  let result = md_to_html(&contents);
  write_file(output_file.as_str(), result.as_str());
}
