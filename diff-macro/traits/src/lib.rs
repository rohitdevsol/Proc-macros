pub trait Diff {
    fn diff(&self, other: &Self) -> Vec<String>;
}

// ["age changed from 16 to 18" , "name changed from subh to beautiful bri"]
