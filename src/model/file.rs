use serde::{Serialize, Deserialize};
use super::FileResourceFileLinkList;
/**This object represents files hosted on Stripe's servers. You can upload
files with the [create file](https://stripe.com/docs/api#create_file) request
(for example, when uploading dispute evidence). Stripe also
creates files independently (for example, the results of a [Sigma scheduled
query](#scheduled_queries)).

Related guide: [File upload guide](https://stripe.com/docs/file-upload)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct File {
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The file expires and isn't available at this time in epoch seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    ///The suitable name for saving the file to a filesystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///A list of [file links](https://stripe.com/docs/api#file_links) that point at this file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<FileResourceFileLinkList>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The [purpose](https://stripe.com/docs/file-upload#uploading-a-file) of the uploaded file.
    pub purpose: String,
    ///The size of the file object in bytes.
    pub size: i64,
    ///A suitable title for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///The returned file type (for example, `csv`, `pdf`, `jpg`, or `png`).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Use your live secret API key to download the file from this URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}