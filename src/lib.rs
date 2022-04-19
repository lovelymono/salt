#![doc(html_logo_url = "https://raw.githubusercontent.com/lovelymono/salt/default/assets/salt-clear.png")]

#[cfg(test)]
mod tests {
	use rug::Integer;

	#[test]
	fn bigint_works() {
		let mut a = Integer::from(0xff);
		assert_eq!(a.to_string_radix(16), "ff");
		a = (a << 96) + 0xee;
		assert_eq!(a.to_string_radix(16), "ff0000000000000000000000ee");
	}
}
