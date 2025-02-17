#[derive(Debug)]
struct User{
    id:u64,
    name:String,
    gender:Sex,
    marital_status:Status,
}

#[derive(Debug)]
enum Status {
    Married,
    Single,
    Divorce,
    Widow
}

#[derive(Debug)]
enum Sex{
    Male,
    Female
}

impl User {
    fn new_user(id:u64,name:String,gender:Sex,marital_status:Status) -> User{
        User {
            id,
            name,
            gender,
            marital_status,
        }
    }
 
    fn update_user_name(name:String, users_data: &mut Vec<User>, id:u64){
        let updated_name = users_data.iter_mut().find(|x| x.id == id ).unwrap();
        updated_name.name = name;
    }
    fn update_user_sex(gender: Sex,users_data: &mut Vec<User>, id:u64){
        let updated_sex = users_data.iter_mut().find(|x| x.id == id ).unwrap();
        updated_sex.gender=gender;
    }
    fn update_user_marital_status(status:Status,users_data: &mut Vec<User>, id:u64){
        let updated_status = users_data.iter_mut().find(|x| x.id == id ).unwrap();
        updated_status.marital_status=status;
    }
}

fn main() {
    // intro_to_u();
    //string_handler();
    let mut users_data :Vec<User>=  Vec::new();
    users_data.push(User::new_user(5, "yunus".to_string(), Sex::Male, Status::Single));
    users_data.push(User::new_user(4, "Titilola".to_string(), Sex::Female, Status::Divorce));
    users_data.push(User::new_user(3, "yunus".to_string(), Sex::Male, Status::Married));
    users_data.push(User::new_user(2, "funke".to_string(), Sex::Female, Status::Widow));

    User::update_user_name("Iliya".to_string(),&mut users_data,5);
    User::update_user_marital_status(Status::Married,&mut users_data,4);
    User::update_user_sex(Sex::Female,&mut users_data,3);
    User::update_user_name("kemi".to_string(),&mut users_data,2);
 

    //User::update_user_name(&mut user_one, "Abdul".to_string());

    //let q = users_data.iter().find(|&x| x.id == 5 );

    println!("user {:#?}", users_data);
}