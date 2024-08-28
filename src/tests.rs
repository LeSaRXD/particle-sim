#[test]
fn remap() {
	use crate::remap;

	assert_eq!(remap(0., 0., 1., 0., 1.), 0.);
	assert_eq!(remap(0., 0., 1., 0., 1.), 0.);
	assert_eq!(remap(0., 0., 1., 0., 1.), 0.);
	assert_eq!(remap(0.5, 0., 1., 0., 2.), 1.);
}

#[test]
fn wrap() {
	use crate::helpers::Wrapping;
	use macroquad::math::Vec2;
	let wrap = Vec2::new(200.0, 200.0);

	let v1 = Vec2::new(1.0, 1.0);
	let v2 = Vec2::new(199.0, 199.0);
	assert_eq!(v1.wrap_sub(&v2, &wrap).to_array(), [2.0, 2.0]);

	let v2 = Vec2::new(199.0, 1.0);
	assert_eq!(v1.wrap_sub(&v2, &wrap).to_array(), [2.0, 0.0]);

	let v2 = Vec2::new(1.0, 199.0);
	assert_eq!(v1.wrap_sub(&v2, &wrap).to_array(), [0.0, 2.0]);
}
