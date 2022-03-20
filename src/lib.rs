
pub mod medias;

pub use crate::medias::{AnimeScore, MangaScore, coefficient::Coefficient};



#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{AnimeScore, MangaScore};
    use crate::medias::MediaScore;

    #[test]
    fn anime_score() {
        let mut score = AnimeScore::new();
        let mut values = HashMap::new();
        values.insert("originality".to_string(), 8 as u8);
        values.insert("story".to_string(), 6 as u8);
        values.insert("drawings".to_string(), 5 as u8);
        values.insert("animation".to_string(), 8 as u8);
        values.insert("music".to_string(), 7 as u8);
        values.insert("assessment".to_string(), 6 as u8);
        values.insert("other".to_string(), 7 as u8);  // should not be use in the calcul
        assert_eq!(score.get_coefficients().len(), 6);
        assert_eq!(score.calculate(&values), 40/6);

        score.set_coeff_value("originality".to_string(), 2 as u8);
        score.set_coeff_value("story".to_string(), 4 as u8);
        score.set_coeff_value("drawings".to_string(), 3 as u8);
        score.set_coeff_value("animation".to_string(), 2 as u8);
        score.set_coeff_value("music".to_string(), 1 as u8);
        score.set_coeff_value("assessment".to_string(), 3 as u8);
        let num = 2*8 + 4*6 + 3*5 + 2*8 + 1*7 + 3*6;
        let det = 15;
        assert_eq!(score.calculate(&values), num / det);
    }

    #[test]
    fn manga_score() {
        let mut score = MangaScore::new();
        let mut values = HashMap::new();
        values.insert("originality".to_string(), 8 as u8);
        values.insert("story".to_string(), 6 as u8);
        values.insert("drawings".to_string(), 5 as u8);
        values.insert("assessment".to_string(), 6 as u8);
        values.insert("other".to_string(), 7 as u8);  // should not be use in the calcul
        assert_eq!(score.get_coefficients().len(), 4);
        assert_eq!(score.calculate(&values), 25/4);

        score.set_coeff_value("originality".to_string(), 2 as u8);
        score.set_coeff_value("story".to_string(), 4 as u8);
        score.set_coeff_value("drawings".to_string(), 3 as u8);
        score.set_coeff_value("assessment".to_string(), 3 as u8);
        let num = 2*8 + 4*6 + 3*5 + 3*6;
        let det = 12;
        assert_eq!(score.calculate(&values), num / det);
    }
}
