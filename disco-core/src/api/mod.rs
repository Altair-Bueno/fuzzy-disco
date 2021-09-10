#![allow(dead_code)]
/// /api/media
pub mod media;
/// /api/posts
pub mod posts;
/// Errors that can be produced on the API
pub mod result;
/// /api/sessions
pub mod sessions;
/// /api/users
pub mod users;


const USER_ALIAS: &str = "alias";
const USER_ID:&str = "_id";
const USER_EMAIL:&str = "email";
const USER_PASSWORD:&str = "password";
const USER_DESCRIPTION:&str = "description";
const USER_CREATION_DATE:&str = "creation_date";
const USER_AVATAR:&str = "avatar";

const MEDIA_ID:&str = "_id";
const MEDIA_UPLOADED_BY:&str = "uploaded_by";
const MEDIA_STATUS:&str = "status";
const MEDIA_FORMAT:&str = "format";
const MEDIA_VISIBILITY:&str = "visibility";

const SESSION_ID:&str = "_id";
const SESSION_USER_ALIAS:&str = "user_alias";
const SESSION_IP:&str = "ip";
const SESSION_DATE:&str = "date";

const POSTS_ID:&str = "_id";
const POSTS_TITLE:&str = "title";
const POSTS_CAPTION:&str = "caption";
const POSTS_AUTHOR:&str = "author";
const POSTS_AUDIO:&str = "audio";
const POSTS_PHOTO:&str = "photo";