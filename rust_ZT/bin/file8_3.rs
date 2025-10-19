#[derive(Debug)]
enum MenuChoice{
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input:&str)->Result<MenuChoice,String>{
    match input{
        "mainmenu"=>Ok(MenuChoice::MainMenu),
        "start"=>Ok(MenuChoice::Start),
        "quit"=>Ok(MenuChoice::Quit),
        _=>Err("Invalid choice".to_owned()),
    }
}

fn main(){
    let choice=get_choice("mainmenu");
//    match choice{
//        Ok(c)=>println!("Selected menu: {:?}",c),
//        Err(e)=>println!("Error: {:?}",e),
//    }
    println!("Choice: {:?}",choice);
}