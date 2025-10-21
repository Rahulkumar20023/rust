fn main(){
    let sentence=String::from("my name is harkirat");
    let first_word:String=get_first_word(sentence);
    println!("First word is {}",first_word);
    let mut str=String::from("hello world");
    str.push_str("gfhjk");
    println!("{}",str);
}

fn get_first_word(sentence:String)->String{
    let mut ans:String=String::from("");
    for ch in sentence.chars(){   //sentence.chars() borrow ke tarah kaam kar raha h, yaha par sentence ki ownership for loop ko tranfer ni hui h
        if ch==' '{
            break;
        }
        else{
            ans.push(ch);
        }
    }
    return ans;
}