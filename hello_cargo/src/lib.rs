mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod sarving {
        fn take_order() {
            super::super::eat_at_restaurant();
            self::take_payment();
        }
        fn server_prder() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}
