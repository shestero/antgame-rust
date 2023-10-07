use crate::ant_state::AntState;
use bit_vec::BitVec;
use itertools::Itertools;

pub enum Color {
    Black,
    White,
}
pub struct Field {
    bv: BitVec,
}

impl Field {
    pub const W: usize = 1024;
    pub const H: usize = 1024;
    const MAX_INDEX: usize = Field::W * Field::H;

    pub fn create() -> Field {
        return Field {
            bv: BitVec::from_elem(Field::W * Field::H, false),
        };
    }

    fn index(x: i16, y: i16) -> usize {
        return (x as usize) + (y as usize) * Field::W;
    }

    fn get_then_invert(&mut self, x: i16, y: i16) -> Color {
        let i: usize = Field::index(x, y);
        let c: bool = self.bv.get(i).unwrap_or(false);
        self.bv.set(i, !c);
        return match c {
            true => Color::Black,
            false => Color::White,
        };
    }

    pub fn play(&mut self) {
        let mut ant: AntState = AntState::start();
        while ant.is_in_field() {
            let color: Color = self.get_then_invert(ant.x, ant.y);
            ant = ant.step(color);
        }
        let blacks: usize = self.bv.iter().filter(|x| *x).count();
        println!("Count of black cells = {blacks}");
    }

    pub fn save(&self, file_name: String) {
        use std::fs::File;
        use std::io::BufWriter;
        use std::path::Path;

        let path = Path::new(&file_name);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, Field::W as u32, Field::H as u32);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::One);
        let mut writer = encoder.write_header().unwrap();

        // An array containing a RGBA sequence. First pixel is red and second pixel is black.
        let vec: Vec<u8> = (0..Field::MAX_INDEX)
            .map(|i| {
                let c: bool = self.bv.get(i).unwrap_or(false);
                if c {
                    0u8
                } else {
                    1u8
                }
            })
            .chunks(8)
            .into_iter()
            .map(|ch| ch.reduce(|acc, bit| acc + acc + bit).unwrap())
            .collect::<Vec<u8>>();
        let data: &[u8] /* : &[u8; Field::max_index] */ = vec.as_ref();

        writer.write_image_data(data).unwrap();
        println!("The field was saved into {file_name}")
    }
}
