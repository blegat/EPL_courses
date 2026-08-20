use anyhow::{Result, bail};
use percent_encoding::percent_decode_str;
use regex::{Captures, Regex};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use walkdir::WalkDir;

#[derive(Clone)]
struct Note {
    source: PathBuf,
    family: String,
    number: String,
    course: String,
    course_path: String,
    title: String,
    url: String,
    is_index: bool,
    body: String,
}

#[derive(Serialize)]
struct GraphData {
    families: Vec<Family>,
    courses: Vec<Course>,
    topics: Vec<Topic>,
    edges: Vec<Edge>,
}

#[derive(Serialize)]
struct Family {
    id: String,
    label: String,
}

#[derive(Serialize)]
struct Course {
    id: String,
    family: String,
    title: String,
}

#[derive(Serialize)]
struct Topic {
    id: String,
    course: String,
    title: String,
    url: String,
}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    source: String,
    target: String,
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args_os().collect();
    if args.len() != 4 {
        bail!("usage: epl-sitegen REPOSITORY CONTENT_DIR GRAPH_JSON");
    }

    let root = PathBuf::from(&args[1]).canonicalize()?;
    let content_dir = PathBuf::from(&args[2]);
    let graph_path = PathBuf::from(&args[3]);
    let family_re = Regex::new(r"^[A-Z]+$")?;
    let number_re = Regex::new(r"^[0-9]+$")?;
    let heading_re = Regex::new(r"(?m)^#\s+(.+?)\s*$")?;

    let mut notes = Vec::new();
    for family_entry in fs::read_dir(&root)? {
        let family_entry = family_entry?;
        if !family_entry.file_type()?.is_dir() {
            continue;
        }
        let family = family_entry.file_name().to_string_lossy().into_owned();
        if !family_re.is_match(&family) {
            continue;
        }

        for course_entry in fs::read_dir(family_entry.path())? {
            let course_entry = course_entry?;
            if !course_entry.file_type()?.is_dir() {
                continue;
            }
            let number = course_entry.file_name().to_string_lossy().into_owned();
            if !number_re.is_match(&number) || !course_entry.path().join("README.md").is_file() {
                continue;
            }
            let course = format!("L{family}{number}");
            let course_path = format!("{family}/{number}");

            for item in WalkDir::new(course_entry.path()).min_depth(1).max_depth(1) {
                let item = item?;
                if !item.file_type().is_file()
                    || item.path().extension().and_then(|s| s.to_str()) != Some("md")
                {
                    continue;
                }
                let source = item.path().canonicalize()?;
                let body = fs::read_to_string(&source)?;
                let title = heading_re
                    .captures(&body)
                    .and_then(|capture| capture.get(1))
                    .map(|m| m.as_str().trim().to_owned())
                    .unwrap_or_else(|| {
                        item.path()
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned()
                    });
                let is_index = item.file_name() == "README.md";
                let url = if is_index {
                    format!("/{course_path}/")
                } else {
                    format!("/{course_path}/{}/", slug(&title))
                };
                notes.push(Note {
                    source,
                    family: family.clone(),
                    number: number.clone(),
                    course: course.clone(),
                    course_path: course_path.clone(),
                    title,
                    url,
                    is_index,
                    body,
                });
            }
        }
    }
    notes.sort_by(|a, b| a.source.cmp(&b.source));

    let url_by_source: BTreeMap<_, _> = notes
        .iter()
        .map(|note| (note.source.clone(), note.url.clone()))
        .collect();
    let topic_id_by_source: BTreeMap<_, _> = notes
        .iter()
        .filter(|note| !note.is_index)
        .map(|note| (note.source.clone(), topic_id(note)))
        .collect();

    if content_dir.exists() {
        fs::remove_dir_all(&content_dir)?;
    }
    fs::create_dir_all(&content_dir)?;

    let mut course_titles = BTreeMap::new();
    for note in &notes {
        let output_dir = content_dir.join(&note.course_path);
        fs::create_dir_all(&output_dir)?;
        let output = if note.is_index {
            course_titles.insert(
                note.course.clone(),
                (
                    note.family.clone(),
                    note.number.clone(),
                    note.course_path.clone(),
                    concise_course_title(&note.title, &note.course),
                ),
            );
            output_dir.join("_index.md")
        } else {
            output_dir.join(format!("{}.md", slug(&note.title)))
        };
        let rewritten = rewrite_links(note, &url_by_source)?;
        let frontmatter = if note.is_index {
            format!(
                "+++\ntitle = \"{}\"\nsort_by = \"title\"\ntemplate = \"section.html\"\npage_template = \"page.html\"\n+++\n\n",
                toml_escape(&note.title)
            )
        } else {
            format!(
                "+++\ntitle = \"{}\"\npath = \"{}\"\ntemplate = \"page.html\"\n[extra]\ncourse = \"{}\"\n+++\n\n",
                toml_escape(&note.title),
                note.url,
                note.course_path
            )
        };
        fs::write(output, frontmatter + &rewritten)?;
    }

    let mut landing = String::from(
        "+++\ntitle = \"EPL course graph\"\ntemplate = \"index.html\"\n+++\n\n# EPL course notes\n\nBrowse the course notes or explore their relationships in the [interactive graph](graph/).\n\n",
    );
    for (course, (_, _, path, title)) in &course_titles {
        landing.push_str(&format!("- [{course} — {title}]({path}/)\n"));
    }
    fs::write(content_dir.join("_index.md"), landing)?;

    let families: BTreeSet<_> = course_titles
        .values()
        .map(|value| value.0.clone())
        .collect();
    for family in &families {
        let family_dir = content_dir.join(family);
        fs::create_dir_all(&family_dir)?;
        fs::write(
            family_dir.join("_index.md"),
            format!("+++\ntitle = \"{family}\"\nsort_by = \"title\"\ntransparent = true\n+++\n"),
        )?;
    }
    let courses = course_titles
        .iter()
        .map(|(id, (family, _, _, title))| Course {
            id: id.clone(),
            family: family.clone(),
            title: title.clone(),
        })
        .collect();
    let topics = notes
        .iter()
        .filter(|note| !note.is_index)
        .map(|note| Topic {
            id: topic_id(note),
            course: note.course.clone(),
            title: note.title.clone(),
            url: format!("../{}/{}/", note.course_path, slug(&note.title)),
        })
        .collect();

    let link_re = markdown_link_regex()?;
    let mut edges = BTreeSet::new();
    for note in notes.iter().filter(|note| !note.is_index) {
        let source_id = topic_id(note);
        for capture in link_re.captures_iter(&note.body) {
            if capture.name("image").is_some() {
                continue;
            }
            if let Some(target) = resolve_markdown_target(note, &capture["destination"])? {
                if let Some(target_id) = topic_id_by_source.get(&target) {
                    if target_id != &source_id {
                        let (source, target) = if source_id < *target_id {
                            (source_id.clone(), target_id.clone())
                        } else {
                            (target_id.clone(), source_id.clone())
                        };
                        edges.insert(Edge { source, target });
                    }
                }
            }
        }
    }

    let graph = GraphData {
        families: families
            .into_iter()
            .map(|id| Family {
                label: id.clone(),
                id,
            })
            .collect(),
        courses,
        topics,
        edges: edges.into_iter().collect(),
    };
    if let Some(parent) = graph_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(graph_path, serde_json::to_vec_pretty(&graph)?)?;
    Ok(())
}

