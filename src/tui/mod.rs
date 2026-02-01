use crate::api;
use crate::generator;
use crate::types::api::{InitializrCapabilities, InitializrDependencies};
use crate::types::generic::SprintInitConfig;
use crate::types::config::FileType;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::collections::BTreeSet;
use std::io;

// --- Theme Colors ---
const BG_COLOR: Color = Color::Rgb(30, 33, 39); // Dark background
const PANEL_BG: Color = Color::Rgb(30, 33, 39); // Panel background
const ACCENT_COLOR: Color = Color::Rgb(109, 179, 63); // Spring Green
const TEXT_COLOR: Color = Color::White;
const MUTED_COLOR: Color = Color::DarkGray;
const INPUT_BG: Color = Color::Rgb(40, 44, 52);

#[derive(Clone, Copy, PartialEq)]
enum Field {
    ProjectType,
    Language,
    BootVersion,
    Packaging,
    JavaVersion,
    GroupId,
    ArtifactId,
    Name,
    Description,
    PackageName,
    ConfigurationFormat,
    Generate,
    Export,
}

#[derive(Clone, Copy, PartialEq)]
enum ActivePane {
    Config,
    Dependencies,
}

struct App {
    capabilities: InitializrCapabilities,
    dependencies: InitializrDependencies,
    config: SprintInitConfig,
    current_field: Field,
    active_pane: ActivePane,
    list_state: ListState,
    deps_list_state: ListState,
    input_mode: bool,
    input_buffer: String,
    deps_search: String,
    selected_deps: BTreeSet<String>,
    status_message: String,
    show_popup: bool,
    show_export_popup: bool,
    export_filename: String,
    export_format_idx: usize, // 0=Yaml, 1=Json, 2=Toml
    export_focus_filename: bool,
    extract_project: bool,
    show_config_popup: bool,
    show_message_popup: bool,
    message_popup_title: String,
    message_popup_text: String,
    message_popup_is_error: bool,
}

impl App {
    fn new(capabilities: InitializrCapabilities, dependencies: InitializrDependencies) -> Self {
        let config = SprintInitConfig {
            project_type: capabilities
                .project_type
                .as_ref()
                .and_then(|pt| pt.default.clone())
                .unwrap_or_else(|| "maven-project".to_string()),
            language: capabilities
                .language
                .as_ref()
                .and_then(|l| l.default.clone())
                .unwrap_or_else(|| "java".to_string()),
            platform_version: "".to_string(),
            packaging: capabilities
                .packaging
                .as_ref()
                .and_then(|p| p.default.clone())
                .unwrap_or_else(|| "jar".to_string()),
            configuration_file_format: "properties".to_string(),
            java_version: capabilities
                .java_version
                .as_ref()
                .and_then(|jv| jv.default.as_ref())
                .and_then(|v| v.parse().ok())
                .unwrap_or(17),
            group_id: capabilities
                .group_id
                .as_ref()
                .map(|g| g.default.clone())
                .unwrap_or_else(|| "com.example".to_string()),
            artifact_id: capabilities
                .artifact_id
                .as_ref()
                .map(|a| a.default.clone())
                .unwrap_or_else(|| "demo".to_string()),
            name: capabilities
                .name
                .as_ref()
                .map(|n| n.default.clone())
                .unwrap_or_else(|| "demo".to_string()),
            description: capabilities
                .description
                .as_ref()
                .map(|d| d.default.clone())
                .unwrap_or_else(|| "Demo project for Spring Boot".to_string()),
            package_name: capabilities
                .package_name
                .as_ref()
                .map(|p| p.default.clone())
                .unwrap_or_else(|| "com.example.demo".to_string()),
            dependencies: "".to_string(),
            boot_version: capabilities
                .boot_version
                .as_ref()
                .and_then(|bv| bv.default.clone())
                .unwrap_or_else(|| "3.2.0".to_string()),
            version: capabilities
                .version
                .as_ref()
                .map(|v| v.default.clone())
                .unwrap_or_else(|| "0.0.1-SNAPSHOT".to_string()),
        };

        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let mut deps_list_state = ListState::default();
        deps_list_state.select(Some(0));
// status_message: "<󰌒Tab> Switch Pane  <> Navigate  <󰌑Enter> Select/Edit  <󰘲Shift+c> Config Menu".to_string(),
        App {
            capabilities,
            dependencies,
            config,
            current_field: Field::ProjectType,
            active_pane: ActivePane::Config,
            list_state,
            deps_list_state,
            input_mode: false,
            input_buffer: String::new(),
            deps_search: String::new(),
            selected_deps: BTreeSet::new(),
            status_message: "<Tab> Switch Pane  <> Navigate  <Enter> Select/Edit  <Shift+c> Config Menu".to_string(),
            show_popup: false,
            show_export_popup: false,
            export_filename: "config".to_string(),
            export_format_idx: 0,
            export_focus_filename: true,
            extract_project: false,
            show_config_popup: false,
            show_message_popup: false,
            message_popup_title: String::new(),
            message_popup_text: String::new(),
            message_popup_is_error: false,
        }
    }

