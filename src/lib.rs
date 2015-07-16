#![deny(missing_docs)]

//! A macro library for backend agnostic design.

#[macro_export]
macro_rules! backend_macro_items { ($($x:item)+) => ($($x)+) }

#[macro_export]
macro_rules! backend {
    ($( $x:ident [$($w:tt)*] ),*) => {backend_macro_items! {

        /// Used to associate an item with a backend.
        pub trait Associated: OfBackend {
            /// The backend.
            type Backend: Backend;
        }

        /// Implemented by backends.
        pub trait Backend {
            $(
            /// An associated item.
            type $x: Associated<Backend = Self> + $($w)*;
            )*
        }

        /// Used to get other types associated with backend.
        pub trait OfBackend {
            $(
            type $x;
            )*
        }

        impl<T> OfBackend for T where T: Associated {
            $(
            type $x = <<T as Associated>::Backend as Backend>::$x;
            )*
        }

    }}
}

#[macro_export]
macro_rules! backend_impl {
    ($x:ident {
        $( $y:ident = $t:path ),*
    }) => {

        $(
            impl Associated for $t {
                type Backend = $x;
            }
        )*

        impl Backend for $x {
            $( type $y = $t; )*
        }

    }
}

