trait Course {
    fn get_method(&self) -> String;
}

struct Workshop {
    title: String,
    instructor: String,
    duration: u32,
}

struct Seminar {
    title: String,
    speaker: String,
    location: String,
}

impl Course for Workshop {
    fn get_method(&self) -> String {
        format!(
            "title: {}\n instructor: {}\n duration: {}\n",
            self.title, self.instructor, self.duration
        )
    }
}

impl Course for Seminar {
    fn get_method(&self) -> String {
        format!(
            "title: {}\n speaker: {}\n location: {}\n",
            self.title, self.speaker, self.location
        )
    }
}

fn print_overview<T:Course>(t:T){
    print!("{:?}",t.get_method());
}

fn main()
{
    let workshop=Workshop{
        title:"Blockchain".to_owned(),
        instructor:"Code Eater".to_owned(),
        duration:10,
    };

    let seminar=Seminar{
        title:"Web Development".to_owned(),
        speaker:"Karan".to_owned(),
        location:"Mumbai".to_owned(),
    };
    print_overview(workshop);
    print_overview(seminar);

}