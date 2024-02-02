use launchy::launchpad_mini_mk3::RgbColor;

// WARNING: bad code
pub fn gen_rainbow_colors_launchpad() -> Vec<RgbColor> {
    let mut all = Vec::new();

    let mut color = (127, 0, 0);
    let mut stage = 0u8;

    loop {
        match stage {
            0 => color.1 += 1,
            1 => color.0 -= 1,
            2 => color.2 += 1,
            3 => color.1 -= 1,
            4 => color.0 += 1,
            5 => color.2 -= 1,
            _ => unreachable!(),
        }

        all.push(RgbColor::new(color.0, color.1, color.2));

        if color == (127, 0, 0) {
            return all;
        }

        if color == (127, 127, 0) {
            stage = 1;
        }

        if color == (0, 127, 0) {
            stage = 2;
        }

        if color == (0, 127, 127) {
            stage = 3;
        }

        if color == (0, 0, 127) {
            stage = 4;
        }

        if color == (127, 0, 127) {
            stage = 5;
        }
    }
}
