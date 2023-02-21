use chrono::{FixedOffset, TimeZone, Utc};
use criterion::{criterion_group, Criterion};
use rss2email_lib::{map_to_html, Blog, Post};

fn mock_blog() -> Vec<Blog> {
  let date_time = FixedOffset::east_opt(1000)
    .unwrap()
    .with_ymd_and_hms(1970, 3, 22, 1, 1, 1)
    .unwrap()
    .with_timezone(&Utc);
  let dummy_str = "a";

  let p = Post {
    title: dummy_str.to_string(),
    link: dummy_str.to_string(),
    description: Some(dummy_str.to_string()),
    pub_date: date_time,
  };

  vec![Blog {
    title: dummy_str.to_string(),
    most_recent_pub_date: date_time,
    posts: vec![p],
  }]
}

pub fn criterion_benchmark(c: &mut Criterion) {
  let blogs = mock_blog();

  c.bench_function("map to html", |b| b.iter(|| map_to_html(&blogs)));
}

criterion_group!(map_to_html_bench, criterion_benchmark);
