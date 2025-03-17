pub trait DiscordMsg {
    fn to_message(&self) -> String;
}
