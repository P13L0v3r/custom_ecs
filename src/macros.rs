#[macro_export]
macro_rules! hashset {
    [$($x:expr),+ $(,)?] => {
        hashbrown::HashSet::from([$($x),+])
    };
}

#[macro_export]
macro_rules! component_set {
    ($($x:ty),+ $(,)?) => {
        hashbrown::HashSet::from([$(<$x as Component>::hash()),+])
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

#[macro_export]
macro_rules! world_query {
    ($world:ident, ($($get:ty),+ $(,)?)) => {

        $world.component_node_bundles(Some(hashbrown::HashSet::from([$(<$get as Component>::hash()),+])), None, None).iter().map(|bundle| ($($world.unpack::<$get>(bundle).unwrap()),+))
    };
}

#[macro_export]
macro_rules! unpack {
    ($world:ident, $bundle:ident, ($($get:ty),+ $(,)?)) => {
        ($($world.unpack::<$get>($bundle).unwrap()),+)
    };
}

#[macro_export]
macro_rules! unpack_mut {
    ($world:ident, $bundle:ident, ($($get:ty),+ $(,)?)) => {
        ($($world.unpack_mut::<$get>($bundle).unwrap()),+)
    };
}

// TODO: Figure out how to make this work. Currently the work-around is manual-implementation
/* #[macro_export]
macro_rules! world_query_mut {
    ($world:ident, ($($get:ty),+ $(,)?)) => {

        $world.component_node_bundles(Some(hashbrown::HashSet::from([$(<$get as Component>::hash()),+])), None, None).iter().map(|bundle| ($($world.unpack_mut::<$get>(bundle).unwrap()),+))
    };
} */

#[macro_export]
macro_rules! query_type {
    ($world:ident, ($($get:ty),+ $(,)?)) => {
        Vec::<($($get),+)>::new()
    };
}
