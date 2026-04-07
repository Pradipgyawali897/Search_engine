use crate::TF;
use crate::tokenizer::tokenize;

pub fn build_term_frequency(content: &str) -> TF {
    let mut tf = TF::new();

    for token in tokenize(content) {
        *tf.entry(token).or_insert(0) += 1;
    }

    tf
}
