#[macro_export]
macro_rules! files {
    [$path:expr =>  $($pat:literal),*] => { {
        use rayon::prelude::*;
        let mut files = vec![];
        $(
             files.push(format!("{}/{}", $path, $pat));
        )*
        files.par_iter().map(|i| Files::collect(i)).collect::<Vec<Files>>()
        }
    };
}
