use regex::Regex;

pub struct GulpeaseIndex;

impl GulpeaseIndex {
    pub fn new() -> Self {
        GulpeaseIndex
    }

    pub fn calculate(&self, text: &str) -> f64 {
        let sentence_count = self.count_sentences(text);
        let word_count = self.count_words(text);
        let letter_count = self.count_letters(text);

        if word_count == 0 || sentence_count == 0 {
            return 0.0;
        }

        // Gulpease Index formula
        let gulpease = 89.0 - (10.0 * letter_count as f64 / word_count as f64) + (300.0 * sentence_count as f64 / word_count as f64);

        gulpease.clamp(0.0, 100.0)
    }

    fn count_sentences(&self, text: &str) -> usize {
        let re = Regex::new(r"[.!?]").unwrap();
        re.find_iter(text).count()
    }

    fn count_words(&self, text: &str) -> usize {
        let re = Regex::new(r"\b\w+\b").unwrap();
        re.find_iter(text).count()
    }

    fn count_letters(&self, text: &str) -> usize {
        let re = Regex::new(r"[a-zA-Z]").unwrap();
        re.find_iter(text).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_italian() {
        let gi = GulpeaseIndex::new();
        let text = "Questo è un testo di prova. È progettato per verificare l'indice di Gulpease.";
        let score = gi.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_german() {
        let gi = GulpeaseIndex::new();
        let text = "Dies ist ein Testtext. Es soll den Gulpease-Index überprüfen.";
        let score = gi.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_portuguese() {
        let gi = GulpeaseIndex::new();
        let text = "Este é um texto de teste. Ele é projetado para verificar o índice de Gulpease.";
        let score = gi.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_edge_cases() {
        let gi = GulpeaseIndex::new();
        let text = "";
        let score = gi.calculate(text);
        assert_eq!(score, 0.0);

        let text = "A.";
        let score = gi.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }
}
