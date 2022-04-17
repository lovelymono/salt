#[cfg(test)]
mod tests {
	use rug::Integer;

	#[test]
	fn bigint_works() {
		let mut a = Integer::from(0xff);
		assert_eq!(a.to_string_radix(16), "ff");
		a = a << 96;
		assert_eq!(a.to_string_radix(16), "ff000000000000000000000000");
		a += 0xee;
		assert_eq!(a.to_string_radix(16), "ff0000000000000000000000ee");
	}
}