fn markdown_link_regex() -> Result<Regex> {
    Ok(Regex::new(
        r"(?P<image>!)?\[(?P<label>[^\]]*)\]\(\s*(?P<destination><[^>]+>|[^\s)]+)",
    )?)
}

fn rewrite_links(note: &Note, urls: &BTreeMap<PathBuf, String>) -> Result<String> {
    let regex = markdown_link_regex()?;
    Ok(regex
        .replace_all(&note.body, |capture: &Captures<'_>| {
            if capture.name("image").is_some() {
                return capture[0].to_owned();
            }
            let destination = capture["destination"].trim_matches(&['<', '>'][..]);
            if destination
                .split_once('#')
                .map_or(destination, |(path, _)| path)
                .to_ascii_lowercase()
                .ends_with(".pdf")
            {
                return capture["label"].to_owned();
            }
            let Ok(Some(target)) = resolve_markdown_target(note, &capture["destination"]) else {
                return capture[0].to_owned();
            };
            let Some(target_url) = urls.get(&target) else {
                return capture[0].to_owned();
            };
            let fragment = capture["destination"]
                .trim_matches(&['<', '>'][..])
                .split_once('#')
                .map(|(_, fragment)| format!("#{fragment}"))
                .unwrap_or_default();
            let relative = relative_url(&note.url, target_url);
            capture[0].replacen(&capture["destination"], &(relative + &fragment), 1)
        })
        .into_owned())
}

fn resolve_markdown_target(note: &Note, raw: &str) -> Result<Option<PathBuf>> {
    let raw = raw.trim_matches(&['<', '>'][..]);
    if raw.starts_with('#')
        || raw.starts_with('/')
        || raw.contains("://")
        || raw.starts_with("mailto:")
    {
        return Ok(None);
    }
    let path = raw.split('#').next().unwrap_or_default();
    let decoded = percent_decode_str(path).decode_utf8_lossy();
    if !decoded.ends_with(".md") {
        return Ok(None);
    }
    let joined = note.source.parent().unwrap().join(decoded.as_ref());
    Ok(Some(normalize(&joined)))
}

fn normalize(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            other => result.push(other.as_os_str()),
        }
    }
    result
}

fn relative_url(source: &str, target: &str) -> String {
    let source = Path::new(source.trim_matches('/'));
    let target = Path::new(target.trim_matches('/'));
    let relative = pathdiff::diff_paths(target, source).unwrap_or_else(|| target.to_path_buf());
    format!("{}/", relative.to_string_lossy())
}

fn topic_id(note: &Note) -> String {
    format!("{}/{}", note.course, slug(&note.title))
}

fn slug(value: &str) -> String {
    let mut result = String::new();
    let mut separator = false;
    for character in value.chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() {
            if separator && !result.is_empty() {
                result.push('-');
            }
            separator = false;
            result.push(character);
        } else {
            separator = true;
        }
    }
    result
}

fn concise_course_title(title: &str, course: &str) -> String {
    title
        .strip_prefix(course)
        .unwrap_or(title)
        .trim_start_matches(&[' ', ':', '—', '-'][..])
        .to_owned()
}

fn toml_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
