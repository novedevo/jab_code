#[derive(Clone, Debug)]
pub struct JABSymbol<'a> {
    payload: &'a str,
    palette_size: u8,
    x: usize,
    y: usize,
    ecc: ErrorCorrectionLevel,
    data: Vec<Vec<char>>,
}
impl Default for JABSymbol<'_> {
    fn default() -> Self {
        Self {
            payload: "testing",
            palette_size: 3,
            x: 1,
            y: 1,
            ecc: ErrorCorrectionLevel::Zero,
            data: vec![vec![' '; 21]; 21],
        }
    }
}
impl std::fmt::Display for JABSymbol<'_> {
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
impl<'a> JABSymbol<'a> {
    pub fn new(data: &'a str) -> Self {
        let mut ret = Self {
            payload: data,
            ..Default::default()
        };
        ret.add_finder_pattern();
        ret
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
    #[rustfmt::skip]
    fn add_finder_pattern(&mut self) {
        let upper_key = [
            "KKK  ",
            "K    ",
            "K K K",
            "    K",
            "  KKK",
        ];
        let ul_colour = [
            "CC ",
            "C C",
            " CC",
        ];
        let ur_colour = [
            "YY ",
            "Y Y",
            " YY",
        ];
        let lower_key = [
            " KK",
            "K K",
            "KK ",
        ];
        let ll_colour = [
            "  CCC",
            "    C",
            "C C C",
            "C    ",
            "CCC  ",
        ];
        let lr_colour = [
            "  YYY",
            "    Y",
            "Y Y Y",
            "Y    ",
            "YYY  ",
        ];

        self.overlay_pattern(1,1, &upper_key);
        self.overlay_pattern(15,1, &upper_key);
        self.overlay_pattern(2,16, &lower_key);
        self.overlay_pattern(16,16, &lower_key);
        self.overlay_pattern(2,2, &ul_colour);
        self.overlay_pattern(16, 2, &ur_colour);
        self.overlay_pattern(1,15, &ll_colour);
        self.overlay_pattern(15,15, &lr_colour);
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
