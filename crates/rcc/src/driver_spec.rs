extern crate hamcrest2;

#[cfg(test)]
mod driver_spec {

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
    
}
