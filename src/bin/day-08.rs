use std::fmt::{self, Debug};

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

    pub fn render(&self) -> Self {
        let layer_size = self.width * self.height;
        let n_layers = self.data.len() / layer_size;
        let rendered = (0..layer_size)
            .map(|pixel_id| {
                (0..n_layers)
                    .map(|layer_id| self.data[layer_id * layer_size + pixel_id])
                    .skip_while(|&x| x == 2)
                    .next()
                    .unwrap_or(0)
            })
            .collect();
        Self {
            data: rendered,
            width: self.width,
            height: self.height,
        }
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.chunks(self.width).for_each(|row| {
            row.iter().for_each(|&x| {
                write!(f, "{}", if x == 1 { "█" } else { " " }).unwrap();
            });
            write!(f, "\n").unwrap();
        });
        Ok(())
    }
}

fn main() {
    let input = include_str!("inputs/day-08.txt");
    let image = Image::from_str(input, 25, 6);

    let objective = |s| (count(s, 0), count(s, 1) * count(s, 2));
    let answer1 = image.iter_layers().map(objective).min().unwrap().1;
    println!("{}", answer1);

    let answer2 = image.render();
    println!("{:?}", answer2);
}
