use super::*;

#[test]
fn prng_generate_state_0_works() {
    let mut prng = Prng { state: 0 };

    assert_eq!(38, prng.get_next());
    assert_eq!(7719, prng.get_next());
    assert_eq!(21238, prng.get_next());
    assert_eq!(2437, prng.get_next());
    assert_eq!(8855, prng.get_next());
    assert_eq!(11797, prng.get_next());
    assert_eq!(8365, prng.get_next());
    assert_eq!(32285, prng.get_next());
    assert_eq!(10450, prng.get_next());
    assert_eq!(30612, prng.get_next());
}

#[test]
fn prng_generate_state_1_works() {
    let mut prng = Prng { state: 1 };

    assert_eq!(41, prng.get_next());
    assert_eq!(18467, prng.get_next());
}
