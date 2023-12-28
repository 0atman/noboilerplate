use std::fs;
use std::env;

fn main() {
    let script_name: &String = &env::args().collect::<Vec<_>>()[1];
    let script = fs::read_to_string("../scripts/".to_string() + &script_name).expect("missing file");
    let lines: Vec<&str> = script.split('\n').collect();
    let comment_lines = ["#", "%%", "f7f7f7", "- Tell", "---", "notes:", "![[", "[github.com", "https://", "- https://"];
    let html_tags = ["style", "split"];
    let mut break_reason = BreakReason::None;
    let mut new_lines = String::new();
    'lines: for line in lines {
        for html_tag in html_tags{
            if line.starts_with(&("<".to_string()+ html_tag)){
                break_reason=BreakReason::Style;
            }
            if line.starts_with(&("</".to_string() + html_tag)){
                break_reason=BreakReason::None;
                continue 'lines;
            }
        }
        for comment in comment_lines {
            if line.starts_with(comment){
                continue 'lines;
            }
        }
        if line.starts_with("```"){
            break_reason = match break_reason {
                BreakReason::Code => BreakReason::None,
                _ => BreakReason::Code,
            };
            continue;
        }
        if break_reason != BreakReason::None {
            continue;
        }
        if line.is_empty(){
            continue;
        }
        new_lines.push_str(&(line.to_owned() + "\n"));
    }
    let _ = fs::write(script_name[..script_name.len()-2].to_string() + "txt", new_lines);
    println!("done");
}

#[derive(PartialEq, Debug)]
enum BreakReason{
    None,
    Code,
    Style,
}
