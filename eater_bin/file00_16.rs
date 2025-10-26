fn main()
{
    let vec=vec![1,2,3,4];

    let double_vec: Vec<i32>=vec.iter().map(|x| *x*2).collect();
    println!("{:?}",double_vec);

    let even_vec:Vec<&i32>=vec.iter().filter(|x| *x%2==0).collect();
    println!("{:?}",even_vec);

    let even_vec:Vec<i32>=vec.into_iter().filter(|x| x%2==0).collect();
    println!("{:?}",even_vec);
}