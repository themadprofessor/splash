use hyper::{Client, Uri};
use hyper::client::connect::Connect;
use futures::Future;

use error::Error;

lazy_static! {
    /// URI of the endpoint to get a list of photos from Unsplash.
    pub static ref ME_URI: Uri = format!("{}{}", ::API_URL, "me").parse().unwrap();
}

/// Me endpoint
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Me;

/// A User on Unsplash
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// User ID.
    pub id: String,
    /// Username.
    pub username: String,
    /// User's real name.
    pub name: String,
    /// URL to the user's portfolio.
    pub portfolio_url: Option<String>,
    /// Users's email.
    pub email: Option<String>,
    /// User's bio.
    pub bio: Option<String>,
    /// User's location.
    pub location: Option<String>,
    /// Total number of likes the user has received.
    pub total_likes: usize,
    /// Total number of photos the user has uploaded.
    pub total_photos: usize,
    /// Total number of collections the user has.
    pub total_collections: usize,
    /// User's instagram username.
    pub instagram_username: Option<String>,
    /// User's twitter username.
    pub twitter_username: Option<String>,
    /// URLs to the user's profile image in various sizes.
    pub profile_image: ProfileImages,
    /// Links to the user's profile.
    pub links: UserLinks,
    /// When user's profile was last updated
    pub updated_at: Option<String>,
    /// Is this user followed by the user who accessed the api.
    pub followed_by_user: Option<bool>
}

/// A user's profile images
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileImages {
    /// URL to a small version of the user's profile.
    pub small: String,
    /// URL to a medium version of the user's profile.
    pub medium: String,
    /// URL to a large version of the user's profile.
    pub large: String,
}

/// Links to pages about a user.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserLinks {
    /// Link to the user in the user endpoint.
    #[serde(rename = "self")]
    pub self_link: String,
    /// Link to the user's profile
    pub html: String,
    /// API link to the user's photos.
    pub photos: String,
    /// API link to the user's likes.
    pub likes: String,
    /// API link to the user's profolio.
    pub portfolio: String,
}

impl Me {
    /// Gets the user data of the current user
    ///
    /// # Errors
    /// - Request wrapping a Hyper error is raised if there is an error
    /// handling the HTTP Stream. - MalformedResponse
    ///     - wrapping a JSON error is raised if the JSON returned from
    /// Unsplash is invalid.     - wrapping an IO error is raised if an IO
    /// error occurs.
    pub fn get<C>(self, client: &Client<C>, bearer: &str) -> impl Future<Item=User, Error=Error> where C: Connect + 'static {
        ::endpoint::get((), client, bearer, ME_URI.clone())
    }
}