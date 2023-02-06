#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Georgia,
    Florida,
    California,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/** The code snippet that follows this comment is equivalent to this:
 *          let config_max = Some(3u8);
 *          match config_max {
 *              Some(max) => println!("The maximum is configured to be {}", max),
 *              _ => (),
 *          }
 *  An if let expression works the same as a match one, but it allows to
 *  code only the arm you are interested in. The trade-off for this is
 *  that if let is less exhaustive than a match statement, so choosing
 *  between both will depend on the need for an exhausting checking.
 * 
 *  In other words, if let is syntax sugar for a match that runs code when 
 *  the value matches one pattern.
 */
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    /* Another example of match vs if let */
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Above match is equivalent to following if let

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
