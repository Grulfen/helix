use crate::compositor::{Component, Context};
use crate::ui;
use helix_vcs::Hunk;
use helix_view::graphics::Rect;
use tui::buffer::Buffer as Surface;
use tui::text::Text;

pub struct Diff {
    pub added: String,
    pub deleted: String,
    pub hunk: Hunk,
}

impl Component for Diff {
    fn render(&mut self, area: Rect, surface: &mut Surface, ctx: &mut Context) {
        use tui::widgets::{Paragraph, Widget, Wrap};

        let theme = &ctx.editor.theme;
        let deleted_text = Text::styled(&self.deleted, theme.get("diff.minus"));
        let added_text = Text::styled(&self.added, theme.get("diff.plus"));
        let text = if self.hunk.is_pure_insertion() {
            added_text
        } else if self.hunk.is_pure_removal() {
            deleted_text
        } else {
            let mut tmp_text = Text::from("");
            tmp_text.extend(deleted_text);
            tmp_text.extend(added_text);
            tmp_text
        };
        let mut par = Paragraph::new(&text).wrap(Wrap { trim: false });
        if let Some(scroll) = ctx.scroll {
            par = par.scroll((scroll as u16, 0))
        }
        par.render(area, surface);
    }

    fn required_size(&mut self, viewport: (u16, u16)) -> Option<(u16, u16)> {
        let deleted_text = Text::from(self.deleted.as_str());
        let added_text = Text::from(self.added.as_str());

        let (del_width, del_height) = ui::text::required_size(&deleted_text, viewport.0);
        let (add_width, add_height) = ui::text::required_size(&added_text, viewport.0);
        Some((del_width.max(add_width), del_height + add_height))
    }
}
