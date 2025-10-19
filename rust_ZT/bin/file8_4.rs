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

fn print_choice(choice:&MenuChoice){
    println!("choice = {:?}",choice);
}

fn main(){
    let choice=get_choice("mainmenu");
//    match choice{
//        Ok(c)=>println!("Selected menu: {:?}",c),
//        Err(e)=>println!("Error: {:?}",e),
//    }
    //println!("Choice: {:?}",choice);
    //print_choice(&choice);
    //ye upar wala error dega kyuki choice ek Result type h,
    //maine fn parameter mai sirf MenuChoice type expect kiya h
    //lekin Err bhi aa skta h
    match choice{
        Ok(inner_choice)=>print_choice(&inner_choice),
        Err(e)=>println!("Error: {:?}",e),
    }
}