# animscore-common-rust

A crate for common features of AnimScore project, a project to calcul a score for anime and manga.


## getting started

### Cargo.toml

In the `Cargo.toml` file, add the dependencie:

```toml
...

[dependencies]
animscore_common = { git = "https://github.com/LostPy/animscore-common-rust" }
```

### Default Coefficient

#### For AnimeScore

Name|HashMap key|Description|Default value
:--:|:---------:|-----------|:-----------:
Originality|originality||1
Story|story||1
Drawings|drawings||1
Animation|animation||1
Music|music||1
Global Assessment|assessment||1


#### For MangaScore

Name|HashMap key|Description|Default value
:--:|:---------:|-----------|:-----------:
Originality|originality||1
Story|story||1
Drawings|drawings||1
Global Assessment|assessment||1

### Calcul an anime score

```rust
use std::collections::HashMap;
use animscore_common::{AnimeScore};


let anim_score = AnimeScore::new();
anim_score.set_coeff_value("originality".to_string(), 3);
anim_score.set_coeff_value("assessment".to_string(), 3);
anim_score.set_coeff_value("story".to_string(), 2);

let mut my_notes = HashMap::new();
my_notes.insert("originality", 6);
my_notes.insert("story", 8);
my_notes.insert("drawings", 5);
my_notes.insert("animation", 7);
my_notes.insert("music", 6);
my_notes.insert("assessment", 7);

println!("score: {}", anim_score.calculate(&my_notes));
```