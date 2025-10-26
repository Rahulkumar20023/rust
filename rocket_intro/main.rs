use rocket::*;

#[get("/home/<name>")]  //eg: 127.0.0.1:8000/home/Rahul
fn hello_user(name:String)->String{
    format!("Hello {}",name)
}


#[launch]
fn rocket()->_{  //ye unnderscore ka matlab h ki rust runtime pe iss function ka return type determin ekarega
    rocket::build().mount("/",routes![hello_user])

}