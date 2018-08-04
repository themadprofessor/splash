use chrono::{DateTime, FixedOffset};

mod list;
mod random;

pub use self::list::{List};
pub use self::random::{Random};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Photos;


#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    pub id: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub width: usize,
    pub height: usize,
    pub color: String,
    pub likes: usize,
    pub liked_by_user: bool,
    pub description: Option<String>,
    pub user: User,
    pub current_user_collections: Vec<Collection>,
    pub urls: Urls,
    pub links: PhotoLinks
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub portfolio_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub total_likes: usize,
    pub total_photos: usize,
    pub total_collections: usize,
    pub instagram_username: Option<String>,
    pub twitter_username: Option<String>,
    pub profile_image: ProfileImages,
    pub links: UserLinks
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileImages {
    pub small: String,
    pub medium: String,
    pub large: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLinks {
    #[serde(rename = "self")]
    pub self_link: String,
    pub html: String,
    pub photos: String,
    pub likes: String,
    pub portfolio: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub id: usize,
    pub title: String,
    pub published_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub curated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Urls {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoLinks {
    #[serde(rename = "self")]
    pub self_link: String,
    pub html: String,
    pub download: String,
    pub download_location: String
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Order {
    Latest,
    Oldest,
    Popular
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Orientation {
    Portrait,
    Landscape,
    Squarish
}

impl Default for Order {
    fn default() -> Self {
        Order::Latest
    }
}

impl Photos {
    pub fn list() -> List {
        List::default()
    }

    pub fn random() -> Random {
        Random::default()
    }
}

