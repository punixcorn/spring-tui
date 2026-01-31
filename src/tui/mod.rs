use crate::api;
use crate::generator;
use crate::types::api::{InitializrCapabilities, InitializrDependencies};
use crate::types::generic::SprintInitConfig;
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
use std::io;

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
    Version,
    ConfigurationFormat,
    Generate,
}

struct App {
    capabilities: InitializrCapabilities,
    dependencies: InitializrDependencies,
    config: SprintInitConfig,
    current_field: Field,
    list_state: ListState,
    input_mode: bool,
    input_buffer: String,
    status_message: String,
    show_popup: bool,
}

impl App {
    fn new(capabilities: InitializrCapabilities, dependencies: InitializrDependencies) -> Self {
        // Initialize config with defaults from capabilities
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
                .unwrap_or_else(|| "4.0.2".to_string()),
            version: capabilities
                .version
                .as_ref()
                .map(|v| v.default.clone())
                .unwrap_or_else(|| "0.0.1-SNAPSHOT".to_string()),
        };

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        App {
            capabilities,
            dependencies,
            config,
            current_field: Field::ProjectType,
            list_state,
            input_mode: false,
            input_buffer: String::new(),
            status_message: "↑↓ Navigate | Enter Select/Edit | Tab Choose | G Generate | Q Quit".to_string(),
            show_popup: false,
        }
    }

    fn next_field(&mut self) {
        self.current_field = match self.current_field {
            Field::ProjectType => Field::Language,
            Field::Language => Field::BootVersion,
            Field::BootVersion => Field::Packaging,
            Field::Packaging => Field::JavaVersion,
            Field::JavaVersion => Field::GroupId,
            Field::GroupId => Field::ArtifactId,
            Field::ArtifactId => Field::Name,
            Field::Name => Field::Description,
            Field::Description => Field::PackageName,
            Field::PackageName => Field::Version,
            Field::Version => Field::ConfigurationFormat,
            Field::ConfigurationFormat => Field::Generate,
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
            Field::Packaging => Field::BootVersion,
            Field::JavaVersion => Field::Packaging,
            Field::GroupId => Field::JavaVersion,
            Field::ArtifactId => Field::GroupId,
            Field::Name => Field::ArtifactId,
            Field::Description => Field::Name,
            Field::PackageName => Field::Description,
            Field::Version => Field::PackageName,
            Field::ConfigurationFormat => Field::Version,
            Field::Generate => Field::ConfigurationFormat,
        };
        self.list_state.select(Some(0));
        self.show_popup = false;
    }

    fn get_current_options(&self) -> Vec<String> {
        match self.current_field {
            Field::ProjectType => self
                .capabilities
                .project_type
                .as_ref()
                .map(|pt| pt.values.iter().map(|v| v.id.clone()).collect())
                .unwrap_or_default(),
            Field::Language => self
                .capabilities
                .language
                .as_ref()
                .map(|l| l.values.iter().map(|v| v.id.clone()).collect())
                .unwrap_or_default(),
            Field::BootVersion => self
                .capabilities
                .boot_version
                .as_ref()
                .map(|bv| bv.values.iter().map(|v| v.id.clone()).collect())
                .unwrap_or_default(),
            Field::Packaging => self
                .capabilities
                .packaging
                .as_ref()
                .map(|p| p.values.iter().map(|v| v.id.clone()).collect())
                .unwrap_or_default(),
            Field::JavaVersion => self
                .capabilities
                .java_version
                .as_ref()
                .map(|jv| jv.values.iter().map(|v| v.id.clone()).collect())
                .unwrap_or_default(),
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
            }
        }
    }

    fn start_edit(&mut self) {
        match self.current_field {
            Field::GroupId => {
                self.input_mode = true;
                self.input_buffer = self.config.group_id.clone();
            }
            Field::ArtifactId => {
                self.input_mode = true;
                self.input_buffer = self.config.artifact_id.clone();
            }
            Field::Name => {
                self.input_mode = true;
                self.input_buffer = self.config.name.clone();
            }
            Field::Description => {
                self.input_mode = true;
                self.input_buffer = self.config.description.clone();
            }
            Field::PackageName => {
                self.input_mode = true;
                self.input_buffer = self.config.package_name.clone();
            }
            Field::Version => {
                self.input_mode = true;
                self.input_buffer = self.config.version.clone();
            }
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
                Field::Version => self.config.version = self.input_buffer.clone(),
                _ => {}
            }
            self.input_mode = false;
            self.input_buffer.clear();
        }
    }
}