    fn toggle_pane(&mut self) {
        self.active_pane = match self.active_pane {
            ActivePane::Config => ActivePane::Dependencies,
            ActivePane::Dependencies => ActivePane::Config,
        };
    }

    fn next_dependency(&mut self) {
        let len = self.dependency_options().len();
        if len > 0 {
            let i = self.deps_list_state.selected().unwrap_or(0);
            self.deps_list_state.select(Some((i + 1) % len));
        }
    }

    fn previous_dependency(&mut self) {
        let len = self.dependency_options().len();
        if len > 0 {
            let i = self.deps_list_state.selected().unwrap_or(0);
            self.deps_list_state.select(Some(if i == 0 { len - 1 } else { i - 1 }));
        }
    }

    fn next_field(&mut self) {
        self.current_field = match self.current_field {
            Field::ProjectType => Field::Language,
            Field::Language => Field::BootVersion,
            Field::BootVersion => Field::GroupId, // Jump to metadata start
            Field::GroupId => Field::ArtifactId,
            Field::ArtifactId => Field::Name,
            Field::Name => Field::Description,
            Field::Description => Field::PackageName,
            Field::PackageName => Field::Packaging, // Layout order
            Field::Packaging => Field::JavaVersion,
            Field::JavaVersion => Field::ConfigurationFormat,
            Field::ConfigurationFormat => Field::Export,
            Field::Export => Field::Generate,
            Field::Generate => Field::ProjectType,
        };
        self.list_state.select(Some(0));
        self.show_popup = false;
    }

    fn previous_field(&mut self) {
        self.current_field = match self.current_field {
            Field::ProjectType => Field::Generate,
            Field::Language => Field::ProjectType,
            Field::BootVersion => Field::Language,
            Field::GroupId => Field::BootVersion,
            Field::ArtifactId => Field::GroupId,
            Field::Name => Field::ArtifactId,
            Field::Description => Field::Name,
            Field::PackageName => Field::Description,
            Field::Packaging => Field::PackageName,
            Field::JavaVersion => Field::Packaging,
            Field::ConfigurationFormat => Field::JavaVersion,
            Field::Export => Field::ConfigurationFormat,
            Field::Generate => Field::Export,
        };
        self.list_state.select(Some(0));
        self.show_popup = false;
    }

    fn get_current_options(&self) -> Vec<String> {
        match self.current_field {
            Field::ProjectType => self.capabilities.project_type.as_ref().map(|pt| pt.values.iter().map(|v| v.id.clone()).collect()).unwrap_or_default(),
            Field::Language => self.capabilities.language.as_ref().map(|l| l.values.iter().map(|v| v.id.clone()).collect()).unwrap_or_default(),
            Field::BootVersion => self.capabilities.boot_version.as_ref().map(|bv| bv.values.iter().map(|v| v.id.clone()).collect()).unwrap_or_default(),
            Field::Packaging => self.capabilities.packaging.as_ref().map(|p| p.values.iter().map(|v| v.id.clone()).collect()).unwrap_or_default(),
            Field::JavaVersion => self.capabilities.java_version.as_ref().map(|jv| jv.values.iter().map(|v| v.id.clone()).collect()).unwrap_or_default(),
            Field::ConfigurationFormat => vec!["properties".to_string(), "yaml".to_string()],
            _ => vec![],
        }
    }

