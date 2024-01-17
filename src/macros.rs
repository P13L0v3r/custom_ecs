#[macro_export]
macro_rules! hashset {
    [$($x:expr),+ $(,)?] => {
        hashbrown::HashSet::from([$($x),+])
    };
}

#[macro_export]
macro_rules! component_set {
    ($($x:ty),+ $(,)?) => {
        hashbrown::HashSet::from([$(<$x>::hash()),+])
    };
}

#[macro_export]
macro_rules! component_identifier {
    ($x:ty) => {
        println!("{:?}", <$x>::hash())
    };
}

#[macro_export]
macro_rules! component_filter {
    (
        ($($get:ty),+ $(,)?)
    ) => {
        $crate::table::NodeFilter{
            get: hashbrown::HashSet::from([$(<$get>::hash()),+]),
            ..Default::default()
        }
    };

    (
        ($($get:ty),+ $(,)?),
        ($($with:ty),+ $(,)?)
    ) => {
        $crate::table::NodeFilter{
            get: hashbrown::HashSet::from([$(<$get>::hash()),+]),
            with: hashbrown::HashSet::from([$(<$with>::hash()),+]),
            ..Default::default()
        }
    };

    (
        ($($get:ty),+ $(,)?),
        (),
        ($($without:ty),+ $(,)?)
    ) => {
        $crate::table::NodeFilter{
            get: hashbrown::HashSet::from([$(<$get>::hash()),+]),
            without: hashbrown::HashSet::from([$(<$without>::hash()),+]),
            ..Default::default()
        }
    };

    (
        ($($get:ty),+ $(,)?),
        ($($with:ty),+ $(,)?),
        ($($without:ty),+ $(,)?)
    ) => {
        $crate::table::NodeFilter{
            get: hashbrown::HashSet::from([$(<$get>::hash()),+]),
            with: hashbrown::HashSet::from([$(<$with>::hash()),+]),
            without: hashbrown::HashSet::from([$(<$without>::hash()),+]),
        }
    };
}