fn ui(f: &mut Frame<'_>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🍃 Spring Initializr - Project Generator")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Green)));
    f.render_widget(title, chunks[0]);

    // Main content
    render_main_content(f, app, chunks[1]);

    // Status bar
    let status_style = if app.status_message.contains("Error") {
        Style::default().fg(Color::Red)
    } else if app.status_message.contains("successfully") {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Cyan)
    };
    
    let status = Paragraph::new(app.status_message.as_str())
        .style(status_style)
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded));
    f.render_widget(status, chunks[2]);

    // Show popup if needed
    if app.show_popup && !app.input_mode {
        render_popup(f, app);
    }

    // Show input dialog if in input mode
    if app.input_mode {
        render_input_dialog(f, app);
    }
}

fn render_main_content(f: &mut Frame<'_>, app: &App, area: Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(3),
        ])
        .split(area);

    render_field(f, app, "Project Type", &app.config.project_type, Field::ProjectType, main_chunks[0], "🏗️");
    render_field(f, app, "Language", &app.config.language, Field::Language, main_chunks[1], "💻");
    render_field(f, app, "Boot Version", &app.config.boot_version, Field::BootVersion, main_chunks[2], "🚀");
    render_field(f, app, "Packaging", &app.config.packaging, Field::Packaging, main_chunks[3], "📦");
    render_field(f, app, "Java Version", &app.config.java_version.to_string(), Field::JavaVersion, main_chunks[4], "☕");
    render_field(f, app, "Group ID", &app.config.group_id, Field::GroupId, main_chunks[5], "🏢");
    render_field(f, app, "Artifact ID", &app.config.artifact_id, Field::ArtifactId, main_chunks[6], "📝");
    render_field(f, app, "Name", &app.config.name, Field::Name, main_chunks[7], "✏️");
    render_field(f, app, "Description", &app.config.description, Field::Description, main_chunks[8], "📄");
    render_field(f, app, "Package Name", &app.config.package_name, Field::PackageName, main_chunks[9], "📂");
    render_field(f, app, "Version", &app.config.version, Field::Version, main_chunks[10], "🔢");
    render_field(f, app, "Config Format", &app.config.configuration_file_format, Field::ConfigurationFormat, main_chunks[11], "⚙️");
    
    // Generate button
    let is_selected = app.current_field == Field::Generate;
    let border_style = if is_selected {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    
    let text_style = if is_selected {
        Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    
    let generate_text = if is_selected {
        ">>> ⚡ GENERATE PROJECT ⚡ <<<"
    } else {
        "⚡ Generate Project"
    };
    
    let generate_button = Paragraph::new(generate_text)
        .style(text_style)
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(if is_selected { BorderType::Double } else { BorderType::Rounded })
            .style(border_style));
    
    f.render_widget(generate_button, main_chunks[12]);
}

fn render_field(f: &mut Frame<'_>, app: &App, label: &str, value: &str, field: Field, area: Rect, icon: &str) {
    let is_selected = app.current_field == field;
    let has_options = !app.get_current_options().is_empty();
    
    let border_style = if is_selected {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    
    let label_style = if is_selected {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Blue)
    };
    
    let value_style = if is_selected {
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    
    let indicator = if is_selected {
        if has_options {
            " [Tab to choose]"
        } else {
            " [Enter to edit]"
        }
    } else {
        ""
    };
    
    let content = Line::from(vec![
        Span::styled(format!("{} {} ", icon, label), label_style),
        Span::raw("│ "),
        Span::styled(value, value_style),
        Span::styled(indicator, Style::default().fg(Color::DarkGray)),
    ]);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(if is_selected { BorderType::Double } else { BorderType::Rounded })
        .style(border_style);
    
    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, area);
}

