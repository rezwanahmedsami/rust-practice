// Enums are types which have a few definite values
enum Movement {
    // varients
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement){
    // performance action depending on info
    match m {
        Movement::Up => {
            println!("Avatar moving up");
        },
        Movement::Down => {
            println!("Avatar moving Down");
        },
        Movement::Left => {
            println!("Avatar moving Left");
        },
        Movement::Right => println!("Avatar moving Right")
    }
}
pub fn run(){
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Left;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}