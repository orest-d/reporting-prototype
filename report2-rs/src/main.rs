use std::vec::Vec;
#[macro_use()]
use serde::{Serialize, Deserialize};

pub enum LinkType {
    Link,
    DataURL,
}

pub trait RenderContext {
    fn default_link_type(&self) -> LinkType;
    fn child_context(&self) -> Self;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Html(u32);
impl RenderContext for Html {
    fn default_link_type(&self) -> LinkType {
        LinkType::Link
    }
    fn child_context(&self) -> Self {
        Html(self.0 + 1)
    }
}

pub trait Renderable<T> {
    fn render(&self, render_context: &mut T) -> String;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ReportNode {
    Text(String),
    Section(Section),
}

impl ReportNode {
    fn new(title: &str) -> ReportNode {
        ReportNode::Section(Section {
            id: String::from("id"),
            title: String::from(title),
            children: vec![],
        })
    }
    fn with_text<'a>(&'a mut self, text: &str) -> &mut Self {
        match self {
            ReportNode::Section(ref mut section) => {
                section.children.push(ReportNode::Text(String::from(text)));
                self
            }
            ReportNode::Text(old_text) => {
                *self = ReportNode::Text(format!("{}\n{}", old_text, text));
                self
            }
        }
    }
    fn with_section<'a>(&'a mut self, title: &str) -> &mut Self {
        match self {
            ReportNode::Section(ref mut section) => {
                section.children.push(ReportNode::new(title));
                self
            }
            ReportNode::Text(ref old_text) => {
                let old_text = String::from(old_text);
                *self = ReportNode::new(title);
                self.with_text(&old_text)
            }
        }
    }
}

impl Renderable<Html> for ReportNode {
    fn render(&self, render_context: &mut Html) -> String {
        match self {
            ReportNode::Section(ref section) => {
                section.render(render_context)
            },
            ReportNode::Text(ref text) => {
                String::from(text)
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Section {
    id: String,
    title: String,
    children: Vec<ReportNode>,
}

impl Renderable<Html> for Section {
    fn render(&self, render_context: &mut Html) -> String {
        format!("<h{}>{}</h{}>{}",
            render_context.0,
            self.title,
            render_context.0,
            self.children.iter().map(|node| node.render(&mut render_context.child_context()))
            .collect::<Vec<_>>()
            .join("\n")
        )
    }
}

/*
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
*/
fn main() {
    let mut report = ReportNode::new("Document");
    let report = report.with_text("Hello").with_section("Subsection");
    let serialized = serde_json::to_string(&report).unwrap();
    println!("Serialized:\n{}",serialized);
    println!("Rendered:{}",report.render(&mut Html(1)))
    /*
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
    */
}
