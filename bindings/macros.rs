#[macro_export]
macro_rules! application {
	() => {
		#[no_mangle]
		extern "C" fn __start() {
			$crate::async_execute_future(main())
		}
	};
}
