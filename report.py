import json

class Report:
    def __init__(self, title=""):
        self.title = title
        self.children = []

    def to_dict(self):
        return dict(title=self.title, children=[c.to_dict() for c in self.children])

    def add_section(self, title):
        section = Section(title)
        self.children.append(section)
        return section
    
    def add_html(self, html):
        o = Html(html)
        self.children.append(o)
        return o

    def add_markdown(self, md):
        o = Markdown(md)
        self.children.append(o)
        return o

    def section(self, title):
        section = Section(title)
        self.children.append(section)
        return self
    
    def html(self, html):
        o = Html(html)
        self.children.append(o)
        return self

    def markdown(self, md):
        o = Markdown(md)
        self.children.append(o)
        return self

    def to_html(self):
        body = "".join(c.to_html() for c in self.children)
        return f"""<html>
<head>
  <title>{self.title}</title>
</head>
<body>
  {body}
</body>
</html>"""


class Section:
    def __init__(self, title, level=1):
        self.title = title
        self.level = level
        self.children = []
    
    def add_html(self, html):
        o = Html(html)
        self.children.append(o)
        return o

    def add_markdown(self, md):
        o = Markdown(md)
        self.children.append(o)
        return o

    def add_section(self, title):
        section = Section(title, level=self.level+1)
        self.children.append(section)
        return section
    
    def section(self, title):
        self.add_section(title)
        return self
    
    def html(self, html):
        self.add_html(html)
        return self

    def markdown(self, md):
        self.add_markdown(md)
        return self

    def to_dict(self):
        return dict(
            Section=dict(title=self.title, children=[c.to_dict() for c in self.children])
        )

    def to_html(self):
        body = "".join(c.to_html() for c in self.children)
        return f"""  <h{self.level}>{self.title}</h{self.level}>
  {body}"""

class Html:
    def __init__(self, html):
        self.html = html

    def to_dict(self):
        return dict(Html=self.html)

    def to_html(self):
        return self.html

class Markdown:
    def __init__(self, markdown):
        self.markdown = markdown

    def to_dict(self):
        return dict(Markdown=self.markdown)

    def to_html(self):
        from markdown import markdown
        return markdown(self.markdown)

if __name__ == "__main__":
    report = Report("A report")
    report.add_section("Section 1").html("Hello <b>world</b>!").markdown("Hello **again**!")
    print(json.dumps(report.to_dict()))
    print(report.to_html())
