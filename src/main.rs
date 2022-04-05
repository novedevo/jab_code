fn main() {
    let encode_string = "testing";

    let jab = JABCode {
        data: encode_string.to_string(),
        depth: ColourCount::Four,
        x: 1,
        y: 1,
        ecc: ErrorCorrectionLevel::Zero,
    };
}

struct JABCode {
    data: String,
    depth: ColourCount,
    x: usize,
    y: usize,
    ecc: ErrorCorrectionLevel,
}

enum ColourCount {
    Four,
    Eight,
}

enum ErrorCorrectionLevel {
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

enum FourColours {
    C,
    M,
    Y,
    K,
}
