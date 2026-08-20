#![cfg(target_arch = "wasm32")]

use eframe::egui::{self, Color32, Pos2, RichText};
use egui_graphs::events::Event;
use egui_graphs::{
    FruchtermanReingoldWithCenterGravity, FruchtermanReingoldWithCenterGravityState, Graph,
    GraphView, LayoutForceDirected, SettingsInteraction, SettingsNavigation, SettingsStyle,
};
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlCanvasElement, Response};

#[derive(Clone, Deserialize)]
struct GraphData {
    families: Vec<Family>,
    courses: Vec<Course>,
    topics: Vec<Topic>,
    edges: Vec<Edge>,
}

#[derive(Clone, Deserialize)]
struct Family {
    id: String,
    label: String,
}

#[derive(Clone, Deserialize)]
struct Course {
    id: String,
    family: String,
    title: String,
}

#[derive(Clone, Deserialize)]
struct Topic {
    id: String,
    course: String,
    title: String,
    url: String,
}

#[derive(Clone, Deserialize)]
struct Edge {
    source: String,
    target: String,
}

#[derive(Clone)]
struct TopicPayload {
    course: String,
    url: String,
    base_color: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Visibility {
    Enabled,
    Context,
    Hidden,
}

impl Visibility {
    fn next(self) -> Self {
        match self {
            Self::Enabled => Self::Context,
            Self::Context => Self::Hidden,
            Self::Hidden => Self::Enabled,
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            Self::Enabled => "●",
            Self::Context => "◐",
            Self::Hidden => "○",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Context => "linked context only",
            Self::Hidden => "hidden",
        }
    }
}

type TopicGraph = Graph<TopicPayload, ()>;

struct CourseGraphApp {
    data: GraphData,
    graph: TopicGraph,
    visibility: BTreeMap<String, Visibility>,
    node_topics: HashMap<usize, TopicPayload>,
    hovered_course: Option<String>,
    sidebar_hovered_course: Option<String>,
}

impl CourseGraphApp {
    fn new(data: GraphData) -> Self {
        let visibility = data
            .courses
            .iter()
            .map(|course| (course.id.clone(), Visibility::Enabled))
            .collect();
        let mut app = Self {
            data,
            graph: Graph::new(Default::default()),
            visibility,
            node_topics: HashMap::new(),
            hovered_course: None,
            sidebar_hovered_course: None,
        };
        app.rebuild_graph();
        app
    }

    fn rebuild_graph(&mut self) {
        let enabled: HashSet<_> = self
            .data
            .topics
            .iter()
            .filter(|topic| self.visibility[&topic.course] == Visibility::Enabled)
            .map(|topic| topic.id.as_str())
            .collect();
        let mut adjacent_to_enabled = HashSet::new();
        for edge in &self.data.edges {
            if enabled.contains(edge.source.as_str()) {
                adjacent_to_enabled.insert(edge.target.as_str());
            }
            if enabled.contains(edge.target.as_str()) {
                adjacent_to_enabled.insert(edge.source.as_str());
            }
        }

        let visible: Vec<_> = self
            .data
            .topics
            .iter()
            .filter(|topic| match self.visibility[&topic.course] {
                Visibility::Enabled => true,
                Visibility::Context => adjacent_to_enabled.contains(topic.id.as_str()),
                Visibility::Hidden => false,
            })
            .cloned()
            .collect();

        let mut graph = TopicGraph::new(Default::default());
        let mut indices = HashMap::new();
        let mut node_topics = HashMap::new();
        let count = visible.len().max(1) as f32;
        for (position, topic) in visible.iter().enumerate() {
            let angle = std::f32::consts::TAU * position as f32 / count;
            let course_position = self
                .data
                .courses
                .iter()
                .position(|course| course.id == topic.course)
                .unwrap_or_default();
            let context = self.visibility[&topic.course] == Visibility::Context;
            let base_color = if context {
                Color32::GRAY
            } else {
                course_color(course_position)
            };
            let payload = TopicPayload {
                course: topic.course.clone(),
                url: topic.url.clone(),
                base_color,
            };
            let index = graph.add_node_with_label_and_location(
                payload.clone(),
                topic.title.clone(),
                Pos2::new(angle.cos() * 280.0, angle.sin() * 280.0),
            );
            graph.node_mut(index).unwrap().set_color(base_color);
            indices.insert(topic.id.as_str(), index);
            node_topics.insert(index.index(), payload);
        }
        for edge in &self.data.edges {
            if let (Some(source), Some(target)) = (
                indices.get(edge.source.as_str()),
                indices.get(edge.target.as_str()),
            ) {
                graph.add_edge(*source, *target, ());
            }
        }
        self.graph = graph;
        self.node_topics = node_topics;
        self.hovered_course = None;
    }

