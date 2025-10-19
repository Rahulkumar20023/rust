

fn main(){
  let my_numbers=vec![10,20,30,40];

  for num in my_numbers{
    match num{
        30=>println!("thirty"),
        _=>println!("{:?}",num),
    }
  }
  println!("{:?}",my_numbers.len());
  //mynumbers ki ownership for loop ke passs 
  //chali gayi thi, aur jaise hi for loop kahtam hua wai sehu
  //mhy_m=number bhi kharma hi gaya h
  //usliye len wala line prient ni hua tha

}