    fn select_option(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let options = self.get_current_options();
            if selected < options.len() {
                let value = options[selected].clone();
                match self.current_field {
                    Field::ProjectType => self.config.project_type = value,
                    Field::Language => self.config.language = value,
                    Field::BootVersion => self.config.boot_version = value,
                    Field::Packaging => self.config.packaging = value,
                    Field::JavaVersion => {
                        if let Ok(v) = value.parse() {
                            self.config.java_version = v;
                        }
                    }
                    Field::ConfigurationFormat => self.config.configuration_file_format = value,
                    _ => {}
                }
                self.show_popup = false;
                self.refresh_package_name();
            }
        }
    }

    fn start_edit(&mut self) {
        match self.current_field {
            Field::GroupId => { self.input_mode = true; self.input_buffer = self.config.group_id.clone(); }
            Field::ArtifactId => { self.input_mode = true; self.input_buffer = self.config.artifact_id.clone(); }
            Field::Name => { self.input_mode = true; self.input_buffer = self.config.name.clone(); }
            Field::Description => { self.input_mode = true; self.input_buffer = self.config.description.clone(); }
            Field::PackageName => { self.input_mode = true; self.input_buffer = self.config.package_name.clone(); }
            _ => {}
        }
    }

    fn finish_edit(&mut self) {
        if self.input_mode {
            match self.current_field {
                Field::GroupId => self.config.group_id = self.input_buffer.clone(),
                Field::ArtifactId => self.config.artifact_id = self.input_buffer.clone(),
                Field::Name => self.config.name = self.input_buffer.clone(),
                Field::Description => self.config.description = self.input_buffer.clone(),
                Field::PackageName => self.config.package_name = self.input_buffer.clone(),
                _ => {}
            }
            if matches!(self.current_field, Field::GroupId | Field::ArtifactId) {
                self.refresh_package_name();
            }
            self.input_mode = false;
            self.input_buffer.clear();
        }
    }

    fn refresh_package_name(&mut self) {
        if self.current_field != Field::PackageName {
            self.config.package_name = format!("{}.{}", self.config.group_id, self.config.artifact_id).replace("-", "");
        }
    }

    fn dependency_options(&self) -> Vec<String> {
        let mut list: Vec<String> = self.dependencies.dependencies.keys().cloned().collect();
        list.sort();
        if self.deps_search.trim().is_empty() {
            return list;
        }
        let needle = self.deps_search.to_lowercase();
        list.into_iter().filter(|name| name.to_lowercase().contains(&needle)).collect()
    }

    fn toggle_dependency(&mut self) {
        let options = self.dependency_options();
        if let Some(selected) = self.deps_list_state.selected() {
            if selected < options.len() {
                let name = options[selected].clone();
                if self.selected_deps.contains(&name) {
                    self.selected_deps.remove(&name);
                } else {
                    self.selected_deps.insert(name);
                }
                self.config.dependencies = self.selected_deps.iter().cloned().collect::<Vec<_>>().join(",");
            }
        }
    }
}

// --- RENDERING ---

fn ui(f: &mut Frame<'_>, app: &mut App) {
    // Global Background
    let size = f.area();
    f.render_widget(Block::default().style(Style::default().bg(BG_COLOR)), size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main Content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    render_header(f, chunks[0]);
    render_main_content(f, app, chunks[1]);
    render_footer(f, app, chunks[2]);

    if app.show_popup && !app.input_mode {
        render_popup(f, app);
    }
    if app.show_export_popup {
        render_export_popup(f, app);
    }
    if app.show_config_popup {
        render_config_popup(f, app);
    }
    if app.show_message_popup {
        render_message_popup(f, app);
    }
    if app.input_mode {
        render_input_dialog(f, app);
    }
}

fn render_header(f: &mut Frame<'_>, area: Rect) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled(" spring ", Style::default().fg(TEXT_COLOR).bg(ACCENT_COLOR).add_modifier(Modifier::BOLD)),
        Span::styled(" initializr ", Style::default().fg(TEXT_COLOR).add_modifier(Modifier::BOLD)),
    ]))
    .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(INPUT_BG)))
    .alignment(Alignment::Center);
    f.render_widget(title, area);
}

fn render_main_content(f: &mut Frame<'_>, app: &mut App, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    render_config_column(f, app, cols[0]);
    render_dependencies_column(f, app, cols[1]);
}

