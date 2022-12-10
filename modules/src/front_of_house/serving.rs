pub mod hosting {
    pub fn add_to_waitlist() {
        println!("Youve been added to waitlist")
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
