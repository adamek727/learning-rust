mod outermost {

    pub fn middle_function() {}

    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

use outermost::middle_function;
use outermost::inside;

fn try_fns() {
    middle_function();
    outermost::middle_secret_function();
    inside::inner_function();
    inside::secret_function();
}