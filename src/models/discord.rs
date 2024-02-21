use serde::Serialize;

#[derive(Serialize)]
pub struct DiscordAuthor {
    pub name: String,
    //pub icon_url: String
}

#[derive(Serialize)]
pub struct DiscordImage {
    pub url: String
}

#[derive(Serialize)]
pub struct DiscordFooter {
    pub text: String,
    pub icon_url: String
}

#[derive(Serialize)]
pub struct DiscordEmbed {
    pub author: DiscordAuthor,
    pub title: String,
    pub url: String,
    pub description: String,
    pub image: DiscordImage,
    pub footer: DiscordFooter,
    pub timestamp: String
}

#[derive(Serialize)]
pub struct DiscordWebhook {
    pub content: String,
    pub embeds: Vec<DiscordEmbed>
}