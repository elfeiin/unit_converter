use super::*;

mod length;
pub use length::*;

pub trait Unit {
	const SI_CONVERSION_RATIO: Number;

	fn value(&self) -> Number;
}

#[macro_export]
macro_rules! with_dollar_sign {
	($($body:tt)*) => {
		 macro_rules! __with_dollar_sign { $($body)* }
		 __with_dollar_sign!($);
	}
}

#[macro_export]
macro_rules! unit_macro {
	($m:ident, $t:ident, $b:ident) => {
		with_dollar_sign! {
			($d:tt) => {
				macro_rules! $m {
					($d($d($n:ident, $c:expr),+ $d(,)?)?) => {
						$d(
							$d(
								pub struct $n(Number);

								impl $t for $n {}

								impl Unit for $n {
									const SI_CONVERSION_RATIO: Number = $c;

									fn value(&self) -> Number {
										self.0
									}
								}

								impl From<$b> for $n
								where $b: $t
								{
									fn from(q: $b) -> Self {
										Self(q.value()/Self::SI_CONVERSION_RATIO)
									}
								}

								impl From<$n> for $b
								where $b: $t
								{
									fn from(q: $n) -> Self {
										Self(q.value()*$n::SI_CONVERSION_RATIO)
									}
								}

								// impl<T> From<T> for $n
								// where T: Unit + $t
								// {
								// 	fn from(q: T) -> Self {
								// 		$n::from($b::from(q))
								// 	}
								// }

								// impl<T> From<$n> for T
								// where T: Unit + $t
								// {
								// 	fn from(q: $n) -> Self {
								// 		T::from($b::from(q))
								// 	}
								// }
							)*
						)?
					};
				}
			}
		}
	};
}
