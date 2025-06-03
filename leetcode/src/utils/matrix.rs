#[macro_export]
macro_rules! gen_matrix {
    ( $( [ $( $x:expr ),* ] ),* $(,)? ) => {
        vec![
            $(
                vec![ $( $x ),* ],
            )*
        ]
    };
}