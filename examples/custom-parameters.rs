#[macro_use]
extern crate lazy_static;
extern crate punkt_stable;

use std::collections::HashSet;

use punkt_stable::params::*;
use punkt_stable::{SentenceTokenizer, Trainer, TrainingData};

struct MyParams;

/// Counts macro argument repetitions from a token tree
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

/// Creates a hashset with elements
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

impl DefinesInternalPunctuation for MyParams {}
impl DefinesNonPrefixCharacters for MyParams {}

lazy_static! {
    static ref NONWORD_CHARS: HashSet<char> = hashset!['?', '!', ')', '"', ';', '}', ']', '*', ':', '@', '\'', '(', '{', '[', '\u{201c}', '\u{201d}'];
}

impl DefinesNonWordCharacters for MyParams {
    fn get_chars() -> &'static HashSet<char> {
        &NONWORD_CHARS
    }
}
impl DefinesPunctuation for MyParams {}
impl DefinesSentenceEndings for MyParams {}

impl TrainerParameters for MyParams {
  const ABBREV_LOWER_BOUND: f64 = 0.3;
  const ABBREV_UPPER_BOUND: f64 = 8f64;
  const IGNORE_ABBREV_PENALTY: bool = false;
  const COLLOCATION_LOWER_BOUND: f64 = 7.88;
  const SENTENCE_STARTER_LOWER_BOUND: f64 = 35f64;
  const INCLUDE_ALL_COLLOCATIONS: bool = false;
  const INCLUDE_ABBREV_COLLOCATIONS: bool = true;
  const COLLOCATION_FREQUENCY_LOWER_BOUND: f64 = 0.8;
}

// The article in this example has some unicode characters in it that are not
// defined in the default settings. The above custom parameters modify some
// of the parameters for the trainer, and add in the unicode characters present
// in the article, to provide better results.
fn main() {
  println!("\n-- Trained using custom parameters --\n");

  let doc = include_str!("../test/raw/ny-times-article-02.txt");
  let trainer: Trainer<MyParams> = Trainer::new();
  let mut data = TrainingData::new();

  trainer.train(doc, &mut data);

  for s in SentenceTokenizer::<MyParams>::new(doc, &data) {
    println!("{:?}", s);
  }
}
