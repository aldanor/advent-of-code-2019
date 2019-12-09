fn count(slice: &[i64], digit: i64) -> usize {
    slice.iter().filter(|&x| *x == digit).count()
}

struct Image {
    data: Vec<i64>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn from_str(string: &str, width: usize, height: usize) -> Self {
        let data = string
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as _)
            .collect();
        Self {
            data,
            width,
            height,
        }
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = &[i64]> {
        self.data.chunks(self.width * self.height)
    }
}

fn main() {
    let input = include_str!("inputs/day-08.txt");
    let image = Image::from_str(input, 25, 6);

    let objective = |s| (count(s, 0), count(s, 1) * count(s, 2));
    let answer1 = image.iter_layers().map(objective).min().unwrap().1;
    println!("{}", answer1);
}