fn render_config_column(f: &mut Frame<'_>, app: &mut App, area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5), // Project & Language
            Constraint::Length(4), // Boot Version
            Constraint::Min(10),   // Metadata
        ])
        .split(area);

    // Row 1: Project & Language
    let row1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[0]);
    
    // Project
    let p_opts: Vec<(String, String)> = app
        .capabilities
        .project_type
        .as_ref()
        .map(|x| x.values.iter().map(|v| (v.id.clone(), v.name.clone())).collect())
        .unwrap_or_default();
    draw_radio_section(f, app, "Project", &p_opts, &app.config.project_type, Field::ProjectType, row1[0]);

    // Language
    let l_opts: Vec<(String, String)> = app
        .capabilities
        .language
        .as_ref()
        .map(|x| x.values.iter().map(|v| (v.id.clone(), v.name.clone())).collect())
        .unwrap_or_default();
    draw_radio_section(f, app, "Language", &l_opts, &app.config.language, Field::Language, row1[1]);

    // Row 2: Boot Version
    let b_opts: Vec<(String, String)> = app
        .capabilities
        .boot_version
        .as_ref()
        .map(|x| x.values.iter().map(|v| (v.id.clone(), v.name.clone())).collect())
        .unwrap_or_default();
    draw_radio_section(f, app, "Spring Boot", &b_opts, &app.config.boot_version, Field::BootVersion, rows[1]);

    // Row 3: Metadata
    render_metadata_section(f, app, rows[2]);
}

fn render_metadata_section(f: &mut Frame<'_>, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(Span::styled("Project Metadata", Style::default().fg(TEXT_COLOR).add_modifier(Modifier::BOLD)))
        .style(Style::default().bg(PANEL_BG))
        .padding(ratatui::widgets::Padding::new(0, 0, 0, 2));
    f.render_widget(block.clone(), area);

    let inner_area = block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Group
            Constraint::Length(3), // Artifact
            Constraint::Length(3), // Name
            Constraint::Length(3), // Desc
            Constraint::Length(3), // Package
            Constraint::Length(4), // Packaging & Java
            Constraint::Length(4), // Config Format
        ])
        .split(inner_area);

    draw_text_input(f, app, "Group", &app.config.group_id, Field::GroupId, chunks[0]);
    draw_text_input(f, app, "Artifact", &app.config.artifact_id, Field::ArtifactId, chunks[1]);
    draw_text_input(f, app, "Name", &app.config.name, Field::Name, chunks[2]);
    draw_text_input(f, app, "Description", &app.config.description, Field::Description, chunks[3]);
    draw_text_input(f, app, "Package name", &app.config.package_name, Field::PackageName, chunks[4]);

    // Packaging & Java split
    let row_last = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[5]);
    
    let pack_opts: Vec<(String, String)> = app
        .capabilities
        .packaging
        .as_ref()
        .map(|x| x.values.iter().map(|v| (v.id.clone(), v.name.clone())).collect())
        .unwrap_or_default();
    draw_radio_section(f, app, "Packaging", &pack_opts, &app.config.packaging, Field::Packaging, row_last[0]);
    
    let j_opts: Vec<(String, String)> = app
        .capabilities
        .java_version
        .as_ref()
        .map(|x| x.values.iter().map(|v| (v.id.clone(), v.name.clone())).collect())
        .unwrap_or_default();
    draw_radio_section(f, app, "Java", &j_opts, &app.config.java_version.to_string(), Field::JavaVersion, row_last[1]);

    // Configuration Format
    let config_opts = vec![
        ("properties".to_string(), "Properties".to_string()),
        ("yaml".to_string(), "YAML".to_string()),
    ];
    draw_radio_section(f, app, "Config Format", &config_opts, &app.config.configuration_file_format, Field::ConfigurationFormat, chunks[6]);
}

fn draw_radio_section(f: &mut Frame<'_>, app: &App, title: &str, options: &[(String, String)], current_val: &str, field: Field, area: Rect) {
    let is_active_field = app.current_field == field && app.active_pane == ActivePane::Config;
    
    // Title styling
    let title_style = if is_active_field { Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD) } else { Style::default().fg(TEXT_COLOR).add_modifier(Modifier::BOLD) };
    
    let block = Block::default().padding(ratatui::widgets::Padding::new(0, 1, 0, 0));
    f.render_widget(block, area);

    // Render Title manually at top
    f.render_widget(Paragraph::new(title).style(title_style), Rect { x: area.x, y: area.y, width: area.width, height: 1 });

    // Render Options
    // Since we want to mimic the UI, we list them.
    // If it's a long list (like Boot versions), we wrap or truncate for the view.
    let mut spans = Vec::new();
    for (id, name) in options.iter().take(6) { // Limit display to avoid overflow
        let is_selected = id == current_val;
        let radio = if is_selected { "(●)" } else { "( )" };
        let style = if is_selected { Style::default().fg(TEXT_COLOR).add_modifier(Modifier::BOLD) } else { Style::default().fg(MUTED_COLOR) };
        let radio_style = if is_selected { Style::default().fg(ACCENT_COLOR) } else { Style::default().fg(MUTED_COLOR) };

        spans.push(Span::styled(radio, radio_style));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(name, style));
        spans.push(Span::raw("  "));
    }

    let p = Paragraph::new(Line::from(spans)).wrap(Wrap { trim: true });
    f.render_widget(p, Rect { x: area.x, y: area.y + 2, width: area.width, height: area.height.saturating_sub(2) });
}

