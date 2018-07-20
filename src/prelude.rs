// Copyright 2016 rust-punkt developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! count_repetitions {
    () => (
        0usize
    );
    ($head:tt) => (
        1usize
    );
    ($head:tt $($tail:tt)+) => (
        1usize + count_repetitions!($($tail)+)
    )
}

macro_rules! hashset {
    ($($element:expr),*) => [{
        use std::collections::HashSet;
        let mut set = HashSet::with_capacity(count_repetitions!($($element)*));
        $(
            set.insert($element);
         )*
        set
    }]
}

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => [{
        use std::collections::HashMap;
        let mut set = HashMap::with_capacity(count_repetitions!($($key)*));
        $(
            set.insert($key, $value);
         )*
        set
    }]
}

use std::collections::{HashMap, HashSet};

/// The set of characters that constitute a sentence ending.
lazy_static! {
  static ref SENTENCE_ENDINGS: HashSet<char> = { hashset!['.', '?', '!'] };
}

/// Defines a set of punctuation that can end a sentence.
pub trait DefinesSentenceEndings {
  /// Checks if a character is a sentence ending.
  #[inline]
  fn is_sentence_ending(c: &char) -> bool {
    SENTENCE_ENDINGS.contains(c)
  }
}

/// The set of legal punctuation characters that can occur within a word.
lazy_static! {
  static ref INTERNAL_PUNCTUATION: HashSet<char> = hashset![',', ':', ';', '\u{2014}'];
}

/// Defines a set of punctuation that can occur within a word.
pub trait DefinesInternalPunctuation {
  /// Checks if a character is a legal punctuation character that can occur
  /// within a word.
  #[inline]
  fn is_internal_punctuation(c: &char) -> bool {
    INTERNAL_PUNCTUATION.contains(c)
  }
}

/// The set of characters that can not occur inside of a word.
lazy_static! {
  static ref NONWORD_CHARS: HashSet<char> = hashset![
    '?', '!', ')', '"', ';', '}', ']', '*', ':', '@', '\'', '(', '{', '['
  ];
}

/// Defines a set of characters that can not occur inside of a word.
pub trait DefinesNonWordCharacters {
  /// Checks if a character is one that can not occur inside of a word.
  #[inline]
  fn is_nonword_char(c: &char) -> bool {
    NONWORD_CHARS.contains(c)
  }
}

/// The set of legal punctuation marks.
lazy_static! {
  static ref PUNCTUATION: HashSet<char> = hashset![';', ':', ',', '.', '!', '?'];
}

/// Defines punctuation that can occur within a sentence.
pub trait DefinesPunctuation {
  /// Checks if a characters is a legal punctuation mark.
  #[inline]
  fn is_punctuation(c: &char) -> bool {
    PUNCTUATION.contains(c)
  }
}

/// The set of characters that can not start a word.
lazy_static! {
  static ref NONPREFIX_CHARS: HashSet<char> = hashset![
    '(', '"', '`', '{', '[', ':', ';', '&', '#', '*', '@', ')', '}', ']', '-', ','
  ];
}

/// Defines a set of a characters that can not start a word.
pub trait DefinesNonPrefixCharacters {
  /// Checks if a character can start a word.
  #[inline]
  fn is_nonprefix_char(c: &char) -> bool {
    NONPREFIX_CHARS.contains(c)
  }
}

/// Configurable parameters for a trainer.
pub trait TrainerParameters: DefinesSentenceEndings + DefinesInternalPunctuation {
  /// Lower bound score for a token to be considered an abbreviation.
  const ABBREV_LOWER_BOUND: f64 = 0.3;

  /// Upper bound score for a token to be considered an abbreviation.
  const ABBREV_UPPER_BOUND: f64 = 5f64;

  /// Disables the abbreviation penalty which exponentially penalizes occurances
  /// of words without a trailing period.
  const IGNORE_ABBREV_PENALTY: bool = false;

  /// Lower bound score for two tokens to be considered a collocation
  const COLLOCATION_LOWER_BOUND: f64 = 7.88;

  /// Lower bound score for a token to be considered a sentence starter.
  const SENTENCE_STARTER_LOWER_BOUND: f64 = 30f64;

  /// Include all pairs where the first token ends with a period.
  const INCLUDE_ALL_COLLOCATIONS: bool = false;

  /// Include all pairs where the first is an abbreviation. Overridden by
  /// `include_all_collocations`.
  const INCLUDE_ABBREV_COLLOCATIONS: bool = false;

  /// Minimum number of times a bigram appears in order to be considered a
  /// collocation.
  const COLLOCATION_FREQUENCY_LOWER_BOUND: f64 = 1f64;
}

/// Standard settings for all tokenizers, and trainers.
pub struct Standard;

impl DefinesInternalPunctuation for Standard {}
impl DefinesNonPrefixCharacters for Standard {}
impl DefinesNonWordCharacters for Standard {}
impl DefinesPunctuation for Standard {}
impl DefinesSentenceEndings for Standard {}
impl TrainerParameters for Standard {}

pub type OrthographicContext = u8;

#[derive(PartialEq, Eq)]
pub enum OrthographyPosition {
  Initial,
  Internal,
  Unknown,
}

impl OrthographyPosition {
  pub fn as_byte(&self) -> u8 {
    match *self {
      OrthographyPosition::Initial => 0b01000000,
      OrthographyPosition::Internal => 0b00100000,
      OrthographyPosition::Unknown => 0b01100000,
    }
  }
}

pub const BEG_UC: OrthographicContext = 0b00000010;
pub const MID_UC: OrthographicContext = 0b00000100;
pub const UNK_UC: OrthographicContext = 0b00001000;
pub const BEG_LC: OrthographicContext = 0b00010000;
pub const MID_LC: OrthographicContext = 0b00100000;
pub const UNK_LC: OrthographicContext = 0b01000000;
pub const ORT_UC: OrthographicContext = BEG_UC | MID_UC | UNK_UC;
pub const ORT_LC: OrthographicContext = BEG_LC | MID_LC | UNK_LC;

/// Map relating a combination of LetterCase and OrthographyPosition
/// to an OrthographicConstant describing orthographic attributes about the
/// token. The chars (in ASCII) map to the result of ORing the byte
/// representation of an OrthographyPosition and LetterCase together.
lazy_static! {
    pub static ref ORTHO_MAP: HashMap<u8, OrthographicContext> = hashmap! {
      b'B' => BEG_UC, // 66
      b'"' => MID_UC, // 34
      b'b' => UNK_UC, // 98
      b'A' => BEG_LC, // 65
      b'!' => MID_LC, // 33
      b'a' => UNK_LC  // 97
    };
}

pub enum LetterCase {
  Upper,
  Lower,
  Unknown,
}

impl LetterCase {
  #[inline(always)]
  pub fn as_byte(&self) -> u8 {
    match *self {
      LetterCase::Upper => 0b00000010,
      LetterCase::Lower => 0b00000001,
      LetterCase::Unknown => 0b00000011,
    }
  }
}
