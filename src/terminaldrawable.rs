use crate::asciicontext::AsciiContext;

pub trait TerminalDrawble {
    fn draw(&self, ctx: &mut AsciiContext);
}
