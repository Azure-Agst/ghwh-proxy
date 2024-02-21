use serde::Deserialize;

/* NOTE:
 *  This isn't everything that is returned from the webhook
 *  Only the stuff we need, really. */

 #[derive(Deserialize)]
pub struct GhostAuthor {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub profile_image: Option<String>
}

#[derive(Deserialize)]
pub struct GhostDocument {
    pub id: String,
    pub uuid: String,
    pub title: String,
    pub slug: String,
    pub feature_image: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub published_at: Option<String>,
    pub primary_author: GhostAuthor,
    pub url: String,
    pub excerpt: Option<String>
}

#[derive(Deserialize)]
pub struct GhostPost {
    pub current: GhostDocument
}

#[derive(Deserialize)]
pub struct GhostWebhook {
    pub post: GhostPost,
}
