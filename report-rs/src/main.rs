use std::vec::Vec;
#[macro_use()]
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Report {
    title: String,
    children: Vec<Content>,
}

trait ToHtml {
    fn to_html(&self) -> String {
        String::new()
    }
}

impl ToHtml for Report {
    fn to_html(&self) -> String {
        format!(
            "<html>
<head>
  <title>{title}</title>
</head>
<body>
  {body}
</body>
</html>
",
            title = self.title,
            body = self.children.iter().map(|x| x.to_html()).collect::<Vec<String>>().join("")
        )
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
enum Content {
    Section(Section),
    Html(String),
    Markdown(String),
}

impl ToHtml for Content {
    fn to_html(&self) -> String {
        match self {
            Self::Section(section) => section.to_html(),
            Self::Html(html) => html.to_owned(),
            Self::Markdown(markdown) => format!("<pre>{}</pre>", markdown),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Section {
    title: String,
    level: i32,
    children: Vec<Content>,
}

impl ToHtml for Section {
    fn to_html(&self) -> String {
        format!(
            "
  <h{level}>{title}</h{level}>
  {body}
",
            title = self.title,
            body = "",
            level = self.level
        )
    }
}

fn main() {
    let report = Report {
        title: "Report title".to_string(),
        children: vec![Content::Section(Section {
            title: "Section 1".to_string(),
            level: 1,
            children: vec![],
        }),
        Content::Html("Some html <b>text</b>".to_owned())
        ],
    };
    println!("Hello, world! {:?}", report);
    let serialized = serde_json::to_string(&report).unwrap();
    println!("JSON {}", serialized);
    println!("HTML {}", report.to_html());
}