    fn set_family_visibility(&mut self, family: &str, visibility: Visibility) {
        for course in self
            .data
            .courses
            .iter()
            .filter(|course| course.family == family)
        {
            self.visibility.insert(course.id.clone(), visibility);
        }
        self.rebuild_graph();
    }

    fn family_visibility(&self, family: &str) -> Visibility {
        let states: BTreeSet<_> = self
            .data
            .courses
            .iter()
            .filter(|course| course.family == family)
            .map(|course| self.visibility[&course.id] as u8)
            .collect();
        if states.len() == 1 {
            match *states.first().unwrap() {
                0 => Visibility::Enabled,
                1 => Visibility::Context,
                _ => Visibility::Hidden,
            }
        } else {
            Visibility::Context
        }
    }

    fn apply_highlight(&mut self) {
        let highlighted = self
            .sidebar_hovered_course
            .as_ref()
            .or(self.hovered_course.as_ref());
        let indices: Vec<_> = self.graph.nodes_iter().map(|(index, _)| index).collect();
        for index in indices {
            let payload = self.node_topics.get(&index.index()).unwrap();
            let color = match highlighted {
                None => payload.base_color,
                Some(course) if course == &payload.course => brighten(payload.base_color),
                Some(_) => Color32::from_gray(175),
            };
            self.graph.node_mut(index).unwrap().set_color(color);
        }
    }

    fn course_label(&self, id: &str) -> String {
        self.data
            .courses
            .iter()
            .find(|course| course.id == id)
            .map(|course| format!("{} — {}", course.id, course.title))
            .unwrap_or_else(|| id.to_owned())
    }
}

impl eframe::App for CourseGraphApp {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        self.sidebar_hovered_course = None;
        ui.horizontal_top(|ui| {
            let sidebar_size = egui::vec2(285.0, ui.available_height());
            ui.allocate_ui_with_layout(
                sidebar_size,
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.horizontal(|ui| {
                        if ui.link("← Notes").clicked() {
                            navigate("../");
                        }
                        ui.heading("Courses");
                    });
                    ui.label("Click a marker to cycle: enabled → context → hidden.");
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let families = self.data.families.clone();
                        for family in families {
                            let state = self.family_visibility(&family.id);
                            ui.horizontal(|ui| {
                                if ui
                                    .button(state.symbol())
                                    .on_hover_text(state.description())
                                    .clicked()
                                {
                                    self.set_family_visibility(&family.id, state.next());
                                }
                                ui.strong(&family.label);
                            });
                            let courses: Vec<_> = self
                                .data
                                .courses
                                .iter()
                                .filter(|course| course.family == family.id)
                                .cloned()
                                .collect();
                            for course in courses {
                                ui.horizontal(|ui| {
                                    ui.add_space(16.0);
                                    let state = self.visibility[&course.id];
                                    if ui
                                        .button(state.symbol())
                                        .on_hover_text(state.description())
                                        .clicked()
                                    {
                                        self.visibility.insert(course.id.clone(), state.next());
                                        self.rebuild_graph();
                                    }
                                    let response =
                                        ui.label(format!("{} — {}", course.id, course.title));
                                    if response.hovered() {
                                        self.sidebar_hovered_course = Some(course.id.clone());
                                    }
                                });
                            }
                            ui.add_space(8.0);
                        }
                    });
                },
            );

