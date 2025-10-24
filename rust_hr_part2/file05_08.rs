fn main()
{
    let vec1=vec![1,2,3,4,5];
    let iter1=vec1.iter();
    let iter2=iter1.filter(|x| *x%2!=0);
    let iter3=iter2.map(|x| x*2);
    for x in iter3 {
        print!("{} ",x);
    }

}