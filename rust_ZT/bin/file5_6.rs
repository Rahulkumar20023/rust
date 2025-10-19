struct Test{
    score:i32,
}

fn main(){
    let my_scores=vec![
        Test{score:99},
        Test{score:85},
        Test{score:76},
        Test{score:88},
    ];

    for test in my_scores{
        println!("{:?}",test.score);
    }   

}