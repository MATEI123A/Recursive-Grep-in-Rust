
use std::path::Path;
use std::{fs, io};


static mut L:i32=0;
unsafe fn traverse_director(dir: &Path, substring: &str, command: &str,number:i32,y:i32) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            traverse_director(&path, substring, command,number,y)?;
        } else if path.is_file() {
            traverse_file(&path, substring, command,number,y)?;
        }
    }
    Ok(())
}

unsafe fn traverse_file(file: &Path, substring: &str, command: &str, number:i32,y:i32) -> io::Result<()> {
    let fisier = fs::read_to_string(file)?;
    let mut matches=0;

    let ok:i32=match command{
        "count_only"=> 2,
        "print_lines"=> 1,
        "max_lines"=>3,
        _=>4,
    };

    for (line_number, line) in fisier.lines().enumerate() {
        for word in line.split(' '){
            if word.to_lowercase().as_str()==substring{
                if ok==1 && y==0{
                    println!("Fisierul este {} iar linia este '{}'",file.display(),line);
                    println!("Numarul liniei este {}",line_number+1);
                    break;
                }

                matches+=1;

                if ok==1 && y==1{

                    if L==number{
                        break;
                    }

                    L+=1;
                    println!("Fisierul este {} iar linia este '{}'",file.display(),line);
                    break;
                }
            }
        }

        if L==number && ok==1 && y==1{
            break;
        }
    }

    if ok==2{
        println!("Cuvantul {} a aparut de {} ori in fisierul {}",substring,matches,file.display());
    }

    Ok(())
}

fn main() -> io::Result<()> {
    unsafe {
        let director = String::from("C:/Users/Matei/Desktop/test");
        let substr: String = "mAtei".to_lowercase();
        let substr = substr.as_str();
        let mut number = 0;
        let mut comanda = String::new();
        let mut ok=0;
        let mut command = String::new();

        println!("Introduceți comanda (ex. 'print_lines', 'count_only'):");
        println!("At print_lines you can put a number which represents max of lines with a specific word");
        io::stdin().read_line(&mut command)?;
        let command = command.trim();

        for i in command.chars() {
            if i.is_ascii_digit() {
                number = number * 10 + i.to_string().parse::<i32>().unwrap();
                ok=1;
            } else if i.is_alphabetic() || i == '_' {
                comanda.push(i);
            }
        }

        println!("Căutăm cuvântul: '{}'", substr);
        traverse_director(Path::new(&director), substr, &comanda, number,ok)?;
    }
    Ok(())
}
