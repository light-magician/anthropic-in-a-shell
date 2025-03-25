use crossterm::{
    cursor, execute, queue,
    style::{self, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{size, Clear, ClearType},
    Result,
};
use std::io::{stdout, Write};

// Style configuration for our UI
pub struct UiStyle {
    pub user_box_fg: Color,
    pub user_box_bg: Color,
    pub model_box_fg: Color,
    pub model_box_bg: Color,
    pub prompt_fg: Color,
    pub status_fg: Color,
    pub border_style: BorderStyle,
}

pub enum BorderStyle {
    Single,
    Double,
    Rounded,
    Bold,
}

impl BorderStyle {
    pub fn get_chars(&self) -> (char, char, char, char, char, char) {
        match self {
            BorderStyle::Single => ('┌', '┐', '└', '┘', '─', '│'),
            BorderStyle::Double => ('╔', '╗', '╚', '╝', '═', '║'),
            BorderStyle::Rounded => ('╭', '╮', '╰', '╯', '─', '│'),
            BorderStyle::Bold => ('┏', '┓', '┗', '┛', '━', '┃'),
        }
    }
}

impl Default for UiStyle {
    fn default() -> Self {
        UiStyle {
            user_box_fg: Color::White,
            user_box_bg: Color::Rgb {
                r: 25,
                g: 25,
                b: 25,
            },
            model_box_fg: Color::White,
            model_box_bg: Color::Rgb { r: 0, g: 30, b: 60 },
            prompt_fg: Color::Cyan,
            status_fg: Color::DarkGrey,
            border_style: BorderStyle::Rounded,
        }
    }
}

pub struct TerminalUi {
    pub style: UiStyle,
    pub term_width: u16,
    pub term_height: u16,
    pub current_model: String,
}

impl TerminalUi {
    pub fn new() -> Result<Self> {
        let (term_width, term_height) = size()?;

        Ok(TerminalUi {
            style: UiStyle::default(),
            term_width,
            term_height,
            current_model: "claude-3-5-haiku-latest".to_string(),
        })
    }

    pub fn set_model(&mut self, model: String) {
        self.current_model = model;
    }

    pub fn update_terminal_size(&mut self) -> Result<()> {
        let (width, height) = size()?;
        self.term_width = width;
        self.term_height = height;
        Ok(())
    }

    pub fn draw_prompt(&self) -> Result<()> {
        let mut stdout = stdout();

        // Get available width
        let available_width = self.term_width as usize;

        // Clear the screen from cursor down
        execute!(stdout, Clear(ClearType::FromCursorDown))?;

        // Draw the model info and prompt
        let model_display = match self.current_model.as_str() {
            "claude-3-5-haiku-latest" => "Claude 3.5 Haiku",
            "claude-3-7-sonnet-latest" => "Claude 3.7 Sonnet",
            other => other,
        };

        let prompt_text = format!("You [{}]: ", model_display);

        queue!(
            stdout,
            SetForegroundColor(self.style.prompt_fg),
            Print(&prompt_text),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    pub fn draw_user_message(&self, message: &str) -> Result<()> {
        let mut stdout = stdout();

        // Get available width accounting for padding
        let available_width = self.term_width as usize - 4; // 2 chars padding on each side

        // Word wrap the message
        let wrapped_lines = self.wrap_text(message, available_width);
        let box_height = wrapped_lines.len() + 2; // +2 for top and bottom borders

        // Box characters
        let (tl, tr, bl, br, h, v) = self.style.border_style.get_chars();

        // Top border
        queue!(
            stdout,
            SetForegroundColor(self.style.user_box_fg),
            SetBackgroundColor(self.style.user_box_bg),
            Print(format!(
                "{}{}{}",
                tl,
                h.to_string().repeat(available_width),
                tr
            )),
            Print("\n")
        )?;

        // Message lines
        for line in &wrapped_lines {
            let padding = " ".repeat(available_width - line.len());
            queue!(
                stdout,
                Print(format!("{} {}{} {}", v, line, padding, v)),
                Print("\n")
            )?;
        }

        // Bottom border
        queue!(
            stdout,
            Print(format!(
                "{}{}{}",
                bl,
                h.to_string().repeat(available_width),
                br
            )),
            ResetColor,
            Print("\n\n")
        )?;

        stdout.flush()?;
        Ok(())
    }

    pub fn draw_model_message(
        &self,
        message: &str,
        tokens_used: Option<(u32, u32)>,
        cost: Option<f64>,
    ) -> Result<()> {
        let mut stdout = stdout();

        // Get available width accounting for padding
        let available_width = self.term_width as usize - 4; // 2 chars padding on each side

        // Word wrap the message
        let wrapped_lines = self.wrap_text(message, available_width);
        let box_height = wrapped_lines.len() + 2; // +2 for top and bottom borders

        // Box characters
        let (tl, tr, bl, br, h, v) = self.style.border_style.get_chars();

        // Draw the model emoji and name
        let model_display = match self.current_model.as_str() {
            "claude-3-5-haiku-latest" => "Claude 3.5 Haiku",
            "claude-3-7-sonnet-latest" => "Claude 3.7 Sonnet",
            other => other,
        };

        queue!(
            stdout,
            SetForegroundColor(self.style.prompt_fg),
            Print(format!("🤖 {}:\n", model_display)),
            ResetColor
        )?;

        // Top border
        queue!(
            stdout,
            SetForegroundColor(self.style.model_box_fg),
            SetBackgroundColor(self.style.model_box_bg),
            Print(format!(
                "{}{}{}",
                tl,
                h.to_string().repeat(available_width),
                tr
            )),
            Print("\n")
        )?;

        // Message lines
        for line in &wrapped_lines {
            let padding = " ".repeat(available_width - line.len());
            queue!(
                stdout,
                Print(format!("{} {}{} {}", v, line, padding, v)),
                Print("\n")
            )?;
        }

        // Bottom border
        queue!(
            stdout,
            Print(format!(
                "{}{}{}",
                bl,
                h.to_string().repeat(available_width),
                br
            )),
            ResetColor,
            Print("\n")
        )?;

        // Display token usage and cost if provided
        if let (Some((input_tokens, output_tokens)), Some(cost)) = (tokens_used, cost) {
            queue!(
                stdout,
                SetForegroundColor(self.style.status_fg),
                Print(format!(
                    "Tokens: {} in, {} out | Cost: ${:.6}\n\n",
                    input_tokens, output_tokens, cost
                )),
                ResetColor
            )?;
        }

        stdout.flush()?;
        Ok(())
    }

    // Word wrapping utility
    pub fn wrap_text(&self, text: &str, width: usize) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 > width {
                // Line would be too long with this word, start a new line
                if !current_line.is_empty() {
                    result.push(current_line);
                    current_line = String::new();
                }

                // Handle words longer than width
                if word.len() > width {
                    // Split long words
                    let mut remaining = word;
                    while !remaining.is_empty() {
                        let (chunk, rest) = if remaining.len() <= width {
                            (remaining, "")
                        } else {
                            remaining.split_at(width)
                        };

                        result.push(chunk.to_string());
                        remaining = rest;
                    }
                } else {
                    current_line = word.to_string();
                }
            } else {
                // Word fits on current line
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
        }

        if !current_line.is_empty() {
            result.push(current_line);
        }

        result
    }

    // Special handling for streaming responses
    pub fn stream_model_message<F>(&self, callback: F) -> Result<()>
    where
        F: FnMut(&str) -> Result<()>,
    {
        // This is a placeholder for a more complex implementation
        // that would handle streaming responses
        Ok(())
    }

    // Initialize UI for a new conversation
    pub fn init_conversation(&self) -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        // Draw a header
        queue!(
            stdout,
            SetForegroundColor(Color::White),
            Print(format!(
                "Claude CLI - Current Model: {}\n",
                self.current_model
            )),
            Print("Type your message or commands (/help, /models, /quit)\n\n"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    // Draw a thinking spinner
    pub fn draw_thinking_spinner(&self) -> Result<()> {
        let mut stdout = stdout();

        queue!(
            stdout,
            SetForegroundColor(self.style.prompt_fg),
            Print("🤖 Thinking..."),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    // Clear the thinking spinner
    pub fn clear_thinking_spinner(&self) -> Result<()> {
        let mut stdout = stdout();

        execute!(
            stdout,
            cursor::SavePosition,
            crossterm::terminal::Clear(ClearType::CurrentLine),
            cursor::RestorePosition
        )?;

        stdout.flush()?;
        Ok(())
    }
}
