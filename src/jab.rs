#[derive(Clone, Debug)]
pub struct JABSymbol<'a, const L: usize> {
    payload: &'a str,
    palette_size: u8,
    x: usize,
    y: usize,
    ecc: ErrorCorrectionLevel,
    data: [[char; L]; L],
}
impl<const L: usize> Default for JABSymbol<'_, L> {
    fn default() -> Self {
        Self {
            payload: "testing",
            palette_size: 7,
            x: 1,
            y: 1,
            ecc: ErrorCorrectionLevel::Zero,
            data: [[' '; L]; L],
        }
    }
}
impl<const L: usize> std::fmt::Display for JABSymbol<'_, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for module in row {
                write!(f, "{} ", module)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}
impl<'a, const L: usize> JABSymbol<'a, L> {
    pub fn new(data: &'a str) -> Self {
        let mut ret = Self {
            payload: data,
            ..Default::default()
        };
        ret.add_finder_pattern();
        ret.add_palette();
        ret
    }

    fn metadata(&self) -> Vec<bool> {
        let mut ret = vec![];
        ret.extend_from_slice(&[false, true, false]); //N_c

        ret.push(false); //SS
        ret.extend_from_slice(&[false, false]); //VF
        ret.extend_from_slice(&[false, false, false]); //MSK
        ret.push(false); //SF

        ret.extend_from_slice(&[false, false]); //V
        ret.extend_from_slice(&[false; 10]); //E

        //no bits needed for S since we don't support secondary symbols yet

        ret
    }

    fn add_palette(&mut self) {
        let palette = "KBGCRMYW";
        let locations = [
            (1, 4),
            (2, 4),
            (1, 5),
            (2, 5),
            (4, 2),
            (5, 2),
            (4, 1),
            (5, 1),
        ];

        for (&(x, y), colour) in locations.iter().zip(palette.chars()) {
            self.data[y][x] = colour;
            self.data[L - 7 + y][L - 1 - x] = colour;
        }
    }

    fn overlay_pattern(&mut self, x_offset: usize, y_offset: usize, pattern: &[&str]) {
        for (y, row) in pattern.iter().enumerate() {
            for (x, char) in row.chars().enumerate() {
                if char == ' ' {
                    continue;
                } else {
                    self.data[y + y_offset][x + x_offset] = char;
                }
            }
        }
    }
    fn add_finder_pattern(&mut self) {
        #[rustfmt::skip]
        let ul = [
            "BBB  ",
            "BYY  ",
            "BYBYB",
            "  YYB",
            "  BBB",
        ];
        #[rustfmt::skip]
        let ur = [
            "GGG  ",
            "GMM  ",
            "GMGMG",
            "  MMG",
            "  GGG",
        ];
        #[rustfmt::skip]
        let ll = [
            "  YYY",
            "  BBY",
            "YBYBY",
            "YBB  ",
            "YYY  ",
        ];
        #[rustfmt::skip]
        let lr = [
            "  MMM",
            "  GGM",
            "MGMGM",
            "MGG  ",
            "MMM  ",
        ];

        self.overlay_pattern(1, 1, &ul);
        self.overlay_pattern(L - 6, 1, &ur);
        self.overlay_pattern(1, L - 6, &ll);
        self.overlay_pattern(L - 6, L - 6, &lr);
    }
}

#[derive(Clone, Debug)]
pub enum ErrorCorrectionLevel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}
impl Default for ErrorCorrectionLevel {
    fn default() -> Self {
        Self::Zero
    }
}
