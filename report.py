import json

class HtmlRenderer:
    def __init__(self):
        self.result=""
    def render(self, entity):
        if isinstance(entity, Report):
            return self.render_report(entity)
        elif isinstance(entity, Section):
            return self.render_section(entity)
        elif isinstance(entity, Html):
            return self.render_html(entity)
        elif isinstance(entity, Markdown):
            return self.render_markdown(entity)
        else:
            return self.render_default(entity)

    def render_report(self, report):
        body = "".join(self.render(x) for x in report.children)
        return f"""<html>
<head>
  <title>{report.title}</title>
</head>
<body>
{body}
</body>
</html>
"""
    def render_section(self, section):
        return f"""  <h{section.level}>{section.title}</h{section.level}>
""" + "".join(self.render(x) for x in section.children)

    def render_html(self, html):
        return html.html

    def render_markdown(self, md):
        from markdown import markdown
        return markdown(md.markdown)

class EntityMixin:
    identifier=None

    def section(self, title, identifier=None):
        section = Section(title, identifier=identifier)
        self.children.append(section)
        return section
    
    def html(self, html, identifier=None):
        o = Html(html, identifier=identifier)
        self.children.append(o)
        return o

    def markdown(self, md, identifier=None):
        o = Markdown(md, identifier=identifier)
        self.children.append(o)
        return o

    def add_section(self, title, identifier=None):
        self.section(title, identifier=identifier)
        return self
    
    def add_html(self, html, identifier=None):
        self.html(html, identifier=identifier)
        return self

    def add_markdown(self, md, identifier=None):
        self.markdown(md, identifier=identifier)
        return self

    def entities(self):
        yield self

    def __getitem__(self, identifier):
        for x in self.entities():
            if x.identifier == identifier:
                return x
        return None

class Report(EntityMixin):
    def __init__(self, title=""):
        self.title = title
        self.children = []

    def to_dict(self):
        return dict(title=self.title, children=[c.to_dict() for c in self.children])

    def entities(self):
        yield self
        for x in self.children:
            for y in x.entities():
                yield y


class Section(EntityMixin):
    def __init__(self, title, level=1, identifier=None):
        self.title = title
        self.level = level
        self.identifier = identifier
        self.children = []

    def section(self, title):
        section = Section(title, level=self.level+1)
        self.children.append(section)
        return section

    def to_dict(self):
        return dict(
            Section=dict(title=self.title, children=[c.to_dict() for c in self.children])
        )

    def entities(self):
        yield self
        for x in self.children:
            for y in x.entities():
                yield y

class Html:
    def __init__(self, html, identifier=None):
        self.html = html
        self.identifier = identifier

    def to_dict(self):
        return dict(Html=self.html)

class Markdown:
    def __init__(self, markdown, identifier=None):
        self.markdown = markdown
        self.identifier = identifier

    def to_dict(self):
        return dict(Markdown=self.markdown)

if __name__ == "__main__":
    report = Report("A report")
    report.section("Section 1").add_html("Hello <b>world</b>!").add_markdown("Hello **again**!")
    print(json.dumps(report.to_dict()))
    print(HtmlRenderer().render(report))
