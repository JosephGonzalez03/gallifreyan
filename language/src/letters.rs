use crate::letter_parts::*;
use crate::math_util::{draw_arc, law_of_sines_angle, Degree, Drawing, Polar};
use std::iter::zip;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct GallifreyanCharacter {
    base: Base,
    modifier: Option<Modifier>,
    size: f32,
    position: Polar,
}

impl GallifreyanCharacter {
    pub fn draw_base(&self) -> Drawing {
        self.base.to_drawing(self.position, &self.size).to_owned()
    }

    pub fn draw_modifier(&self) -> Option<Vec<Drawing>> {
        if let Some(modifier) = &self.modifier {
            let base_height = match self.base {
                Base::Crescent => CRESCENT_HEIGHT,
                Base::Full => FULL_HEIGHT,
                _ => DEFAULT_BASE_HEIGHT,
            };

            Some(
                modifier
                    .to_drawings(&self.position, &self.size, &base_height)
                    .to_owned(),
            )
        } else {
            None
        }
    }

    pub fn has_edge(&self) -> bool {
        matches!(&self.base, Base::Crescent | Base::Quarter)
    }

    fn starting_angle(&self) -> Option<Degree> {
        match self.base {
            Base::Crescent => Some(
                self.position.angle()
                    - law_of_sines_angle(&self.position.radius(), &self.size, CRESCENT_BASE_OFFSET),
            ),
            Base::Quarter => Some(
                self.position.angle()
                    - law_of_sines_angle(&self.position.radius(), &self.size, QUARTER_BASE_OFFSET),
            ),
            _ => None,
        }
    }

    fn ending_angle(&self) -> Option<Degree> {
        match self.base {
            Base::Crescent => Some(
                self.position.angle()
                    + law_of_sines_angle(&self.position.radius(), &self.size, CRESCENT_BASE_OFFSET),
            ),
            Base::Quarter => Some(
                self.position.angle()
                    + law_of_sines_angle(&self.position.radius(), &self.size, QUARTER_BASE_OFFSET),
            ),
            _ => None,
        }
    }
}

struct GallifreyanCharacterCollection(pub Vec<GallifreyanCharacter>);

impl FromIterator<GallifreyanCharacter> for GallifreyanCharacterCollection {
    fn from_iter<T: IntoIterator<Item = GallifreyanCharacter>>(iter: T) -> Self {
        let mut gallifreyan_characters = GallifreyanCharacterCollection::new();

        for i in iter {
            gallifreyan_characters.0.push(i)
        }

        gallifreyan_characters
    }
}

impl GallifreyanCharacterCollection {
    fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGallifreyanLetterError;

/// An enumeration for the letters in the Gallifreyan alphabet.
pub enum GallifreyanLetter {
    E,
    A,
    I,
    O,
    U,
    B,
    CH,
    D,
    G,
    H,
    F,
    J,
    PH,
    K,
    L,
    C,
    N,
    P,
    M,
    T,
    WH,
    SH,
    R,
    V,
    W,
    S,
    TH,
    GH,
    Y,
    Z,
    Q,
    QU,
    X,
    NG,
}

impl FromStr for GallifreyanLetter {
    type Err = ParseGallifreyanLetterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Result::Ok(GallifreyanLetter::A),
            "E" => Result::Ok(GallifreyanLetter::E),
            "I" => Result::Ok(GallifreyanLetter::I),
            "O" => Result::Ok(GallifreyanLetter::O),
            "U" => Result::Ok(GallifreyanLetter::U),
            "B" => Result::Ok(GallifreyanLetter::B),
            "CH" => Result::Ok(GallifreyanLetter::CH),
            "D" => Result::Ok(GallifreyanLetter::D),
            "G" => Result::Ok(GallifreyanLetter::G),
            "H" => Result::Ok(GallifreyanLetter::H),
            "F" => Result::Ok(GallifreyanLetter::F),
            "J" => Result::Ok(GallifreyanLetter::J),
            "PH" => Result::Ok(GallifreyanLetter::PH),
            "K" => Result::Ok(GallifreyanLetter::K),
            "L" => Result::Ok(GallifreyanLetter::L),
            "C" => Result::Ok(GallifreyanLetter::C),
            "N" => Result::Ok(GallifreyanLetter::N),
            "P" => Result::Ok(GallifreyanLetter::P),
            "M" => Result::Ok(GallifreyanLetter::M),
            "T" => Result::Ok(GallifreyanLetter::T),
            "WH" => Result::Ok(GallifreyanLetter::WH),
            "SH" => Result::Ok(GallifreyanLetter::SH),
            "R" => Result::Ok(GallifreyanLetter::R),
            "V" => Result::Ok(GallifreyanLetter::V),
            "W" => Result::Ok(GallifreyanLetter::W),
            "S" => Result::Ok(GallifreyanLetter::S),
            "TH" => Result::Ok(GallifreyanLetter::TH),
            "GH" => Result::Ok(GallifreyanLetter::GH),
            "Y" => Result::Ok(GallifreyanLetter::Y),
            "Z" => Result::Ok(GallifreyanLetter::Z),
            "Q" => Result::Ok(GallifreyanLetter::Q),
            "QU" => Result::Ok(GallifreyanLetter::QU),
            "X" => Result::Ok(GallifreyanLetter::X),
            "NG" => Result::Ok(GallifreyanLetter::NG),
            _ => Result::Err(ParseGallifreyanLetterError),
        }
    }
}