fn draw_text_input(f: &mut Frame<'_>, app: &App, label: &str, value: &str, field: Field, area: Rect) {
    let is_focused = app.current_field == field && app.active_pane == ActivePane::Config;
    
    let chunks = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Length(15), Constraint::Min(10)]).split(area);
    
    // Label
    let label_style = if is_focused { Style::default().fg(ACCENT_COLOR) } else { Style::default().fg(TEXT_COLOR) };
    f.render_widget(Paragraph::new(label).style(label_style).alignment(Alignment::Left), chunks[0]);

    // Input Box
    let border_style = if is_focused { Style::default().fg(ACCENT_COLOR) } else { Style::default().fg(INPUT_BG) };
    let text_style = if is_focused { Style::default().fg(Color::White) } else { Style::default().fg(Color::Gray) };
    
    let input_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(border_style);
    
    let val_display = if is_focused && app.input_mode {
        format!("{}█", app.input_buffer) // Cursor
    } else {
        value.to_string()
    };

    // Compact the height to 2 to ensure the bottom border (underline) is right below the text
    let mut input_area = chunks[1];
    input_area.height = 2;

    f.render_widget(Paragraph::new(val_display).block(input_block).style(text_style), input_area);
}

fn render_dependencies_column(f: &mut Frame<'_>, app: &mut App, area: Rect) {
    let border_style = if app.active_pane == ActivePane::Dependencies {
        Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(INPUT_BG)
    };
    let block = Block::default()
        .borders(Borders::LEFT)
        .border_style(border_style)
        .padding(ratatui::widgets::Padding::new(1, 1, 1, 1));
    f.render_widget(block.clone(), area);
    let inner = block.inner(area);

    let chunks = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(3), Constraint::Min(5)]).split(inner);

    // Header
    f.render_widget(Paragraph::new("Dependencies").style(Style::default().fg(TEXT_COLOR).add_modifier(Modifier::BOLD)), chunks[0]);

    // Search Bar
    let search_style = if !app.deps_search.is_empty() { Style::default().fg(Color::White) } else { Style::default().fg(MUTED_COLOR) };
    let search_txt = if app.deps_search.is_empty() { "Press 'Tab' then type to search..." } else { &app.deps_search };
    f.render_widget(
        Paragraph::new(format!(" {}", search_txt))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(INPUT_BG)).border_type(BorderType::Rounded)
    .padding(ratatui::widgets::Padding::new(1,1,0,0)))
        .style(search_style), 
        chunks[1]
    );

    // List
    let options = app.dependency_options();
    let items: Vec<ListItem> = options.iter().map(|name| {
        let is_selected = app.selected_deps.contains(name);
        // let check = if is_selected { " [x] " } else { " [ ] " };
        let check = if is_selected { " 󱧕 " } else { " 󰏖 " };
        let style = if is_selected { Style::default().fg(ACCENT_COLOR) } else { Style::default().fg(TEXT_COLOR) };
        ListItem::new(format!("{}{}", check, name)).style(style)
    }).collect();

    let list = List::new(items)
        .highlight_style(Style::default().bg(INPUT_BG).add_modifier(Modifier::BOLD))
        .highlight_symbol("󰁕 ");
    
    f.render_stateful_widget(list, chunks[2], &mut app.deps_list_state);
}

