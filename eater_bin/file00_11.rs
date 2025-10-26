fn main()
{
    let mut vrr=vec!["Hello","World"];
    read_arr(&vrr);
    write_arr(&mut vrr);
    
}

fn read_arr(vrr:&Vec<&str>)
{
    println!("{:?}",vrr);
}

fn write_arr(vrr:&mut Vec<&str>)
{
    vrr.push("New");
}