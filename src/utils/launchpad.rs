use launchy::launchpad_mini_mk3::RgbColor;
use launchy::mini_mk3::Output;
use launchy::{MidiError, OutputDevice};

use crate::models::vector2i::Vector2i;

pub type RgbUpdate = (Vector2i, RgbColor);

pub fn light_rbg(out: &mut Output, updates: &[RgbUpdate]) -> Result<(), MidiError> {
    let mut bytes = Vec::with_capacity(8 + 12 * updates.len());

    bytes.extend(&[240, 0, 32, 41, 2, 13, 3]);
    for update in updates {
        let (Vector2i { x, y }, color) = update;
        assert!(color.is_valid());
        bytes.extend(&[
            3,
            ((y + 1) * 10) + (x + 1),
            color.red(),
            color.green(),
            color.blue(),
        ]);
    }
    bytes.push(247);

    out.send(&bytes)
}