fn render_footer(f: &mut Frame<'_>, app: &App, area: Rect) {
    let block = Block::default().style(Style::default().bg(INPUT_BG));
    f.render_widget(block, area);

    let is_gen_selected = app.current_field == Field::Generate;
    let is_exp_selected = app.current_field == Field::Export;
    
    let gen_style = if is_gen_selected {
        Style::default().bg(ACCENT_COLOR).fg(Color::White).add_modifier(Modifier::BOLD)
    } else {
        Style::default().bg(MUTED_COLOR).fg(Color::Black)
    };

    let exp_style = if is_exp_selected {
        Style::default().bg(ACCENT_COLOR).fg(Color::White).add_modifier(Modifier::BOLD)
    } else {
        Style::default().bg(MUTED_COLOR).fg(Color::Black)
    };

    let gen_txt = if is_gen_selected { " GENERATE (Enter) " } else { " GENERATE " };
    let exp_txt = if is_exp_selected { " EXPORT (Enter) " } else { " EXPORT " };

    let chunks = Layout::default().direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(40), 
            Constraint::Length(18), // Export
            Constraint::Length(20)  // Generate
        ]).split(area);

    // Split left section vertically for keybindings label + status
    let left_section = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // "keybindings" label
            Constraint::Length(1), // status message
        ])
        .split(chunks[0]);

    f.render_widget(
        Paragraph::new(" keybindings")
            .alignment(Alignment::Center)
            .style(Style::default().fg(MUTED_COLOR).add_modifier(Modifier::DIM)),
        left_section[0]
    );
    
    f.render_widget(
        Paragraph::new(app.status_message.as_str())
            .alignment(Alignment::Center)
            .style(Style::default().fg(TEXT_COLOR)),
        left_section[1]
    );
    
    f.render_widget(Paragraph::new(exp_txt).alignment(Alignment::Center).style(exp_style)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(BG_COLOR))), chunks[1]);

    f.render_widget(Paragraph::new(gen_txt).alignment(Alignment::Center).style(gen_style)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(BG_COLOR))), chunks[2]);
}

fn render_popup(f: &mut Frame<'_>, app: &mut App) {
    let options = app.get_current_options();
    if options.is_empty() { return; }
    
    let area = centered_rect(60, 20, f.area());
    f.render_widget(Clear, area);
    f.render_widget(Block::default().borders(Borders::ALL).style(Style::default().bg(BG_COLOR).fg(ACCENT_COLOR)).title(" Select Option "), area);

    let items: Vec<ListItem> = options.iter().map(|opt| ListItem::new(format!("  {}", opt))).collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
        .highlight_style(Style::default().bg(ACCENT_COLOR).fg(Color::Black));
    
    f.render_stateful_widget(list, area.inner(ratatui::layout::Margin { vertical: 2, horizontal: 2 }), &mut app.list_state);
}

fn render_export_popup(f: &mut Frame<'_>, app: &mut App) {
    let area = centered_rect(60, 40, f.area());
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(BG_COLOR).fg(TEXT_COLOR))
        .title(" Export Configuration ");
    f.render_widget(block.clone(), area);

    let inner = block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2), // Filename Label
            Constraint::Length(2), // Input
            Constraint::Length(2), // Format Label
            Constraint::Length(3), // options
            Constraint::Min(1),
            Constraint::Length(1), // Help
        ])
        .split(inner);

    // Filename
    let name_label_style = if app.export_focus_filename { Style::default().fg(ACCENT_COLOR) } else { Style::default().fg(TEXT_COLOR) };
    f.render_widget(Paragraph::new("Filename (no extension):").style(name_label_style), chunks[0]);
    
    let name_style = if app.export_focus_filename { Style::default().fg(Color::White) } else { Style::default().fg(MUTED_COLOR) };
    let border_style = if app.export_focus_filename { Style::default().fg(ACCENT_COLOR) } else { Style::default().fg(INPUT_BG) };
    
    let name_cursor = if app.export_focus_filename { format!("{}█", app.export_filename) } else { app.export_filename.clone() };
    f.render_widget(Paragraph::new(name_cursor).block(Block::default().borders(Borders::BOTTOM).border_style(border_style)).style(name_style), chunks[1]);

    // Format
    let fmt_label_style = if !app.export_focus_filename { Style::default().fg(ACCENT_COLOR) } else { Style::default().fg(TEXT_COLOR) };
    f.render_widget(Paragraph::new("Format:").style(fmt_label_style), chunks[2]);
    
    let fmts = ["YAML", "JSON", "TOML"];
    let mut spans = Vec::new();
    for (i, fmt) in fmts.iter().enumerate() {
        let is_focused = !app.export_focus_filename;
        let is_selected = i == app.export_format_idx;
        
        let radio = if is_selected { "(●)" } else { "( )" };
        let mut color = if is_selected { ACCENT_COLOR } else { MUTED_COLOR };
        if is_focused && is_selected { color = Color::Yellow; } // Highlight selection if focused
        if is_focused && !is_selected { color = Color::White; } // Highlight potential selection options

        spans.push(Span::styled(radio, Style::default().fg(color)));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(*fmt, Style::default().fg(color)));
        spans.push(Span::raw("   "));
    }
    
    f.render_widget(Paragraph::new(Line::from(spans)), chunks[3]);

    // Help
    f.render_widget(Paragraph::new("Tab: Focus | ← →: Format | Enter: Save | Esc: Cancel").alignment(Alignment::Center).style(Style::default().fg(MUTED_COLOR)), chunks[5]);
}

