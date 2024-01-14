//use hashbrown;

#[macro_export]
macro_rules! hashset {
    ($($x:expr),+ $(,)?) => {
        hashbrown::HashSet::from([$($x),+])
    };

    }
