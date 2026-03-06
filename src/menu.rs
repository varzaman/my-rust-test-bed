use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

#[derive(Clone, Copy, Debug)]
pub struct MenuStyle {
    pub help_text: &'static str,
    pub selected_foreground: Color,
    pub selected_background: Color,
    pub selected_bold: bool,
    pub selected_prefix: &'static str,
    pub unselected_prefix: &'static str,
}

pub const DEFAULT_MENU_STYLE: MenuStyle = MenuStyle {
    help_text: "Use up/down arrows to move, Enter to select.",
    selected_foreground: Color::Black,
    selected_background: Color::Cyan,
    selected_bold: true,
    selected_prefix: "> ",
    unselected_prefix: "  ",
};

const STATIC_MENU_ITEMS: [&str; 1] = ["Quit."];

pub fn render_menu(items: &[&str]) -> usize {
    render_menu_with_style(items, None)
}

pub fn render_menu_with_style(items: &[&str], style: Option<MenuStyle>) -> usize {
    let style = style.unwrap_or(DEFAULT_MENU_STYLE);

    if items.is_empty() {
        println!("No menu items available.");
        return 0;
    }

    let menu_labels = build_menu_labels(items);
    let dynamic_count = menu_labels.len();
    let total_items = dynamic_count + STATIC_MENU_ITEMS.len();
    let mut selected = 0usize;
    let mut stdout = io::stdout();

    enable_raw_mode().expect("Failed to enable raw mode");
    execute!(&mut stdout, EnterAlternateScreen, Hide).expect("Failed to initialize terminal UI");

    drain_pending_events();

    loop {
        render_menu_frame(&mut stdout, &menu_labels, selected, style)
            .expect("Failed to render menu");
        stdout.flush().expect("Failed to flush stdout");

        if let Event::Key(key_event) = event::read().expect("Failed to read key") {
            if key_event.kind != KeyEventKind::Press && key_event.kind != KeyEventKind::Repeat {
                continue;
            }
            match key_event.code {
                KeyCode::Enter => {
                    if selected >= dynamic_count {
                        restore_terminal(&mut stdout);
                        std::process::exit(0);
                    }
                    break;
                }
                KeyCode::Up => selected = selected.saturating_sub(1),
                KeyCode::Down => {
                    if selected + 1 < total_items {
                        selected += 1;
                    }
                }
                _ => {}
            }
        }
    }

    restore_terminal(&mut stdout);
    selected
}

fn build_menu_labels(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(index, item)| format!("{}. {}", index + 1, item))
        .collect()
}

fn render_menu_option(
    is_selected: bool,
    stdout: &mut io::Stdout,
    row: u16,
    label: &str,
    style: &MenuStyle,
) -> io::Result<()> {
    queue!(stdout, MoveTo(0, row), Clear(ClearType::CurrentLine))?;

    if is_selected {
        queue!(
            stdout,
            SetForegroundColor(style.selected_foreground),
            SetBackgroundColor(style.selected_background),
        )?;
        if style.selected_bold {
            queue!(stdout, SetAttribute(Attribute::Bold))?;
        }
        queue!(stdout, Print(style.selected_prefix), Print(label))?;
        queue!(stdout, ResetColor, SetAttribute(Attribute::Reset),)?;
    } else {
        queue!(stdout, Print(style.unselected_prefix), Print(label))?;
    }

    Ok(())
}

fn render_menu_frame(
    stdout: &mut io::Stdout,
    dynamic_menu_labels: &[String],
    selected: usize,
    style: MenuStyle,
) -> io::Result<()> {
    queue!(
        stdout,
        MoveTo(0, 0),
        Clear(ClearType::All),
        Print(style.help_text)
    )?;

    let menu_start_row = 2u16;

    // We create the dynamic menu labels.
    for (index, label) in dynamic_menu_labels.iter().enumerate() {
        let row = menu_start_row + index as u16;
        render_menu_option(index == selected, stdout, row, label, &style)?;
    }

    // We create the static menu labels we know we will always want.
    for (index, label) in STATIC_MENU_ITEMS.iter().enumerate() {
        let absolute_index = dynamic_menu_labels.len() + index;
        let row = menu_start_row + absolute_index as u16;
        let indexed_label = format!("{}. {}", absolute_index + 1, label);
        render_menu_option(
            absolute_index == selected,
            stdout,
            row,
            &indexed_label,
            &style,
        )?;
    }
    Ok(())
}

fn restore_terminal(stdout: &mut io::Stdout) {
    disable_raw_mode().expect("Failed to disable raw mode");
    execute!(stdout, Show, LeaveAlternateScreen).expect("Failed to restore terminal state");
}

fn drain_pending_events() {
    while event::poll(std::time::Duration::from_millis(0)).expect("Failed to poll for events") {
        let _ = event::read().expect("Failed to read stale event");
    }
}

pub fn inline_prompt(prompt: &str) {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout.");
}