fn render_config_popup(f: &mut Frame<'_>, app: &mut App) {
    let area = centered_rect(40, 20, f.area());
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(BG_COLOR).fg(TEXT_COLOR))
        .title(" TUI Configuration ");
    f.render_widget(block.clone(), area);

    let inner = block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Option
            Constraint::Min(1),
            Constraint::Length(1), // Help
        ])
        .split(inner);

    let check = if app.extract_project { "[x]" } else { "[ ]" };
    let color = if app.extract_project { ACCENT_COLOR } else { TEXT_COLOR };
    
    let line = Line::from(vec![
        Span::styled(check, Style::default().fg(color).add_modifier(Modifier::BOLD)),
        Span::raw(" Extract project zip after download"),
    ]);

    f.render_widget(Paragraph::new(line), chunks[0]);
    f.render_widget(Paragraph::new("Space/Enter: Toggle | Esc: Close").alignment(Alignment::Center).style(Style::default().fg(MUTED_COLOR)), chunks[2]);
}

fn render_message_popup(f: &mut Frame<'_>, app: &mut App) {
    let area = centered_rect(50, 20, f.area());
    f.render_widget(Clear, area);
    let title_color = if app.message_popup_is_error { Color::Red } else { ACCENT_COLOR };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(BG_COLOR).fg(TEXT_COLOR))
        .title(Span::styled(&app.message_popup_title, Style::default().fg(title_color).add_modifier(Modifier::BOLD)));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let p = Paragraph::new(app.message_popup_text.as_str())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .style(Style::default().fg(TEXT_COLOR));
    
    // Vertically center text
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1), // Help footer
        ])
        .split(inner_area);
    
    f.render_widget(p, chunks[0]);
    f.render_widget(Paragraph::new("Press Enter to Close").alignment(Alignment::Center).style(Style::default().fg(MUTED_COLOR)), chunks[1]);
}

fn render_input_dialog(_f: &mut Frame<'_>, _app: &mut App) {
    // Input is rendered inline in draw_text_input, but we need to block other rendering if strictly modal.
    // However, for this UI, inline editing looks better. We just consume keys.
    // Visual feedback is already in draw_text_input via cursor char.
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ]).split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ]).split(popup_layout[1])[1]
}

// --- RUN LOOP ---

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching Spring Initializr capabilities...");
    let capabilities = api::get_capabilities().await?;
    println!("Fetching available dependencies...");
    let dependencies = api::get_dependencies().await?;
    println!("Starting TUI...");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(capabilities, dependencies);
    let res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }
    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn std::error::Error>>