            ui.separator();
            ui.vertical(|ui| {
                self.apply_highlight();
                if let Some(course) = self
                    .sidebar_hovered_course
                    .as_ref()
                    .or(self.hovered_course.as_ref())
                {
                    ui.label(RichText::new(self.course_label(course)).strong());
                } else {
                    ui.label("Hover a topic to highlight its course; click it to open the note.");
                }

                let events = Rc::new(RefCell::new(Vec::new()));
                let mut view = GraphView::<
                    TopicPayload,
                    (),
                    _,
                    _,
                    _,
                    _,
                    FruchtermanReingoldWithCenterGravityState,
                    LayoutForceDirected<FruchtermanReingoldWithCenterGravity>,
                >::new(&mut self.graph)
                .with_interactions(
                    &SettingsInteraction::new()
                        .with_dragging_enabled(true)
                        .with_hover_enabled(true)
                        .with_node_clicking_enabled(true),
                )
                .with_navigations(
                    &SettingsNavigation::new()
                        .with_zoom_and_pan_enabled(true)
                        .with_fit_to_screen_enabled(true)
                        .with_fit_to_screen_padding(0.03),
                )
                .with_styles(&SettingsStyle::new().with_labels_always(false))
                .with_event_sink(&events);
                ui.add(&mut view);
                drop(view);

                for event in events.borrow_mut().drain(..) {
                    match event {
                        Event::NodeHoverEnter(event) => {
                            self.hovered_course = self
                                .node_topics
                                .get(&event.id)
                                .map(|topic| topic.course.clone());
                        }
                        Event::NodeHoverLeave(_) => self.hovered_course = None,
                        Event::NodeClick(event) => {
                            if let Some(topic) = self.node_topics.get(&event.id) {
                                navigate(&topic.url);
                            }
                        }
                        _ => {}
                    }
                }
            });
        });
        ui.ctx().request_repaint();
    }

    fn clear_color(&self, _: &egui::Visuals) -> [f32; 4] {
        Color32::from_rgb(251, 250, 252).to_normalized_gamma_f32()
    }
}

fn course_color(index: usize) -> Color32 {
    const COLORS: [Color32; 10] = [
        Color32::from_rgb(77, 103, 178),
        Color32::from_rgb(204, 82, 82),
        Color32::from_rgb(65, 145, 111),
        Color32::from_rgb(164, 103, 184),
        Color32::from_rgb(209, 137, 52),
        Color32::from_rgb(57, 143, 171),
        Color32::from_rgb(181, 88, 139),
        Color32::from_rgb(116, 137, 60),
        Color32::from_rgb(103, 93, 164),
        Color32::from_rgb(172, 100, 62),
    ];
    COLORS[index % COLORS.len()]
}

fn brighten(color: Color32) -> Color32 {
    Color32::from_rgb(
        color.r().saturating_add(45),
        color.g().saturating_add(45),
        color.b().saturating_add(45),
    )
}

fn navigate(url: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href(url);
    }
}

async fn fetch_graph() -> Result<GraphData, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let response = JsFuture::from(window.fetch_with_str("../graph.json")).await?;
    let response: Response = response.dyn_into()?;
    let json = JsFuture::from(response.json()?).await?;
    serde_wasm_bindgen::from_value(json).map_err(|error| JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        if let Err(error) = run().await {
            web_sys::console::error_1(&error);
        }
    });
    Ok(())
}

async fn run() -> Result<(), JsValue> {
    let data = fetch_graph().await?;
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("no document"))?;
    if let Some(loading) = document.get_element_by_id("loading") {
        loading.remove();
    }
    let canvas = document
        .get_element_by_id("graph-canvas")
        .ok_or_else(|| JsValue::from_str("graph canvas not found"))?
        .dyn_into::<HtmlCanvasElement>()?;
    eframe::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(move |creation_context| {
                let mut visuals = egui::Visuals::light();
                let paper = Color32::from_rgb(251, 250, 252);
                visuals.panel_fill = paper;
                visuals.window_fill = paper;
                visuals.extreme_bg_color = Color32::WHITE;
                creation_context.egui_ctx.set_visuals(visuals);
                Ok(Box::new(CourseGraphApp::new(data)))
            }),
        )
        .await
}