impl GallifreyanLetter {
    pub fn to_gallifreyan_character(&self, position: Polar) -> GallifreyanCharacter {
        let size = 2.0;
        match self {
            GallifreyanLetter::A => todo!(""),
            GallifreyanLetter::E => todo!(""),
            GallifreyanLetter::I => todo!(""),
            GallifreyanLetter::O => todo!(""),
            GallifreyanLetter::U => todo!(""),
            GallifreyanLetter::B => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: None,
                size,
                position,
            },
            GallifreyanLetter::CH => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Dot2),
                size,
                position,
            },
            GallifreyanLetter::D => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Dot3),
                size,
                position,
            },
            GallifreyanLetter::G => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Line1),
                size,
                position,
            },
            GallifreyanLetter::H => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Line2),
                size,
                position,
            },
            GallifreyanLetter::F => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Line3),
                size,
                position,
            },
            GallifreyanLetter::J => GallifreyanCharacter {
                base: Base::Full,
                modifier: None,
                size,
                position,
            },
            GallifreyanLetter::PH => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot1),
                size,
                position,
            },
            GallifreyanLetter::K => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot2),
                size,
                position,
            },
            GallifreyanLetter::L => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot3),
                size,
                position,
            },
            GallifreyanLetter::C => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot4),
                size,
                position,
            },
            GallifreyanLetter::N => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Line1),
                size,
                position,
            },
            GallifreyanLetter::P => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Line2),
                size,
                position,
            },
            GallifreyanLetter::M => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Line3),
                size,
                position,
            },
            GallifreyanLetter::T => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: None,
                size,
                position,
            },
            GallifreyanLetter::WH => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Dot1),
                size,
                position,
            },
            GallifreyanLetter::SH => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Dot2),
                size,
                position,
            },
            GallifreyanLetter::R => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Dot3),
                size,
                position,
            },
            GallifreyanLetter::V => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Line1),
                size,
                position,
            },
            GallifreyanLetter::W => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Line2),
                size,
                position,
            },
            GallifreyanLetter::S => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Line3),
                size,
                position,
            },
            GallifreyanLetter::TH => GallifreyanCharacter {
                base: Base::New,
                modifier: None,
                size,
                position,
            },
            GallifreyanLetter::GH => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot1),
                size,
                position,
            },
            GallifreyanLetter::Y => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot2),
                size,
                position,
            },
            GallifreyanLetter::Z => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot3),
                size,
                position,
            },
            GallifreyanLetter::Q => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot4),
                size,
                position,
            },
            GallifreyanLetter::QU => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Line1),
                size,
                position,
            },
            GallifreyanLetter::X => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Line2),
                size,
                position,
            },
            GallifreyanLetter::NG => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Line3),
                size,
                position,
            },
        }
    }

    fn is_vowel(&self) -> bool {
        matches!(
            self,
            GallifreyanLetter::A
                | GallifreyanLetter::E
                | GallifreyanLetter::I
                | GallifreyanLetter::O
                | GallifreyanLetter::U
        )
    }
}

pub struct GallifreyanWord(Vec<GallifreyanLetter>);

