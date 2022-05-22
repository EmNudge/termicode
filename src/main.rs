mod data;
mod ui;
mod query;
use clipboard::{ ClipboardContext, ClipboardProvider };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let data = data::get_unicode_data().await?;
    let unicode_file = data::parse_unicode_data(&data);
    
    let user_selection = ui::create_interface(&unicode_file)?;
    
    if let Some(unicode_data) = user_selection {
        let symbol = char::from_u32(unicode_data.codepoint).unwrap_or('�');
        let mut pargs = pico_args::Arguments::from_env();

        if let Some(output_type) = pargs.opt_value_from_str::<&str, String>("--output").unwrap() {
            match output_type.as_str() {
                "symbol" => {
                    print!("{}", symbol);
                }
                "codepoint" => {
                    print!("{}", unicode_data.codepoint);
                }
                "name" => {
                    print!("{}", unicode_data.name);
                }
                _ => {
                    println!("invalid display type \"{}\". Choose one of: symbol, codepoint, name", output_type);
                }
            }
        } else {
            println!("{} ({:#x}) - {}", &symbol, unicode_data.codepoint, unicode_data.name);
        }
        
        if pargs.contains("--copy") {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            
            ctx.set_contents(symbol.to_string()).unwrap();
        }
    }


    Ok(())
}
