struct Rectangle{
    length:u8,
    breadth:u8,
}

fn area_rec(rec:&Rectangle)->u32
{
    let ans:u32=(rec.length as u32)*(rec.breadth as u32);
    return ans;
}

fn main()
{
    let rec_one=Rectangle{
        length:10,
        breadth:5,
    };
    let rec_two=Rectangle{
        length:20,
        breadth:15,
    };
    println!("Area of rec1 : {}",area_rec(&rec_one));
    println!("Area of rec2 : {}",area_rec(&rec_two));

}