fn render_popup(f: &mut Frame<'_>, app: &mut App) {
    let options = app.get_current_options();
    if options.is_empty() {
        return;
    }
    
    // Calculate popup size
    let popup_height = (options.len() as u16 + 4).min(20);
    let popup_width = 60;
    
    let area = centered_rect(popup_width, popup_height, f.area());
    
    // Clear the background
    f.render_widget(Clear, area);
    
    // Create items
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let is_current = match app.current_field {
                Field::ProjectType => opt == &app.config.project_type,
                Field::Language => opt == &app.config.language,
                Field::BootVersion => opt == &app.config.boot_version,
                Field::Packaging => opt == &app.config.packaging,
                Field::JavaVersion => opt == &app.config.java_version.to_string(),
                Field::ConfigurationFormat => opt == &app.config.configuration_file_format,
                _ => false,
            };
            
            let style = if is_current {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            let marker = if is_current { "✓ " } else { "  " };
            ListItem::new(format!("{}{}", marker, opt)).style(style)
        })
        .collect();
    
    let field_name = match app.current_field {
        Field::ProjectType => "Project Type",
        Field::Language => "Language",
        Field::BootVersion => "Boot Version",
        Field::Packaging => "Packaging",
        Field::JavaVersion => "Java Version",
        Field::ConfigurationFormat => "Configuration Format",
        _ => "Options",
    };
    
    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(format!(" Select {} ", field_name))
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black).fg(Color::Yellow)))
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    
    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_input_dialog(f: &mut Frame<'_>, app: &mut App) {
    let area = centered_rect(60, 7, f.area());
    
    // Clear the background
    f.render_widget(Clear, area);
    
    let field_name = match app.current_field {
        Field::GroupId => "Group ID",
        Field::ArtifactId => "Artifact ID",
        Field::Name => "Name",
        Field::Description => "Description",
        Field::PackageName => "Package Name",
        Field::Version => "Version",
        _ => "Input",
    };
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(area);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .title(format!(" Edit {} ", field_name))
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::Black).fg(Color::Cyan));
    
    f.render_widget(block, area);
    
    // Input field
    let input = Paragraph::new(app.input_buffer.as_str())
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded));
    f.render_widget(input, chunks[1]);
    
    // Help text
    let help = Paragraph::new("Enter: Save | Esc: Cancel")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Length((r.height.saturating_sub(height)) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Length((r.width.saturating_sub(width)) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch capabilities and dependencies
    println!("Fetching Spring Initializr capabilities...");
    let capabilities = api::get_capabilities().await?;
    
    println!("Fetching available dependencies...");
    let dependencies = api::get_dependencies().await?;
    
    println!("Starting TUI...");

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(capabilities, dependencies);
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> 
where
    <B as Backend>::Error: std::error::Error + Send + Sync + 'static,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if app.input_mode {
                match key.code {
                    KeyCode::Enter => app.finish_edit(),
                    KeyCode::Esc => {
                        app.input_mode = false;
                        app.input_buffer.clear();
                    }
                    KeyCode::Char(c) => app.input_buffer.push(c),
                    KeyCode::Backspace => {
                        app.input_buffer.pop();
                    }
                    _ => {}
                }
            } else if app.show_popup {
                match key.code {
                    KeyCode::Esc => {
                        app.show_popup = false;
                        app.list_state.select(Some(0));
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        let options = app.get_current_options();
                        if !options.is_empty() {
                            let current = app.list_state.selected().unwrap_or(0);
                            let next = (current + 1) % options.len();
                            app.list_state.select(Some(next));
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        let options = app.get_current_options();
                        if !options.is_empty() {
                            let current = app.list_state.selected().unwrap_or(0);
                            let prev = if current == 0 {
                                options.len() - 1
                            } else {
                                current - 1
                            };
                            app.list_state.select(Some(prev));
                        }
                    }
                    KeyCode::Enter => {
                        app.select_option();
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next_field(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous_field(),
                    KeyCode::Tab => {
                        if !app.get_current_options().is_empty() {
                            app.show_popup = true;
                            app.list_state.select(Some(0));
                        }
                    }
                    KeyCode::Enter => {
                        if app.current_field == Field::Generate {
                            // Generate the project
                            app.status_message = "⏳ Generating project...".to_string();
                            terminal.draw(|f| ui(f, app))?;
                            
                            match generator::generate_project(&app.config).await {
                                Ok(_) => {
                                    app.status_message = "✅ Project generated successfully! Press 'Q' to quit.".to_string();
                                }
                                Err(e) => {
                                    app.status_message = format!("❌ Error: {}", e);
                                }
                            }
                        } else if !app.get_current_options().is_empty() {
                            app.show_popup = true;
                            app.list_state.select(Some(0));
                        } else {
                            app.start_edit();
                        }
                    }
                    KeyCode::Char('g') | KeyCode::Char('G') => {
                        // Quick generate shortcut
                        app.current_field = Field::Generate;
                        app.status_message = "⚡ Ready to generate. Press Enter to confirm.".to_string();
                    }
                    _ => {}
                }
            }
        }
    }
}