impl FromIterator<GallifreyanLetter> for GallifreyanWord {
    fn from_iter<T: IntoIterator<Item = GallifreyanLetter>>(iter: T) -> Self {
        let mut word = GallifreyanWord::new();

        for i in iter {
            word.push(i);
        }

        word
    }
}

#[derive(Debug)]
struct UsizeParsingError;

impl GallifreyanWord {
    pub fn new() -> GallifreyanWord {
        GallifreyanWord(Vec::new())
    }

    fn push(&mut self, letter: GallifreyanLetter) {
        self.0.push(letter);
    }

    pub fn from(word: &str) -> Result<GallifreyanWord, ParseGallifreyanLetterError> {
        let mut grouped_letters = Vec::<String>::new();
        let mut char_iter = word.chars().into_iter().peekable();

        while let Some(current_letter) = char_iter.next() {
            let entry = match current_letter.to_ascii_uppercase() {
                'C' | 'P' | 'W' | 'S' | 'T' | 'G' => match char_iter.next_if_eq(&'H') {
                    Some(next_letter) => current_letter.to_string() + &next_letter.to_string(),
                    None => current_letter.to_string(),
                },
                'Q' => match char_iter.next_if_eq(&'U') {
                    Some(next_letter) => current_letter.to_string() + &next_letter.to_string(),
                    None => current_letter.to_string(),
                },
                'N' => match char_iter.next_if_eq(&'G') {
                    Some(next_letter) => current_letter.to_string() + &next_letter.to_string(),
                    None => current_letter.to_string(),
                },
                _ => current_letter.to_string(),
            };

            grouped_letters.push(entry);
        }

        grouped_letters.iter().for_each(|gl| println!("{}", gl));
        grouped_letters
            .iter()
            .map(|letter| letter.parse::<GallifreyanLetter>())
            .collect::<Result<GallifreyanWord, ParseGallifreyanLetterError>>()
    }

    fn parse_usize(num: &usize) -> Result<f32, UsizeParsingError> {
        if num > &(f32::MAX as usize) {
            Err(UsizeParsingError)
        } else {
            Ok(*num as f32)
        }
    }

    pub fn to_drawings(&self, word: f32) -> Vec<Drawing> {
        let mut current_position = -1.0;
        let num_of_consonants: usize = self
            .0
            .iter()
            .map(|gallifreyan_letter| !gallifreyan_letter.is_vowel())
            .count();
        let position_step: f32 = 360.0 / Self::parse_usize(&num_of_consonants).unwrap();

        let positions = self
            .0
            .iter()
            .map(|gallifreyan_letter| match gallifreyan_letter.is_vowel() {
                true => current_position,
                false => {
                    current_position += 1.0;
                    current_position
                }
            })
            .map(|position| (position * position_step) - 90.0);

        let gallifreyan_characters = zip(&self.0, positions)
            .into_iter()
            .map(|(gallifreyan_letter, position)| {
                gallifreyan_letter.to_gallifreyan_character(Polar::new(word, Degree(position)))
            })
            .collect::<GallifreyanCharacterCollection>()
            .0;

        let mut character_drawings = gallifreyan_characters
            .iter()
            .flat_map(|gallifreyan_character| {
                let mut drawings = DrawingCollection::new();

                drawings.0.push(gallifreyan_character.draw_base());

                if let Some(modifier_drawings) = &mut gallifreyan_character.draw_modifier() {
                    drawings.0.append(modifier_drawings);
                }

                drawings.0
            })
            .collect::<DrawingCollection>()
            .0;

        let mut characters_with_edges = gallifreyan_characters
            .into_iter()
            .filter(|gallifreyan_character| gallifreyan_character.has_edge())
            .collect::<GallifreyanCharacterCollection>()
            .0;

        characters_with_edges.extend_from_within(..1);
        characters_with_edges
            .as_slice()
            .windows(2)
            .map(|letters| {
                let edge1 = letters[0]
                    .ending_angle()
                    .expect("The Gallifreyan character should have an edge.");
                let edge2 = letters[1]
                    .starting_angle()
                    .expect("The Gallifreyan character should have an edge.");

                draw_arc(letters[0].position.radius(), (edge1, edge2))
            })
            .for_each(|drawings| character_drawings.push(drawings));

        character_drawings
    }
}
