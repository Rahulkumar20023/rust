lec 7 jane se phele ka notes h yaha

Option:
*A type that may be one of two things
  *Some data of a specified type
  *Nothing

*Used in scenarios where data may not be required or is unavailable 
    * Unable to find something
    *Ran out of items in a list
    *Form field not filled out

Option ek enumeration ke tarah h jo aisa dikhta h

enum Option<T>{
    Some(T),
    None
}
Some matlab ki kuch value h, None matalb oi value ni h, 

ab tu file dekh le whaa code se samjh aa jayega


lec 8_2 se
Result:
-> A data type that containes one or two types of data: 
    ->"Successfull" data
    ->"Error" data
->Used in scenarios where an action needs to be taken, but has the possibility of failue
    ->Copying a file
    ->Connecting to a website

Kuch aisa dikhta h

enum Result<T,E>{
    Ok(T),
    Err(E)
}

