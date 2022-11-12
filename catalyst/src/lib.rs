pub fn foo() {
    use procmacros::*;

    #[derive(EnumIter)]
    enum Foobar {
        A = 0b0001,
        B = 0b0010,
        C = 0b0100,
    }
}

#[cfg(test)]
mod tests {
    use procmacros::bitflag;

    #[test]
    fn it_works() {
        #[bitflag]
        #[derive(Copy, Clone, Debug)]
        enum Foobar {
            A = 0b0001,
            B = 0b0010,
            C = 0b0100,
        }

        let x = Foobar::all();
        let y = Foobar::A | Foobar::C;

        println!("{:?}", x);

        println!("{}", x - y);
    }
}
