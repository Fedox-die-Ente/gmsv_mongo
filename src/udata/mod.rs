macro_rules! udata {
	(
		$(#[$outer:meta])*
		$vis:vis struct $name:ident {
			$(
				$fieldvis:vis $field:ident: $ty:ty
			),*
		}
		$($rest:tt)*
	) => {
		#[repr(C)]
		#[derive(
			derive_more::Add,
			derive_more::AddAssign,
			derive_more::Sub,
			derive_more::SubAssign,
			derive_more::Mul,
			derive_more::MulAssign,
			derive_more::Div,
			derive_more::DivAssign,
			derive_more::Rem,
			derive_more::RemAssign,
			derive_more::Into,
			PartialEq,
			PartialOrd,
			Debug,
			Default,
			Copy,
			Clone
		)]
		$(#[$outer])*
		$vis struct $name {
			$(
				$fieldvis $field: $ty
			),*
		}

		impl $name {
			pub fn new( $($field: $ty),* ) -> $name {
				$name {
					$($field),*
				}
			}
		}
		udata!( $($rest)* );
	};
	() => ()
}

udata! {
	pub struct MongoDBClient {
		pub client: mongodb::Client
	}

	pub struct MongoDBDatabase {
		pub database: mongodb::Database
	}

    pub struct MongoDBCollection {
        pub collection: mongodb::Collection<mongodb::bson::Document>
    }
}