where
    <B as Backend>::Error: std::error::Error + Send + Sync + 'static,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if app.input_mode {
                match key.code {
                    KeyCode::Enter => app.finish_edit(),
                    KeyCode::Esc => { app.input_mode = false; app.input_buffer.clear(); }
                    KeyCode::Char(c) => app.input_buffer.push(c),
                    KeyCode::Backspace => { app.input_buffer.pop(); }
                    _ => {}
                }
            } else if app.show_popup {
                match key.code {
                    KeyCode::Esc => app.show_popup = false,
                    KeyCode::Down | KeyCode::Char('j') => {
                        let i = app.list_state.selected().unwrap_or(0);
                        let len = app.get_current_options().len();
                        if len > 0 { app.list_state.select(Some((i + 1) % len)); }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        let i = app.list_state.selected().unwrap_or(0);
                        let len = app.get_current_options().len();
                        if len > 0 { app.list_state.select(Some(if i == 0 { len - 1 } else { i - 1 })); }
                    }
                    KeyCode::Enter => app.select_option(),
                    _ => {}
                }
            } else if app.show_export_popup {
                match key.code {
                   KeyCode::Esc => app.show_export_popup = false,
                   KeyCode::Tab => app.export_focus_filename = !app.export_focus_filename,
                   KeyCode::Left | KeyCode::Char('h') => {
                       if !app.export_focus_filename {
                           if app.export_format_idx > 0 { app.export_format_idx -= 1; }
                       }
                   }
                   KeyCode::Right | KeyCode::Char('l') => {
                       if !app.export_focus_filename {
                           if app.export_format_idx < 2 { app.export_format_idx += 1; }
                       }
                   }
                   KeyCode::Char(c) => {
                       if app.export_focus_filename {
                           app.export_filename.push(c);
                       }
                   }
                   KeyCode::Backspace => {
                       if app.export_focus_filename {
                           app.export_filename.pop();
                       }
                   }
                   KeyCode::Enter => {
                       let fmt = match app.export_format_idx {
                           0 => FileType::Yaml,
                           1 => FileType::Json,
                           _ => FileType::Toml,
                       };
                       let name = if app.export_filename.is_empty() { None } else { Some(app.export_filename.clone()) };
                       
                       match generator::generate_project_config_file(&app.config, fmt, name) {
                           Ok(_) => {
                                app.status_message = "Config Exported!".to_string();
                                app.show_message_popup = true;
                                app.message_popup_title = "Success".to_string();
                                app.message_popup_text = "Configuration file exported successfully.".to_string();
                                app.message_popup_is_error = false;
                           },
                           Err(e) => {
                                app.status_message = format!("Export Error: {}", e);
                                app.show_message_popup = true;
                                app.message_popup_title = "Export Failed".to_string();
                                app.message_popup_text = e.to_string();
                                app.message_popup_is_error = true;
                           },
                       }
                       app.show_export_popup = false;
                   }
                   _ => {}
                }
            } else if app.show_config_popup {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('C') => app.show_config_popup = false,
                    KeyCode::Enter | KeyCode::Char(' ') => app.extract_project = !app.extract_project,
                     _ => {}
                }
            } else if app.show_message_popup {
                match key.code {
                    KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => app.show_message_popup = false,
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                    KeyCode::Char('C') => app.show_config_popup = true,
                    KeyCode::Tab => app.toggle_pane(),
                    KeyCode::Down | KeyCode::Char('j') => {
                        match app.active_pane {
                            ActivePane::Config => app.next_field(),
                            ActivePane::Dependencies => app.next_dependency(),
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        match app.active_pane {
                            ActivePane::Config => app.previous_field(),
                            ActivePane::Dependencies => app.previous_dependency(),
                        }
                    }
                    KeyCode::Char(' ') => {
                        if app.active_pane == ActivePane::Dependencies {
                            app.toggle_dependency();
                        }
                    }
                    KeyCode::Enter => {
                        if app.current_field == Field::Generate && app.active_pane == ActivePane::Config {
                            app.status_message = "Generating...".to_string();
                            terminal.draw(|f| ui(f, app))?;
                            match generator::generate_project(&app.config, app.extract_project).await {
                                Ok(_) =>  {
                                    app.status_message = "Success! Saved.".to_string();
                                    app.show_message_popup = true;
                                    app.message_popup_title = "Success".to_string();
                                    app.message_popup_text = format!("Project '{}' generated successfully!", app.config.artifact_id);
                                    app.message_popup_is_error = false;
                                },
                                Err(e) => {
                                    app.status_message = format!("Error: {}", e);
                                    app.show_message_popup = true;
                                    app.message_popup_title = "Generation Failed".to_string();
                                    app.message_popup_text = e.to_string();
                                    app.message_popup_is_error = true;
                                },
                            }
                        } else if app.current_field == Field::Export && app.active_pane == ActivePane::Config {
                            app.show_export_popup = true;
                            app.export_focus_filename = true;
                        } else if app.active_pane == ActivePane::Config {
                            if !app.get_current_options().is_empty() {
                                app.show_popup = true;
                                app.list_state.select(Some(0));
                            } else {
                                app.start_edit();
                            }
                        } else if app.active_pane == ActivePane::Dependencies {
                            app.toggle_dependency();
                        }
                    }
                    KeyCode::Backspace => {
                         if app.active_pane == ActivePane::Dependencies && !app.deps_search.is_empty() {
                             app.deps_search.pop();
                             app.deps_list_state.select(Some(0));
                         }
                    }
                    KeyCode::Char(c) => {
                        if !c.is_control() {
                            if app.active_pane == ActivePane::Dependencies {
                                app.deps_search.push(c);
                                app.deps_list_state.select(Some(0));
                            } else if c == 'g' || c == 'G' {
                                // optional shortcuts for Config pane?
                                // Let's keep 'g' for generate if in Config pane
                                if app.active_pane == ActivePane::Config {
                                    app.current_field = Field::Generate;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}