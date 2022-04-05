#[derive(Clone, Debug)]
pub struct JABSymbol<'a> {
    payload: &'a str,
    palette_size: u8,
    x: usize,
    y: usize,
    ecc: ErrorCorrectionLevel,
    data: Vec<Vec<u8>>,
}
impl Default for JABSymbol<'_> {
    fn default() -> Self {
        Self {
            payload: "testing",
            palette_size: 3,
            x: 1,
            y: 1,
            ecc: ErrorCorrectionLevel::Zero,
            data: vec![vec![4; 21]; 21],
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
    fn add_finder_pattern(&mut self) {
        self.add_finder_pattern_ul();
        self.add_finder_pattern_ur();
        self.add_finder_pattern_ll();
        self.add_finder_pattern_lr();
    }

    fn add_finder_pattern_ul(&mut self) {
        self.data[1][1] = 3;
        self.data[1][2] = 3;
        self.data[1][3] = 3;
        self.data[2][1] = 3;
        self.data[3][1] = 3;

        self.data[2][2] = 0;
        self.data[2][3] = 0;
        self.data[3][2] = 0;

        self.data[3][3] = 3;

        self.data[4][4] = 0;
        self.data[4][3] = 0;
        self.data[3][4] = 0;

        self.data[5][5] = 3;
        self.data[5][4] = 3;
        self.data[5][3] = 3;
        self.data[4][5] = 3;
        self.data[3][5] = 3;
    }
    fn add_finder_pattern_ur(&mut self) {
        self.data[1][15] = 3;
        self.data[1][16] = 3;
        self.data[1][17] = 3;
        self.data[2][15] = 3;
        self.data[3][15] = 3;

        self.data[2][16] = 2;
        self.data[2][17] = 2;
        self.data[3][16] = 2;

        self.data[3][17] = 3;

        self.data[4][18] = 2;
        self.data[4][17] = 2;
        self.data[3][18] = 2;

        self.data[5][19] = 3;
        self.data[5][18] = 3;
        self.data[5][17] = 3;
        self.data[4][19] = 3;
        self.data[3][19] = 3;
    }
    fn add_finder_pattern_ll(&mut self) {
        self.data[15][1] = 0;
        self.data[15][2] = 0;
        self.data[15][3] = 0;
        self.data[16][1] = 0;
        self.data[17][1] = 0;

        self.data[16][2] = 3;
        self.data[16][3] = 3;
        self.data[17][2] = 3;

        self.data[17][3] = 0;

        self.data[18][4] = 3;
        self.data[18][3] = 3;
        self.data[17][4] = 3;

        self.data[19][5] = 0;
        self.data[19][4] = 0;
        self.data[19][3] = 0;
        self.data[18][5] = 0;
        self.data[17][5] = 0;
    }
    fn add_finder_pattern_lr(&mut self) {
        self.data[15][15] = 2;
        self.data[15][16] = 2;
        self.data[15][17] = 2;
        self.data[16][15] = 2;
        self.data[17][15] = 2;

        self.data[16][16] = 3;
        self.data[16][17] = 3;
        self.data[17][16] = 3;

        self.data[17][17] = 2;

        self.data[18][18] = 3;
        self.data[18][17] = 3;
        self.data[17][18] = 3;

        self.data[19][19] = 2;
        self.data[19][18] = 2;
        self.data[19][17] = 2;
        self.data[18][19] = 2;
        self.data[17][19] = 2;
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
