use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use insta::assert_debug_snapshot;

fn draw_pretty_prompt(terminal: &mut Terminal<TestBackend>) -> anyhow::Result<()> {
    terminal.draw(|f| {
        let area: Rect = f.area();

        // Bordered prompt block like pretty mode
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block.clone(), area);

        let inner = block.inner(area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1)].as_ref())
            .split(inner);

        let prompt = Paragraph::new("> ");
        f.render_widget(prompt, chunks[0]);
    })?;
    Ok(())
}

#[test]
fn pretty_prompt_snapshot() -> anyhow::Result<()> {
    // Fixed terminal size ensures deterministic snapshot
    let backend = TestBackend::new(40, 6);
    let mut terminal = Terminal::new(backend)?;

    draw_pretty_prompt(&mut terminal)?;

    // Read buffer for snapshot
    let buf: &Buffer = terminal.backend().buffer();
    assert_debug_snapshot!(buf);
    Ok(())
}
