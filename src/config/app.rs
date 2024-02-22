#[derive(Copy, Clone,Debug)]
pub struct App{
    proxy:String,
    auto_detect_discord:bool,
    discord_path:Option<String>
}