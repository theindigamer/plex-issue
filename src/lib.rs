#[macro_use]
extern crate lalrpop_util;
extern crate regex;
#[macro_use]
extern crate plex;

lalrpop_mod!(pub foo);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
