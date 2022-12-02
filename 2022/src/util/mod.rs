#![allow(dead_code)]

pub type HashMap<K, V> = ahash::AHashMap<K, V>;
pub type HashSet<V> = ahash::AHashSet<V>;

pub trait BStrParse {
	fn parse<F: lexical_core::FromLexical>(&self) -> F;
}

impl BStrParse for [u8] {
	fn parse<F: lexical_core::FromLexical>(&self) -> F {
		lexical_core::parse(self).unwrap()
	}
}
