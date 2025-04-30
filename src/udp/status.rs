#![allow(clippy::doc_markdown)]
//! Enums representing AniDB UDP API status and error codes.
//! Generated based on the UDP API Definition HTML file (Version: 0.03.730).

use std::convert::TryFrom;
use thiserror::Error;

/// Represents all possible numeric status codes returned by the AniDB UDP API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum StatusCode {
    // --- 2xx Success ---
    /// 200: LOGIN_ACCEPTED
    LoginAccepted = 200,
    /// 201: LOGIN_ACCEPTED_NEW_VERSION
    LoginAcceptedNewVersion = 201,
    /// 203: LOGGED_OUT
    LoggedOut = 203,
    /// 205: RESOURCE - (Specific resource info follows, context dependent)
    Resource = 205,
    /// 206: STATS - (DEPRECATED/DISABLED in spec)
    Stats = 206,
    /// 207: TOP - (DEPRECATED/DISABLED in spec)
    Top = 207,
    /// 208: UPTIME
    Uptime = 208,
    /// 209: ENCRYPTION_ENABLED
    EncryptionEnabled = 209,
    /// 210: MYLIST_ENTRY_ADDED
    MylistEntryAdded = 210,
    /// 211: MYLIST_ENTRY_DELETED
    MylistEntryDeleted = 211,
    /// 214: ADDED_FILE
    AddedFile = 214,
    /// 215: ADDED_STREAM
    AddedStream = 215,
    /// 217: EXPORT_QUEUED
    ExportQueued = 217,
    /// 218: EXPORT_CANCELLED
    ExportCancelled = 218,
    /// 219: ENCODING_CHANGED
    EncodingChanged = 219,
    /// 220: FILE
    File = 220,
    /// 221: MYLIST
    Mylist = 221,
    /// 222: MYLIST_STATS
    MylistStats = 222,
    /// 223: WISHLIST
    Wishlist = 223,
    /// 224: NOTIFICATION
    Notification = 224, // Renamed to avoid clash with 290
    /// 225: GROUP_STATUS
    GroupStatus = 225,
    /// 226: WISHLIST_ENTRY_ADDED
    WishlistEntryAdded = 226,
    /// 227: WISHLIST_ENTRY_DELETED
    WishlistEntryDeleted = 227,
    /// 228: WISHLIST_ENTRY_UPDATED
    WishlistEntryUpdated = 228,
    /// 229: MULTIPLE_WISHLIST
    MultipleWishlist = 229,
    /// 230: ANIME
    Anime = 230,
    /// 231: ANIME_BEST_MATCH
    AnimeBestMatch = 231,
    /// 232: RANDOM_ANIME
    RandomAnime = 232,
    /// 233: ANIME_DESCRIPTION
    AnimeDescription = 233,
    /// 234: REVIEW
    Review = 234,
    /// 235: CHARACTER
    Character = 235,
    /// 236: SONG
    Song = 236,
    /// 237: ANIMETAG
    Animetag = 237,
    /// 238: CHARACTERTAG
    Charactertag = 238,
    /// 240: EPISODE
    Episode = 240,
    /// 243: UPDATED
    Updated = 243,
    /// 244: TITLE
    Title = 244,
    /// 245: CREATOR
    Creator = 245,
    /// 246: NOTIFICATION_ENTRY_ADDED
    NotificationEntryAdded = 246,
    /// 247: NOTIFICATION_ENTRY_DELETED
    NotificationEntryDeleted = 247,
    /// 248: NOTIFICATION_ENTRY_UPDATE
    NotificationEntryUpdate = 248,
    /// 249: MULTIPLE_NOTIFICATION
    MultipleNotification = 249,
    /// 250: GROUP
    Group = 250,
    /// 251: CATEGORY
    Category = 251,
    /// 253: BUDDY_LIST
    BuddyList = 253,
    /// 254: BUDDY_STATE
    BuddyState = 254,
    /// 255: BUDDY_ADDED
    BuddyAdded = 255,
    /// 256: BUDDY_DELETED
    BuddyDeleted = 256,
    /// 257: BUDDY_ACCEPTED
    BuddyAccepted = 257,
    /// 258: BUDDY_DENIED
    BuddyDenied = 258,
    /// 260: VOTED
    Voted = 260,
    /// 261: VOTE_FOUND
    VoteFound = 261,
    /// 262: VOTE_UPDATED
    VoteUpdated = 262,
    /// 263: VOTE_REVOKED
    VoteRevoked = 263,
    /// 265: HOT_ANIME
    HotAnime = 265,
    /// 266: RANDOM_RECOMMENDATION
    RandomRecommendation = 266,
    /// 267: RANDOM_SIMILAR
    RandomSimilar = 267,
    /// 270: NOTIFICATION_ENABLED
    NotificationEnabled = 270,
    /// 281: NOTIFYACK_SUCCESSFUL_MESSAGE
    NotifyackSuccessfulMessage = 281,
    /// 282: NOTIFYACK_SUCCESSFUL_NOTIFICATION
    NotifyackSuccessfulNotification = 282,
    /// 290: NOTIFICATION_STATE (from NOTIFY command)
    NotificationState = 290,
    /// 291: NOTIFYLIST
    Notifylist = 291,
    /// 292: NOTIFYGET_MESSAGE
    NotifygetMessage = 292,
    /// 293: NOTIFYGET_NOTIFY
    NotifygetNotify = 293,
    /// 294: SENDMESSAGE_SUCCESSFUL
    SendmessageSuccessful = 294,
    /// 295: USER_ID
    UserId = 295,
    /// 297: CALENDAR
    Calendar = 297,

    // --- 3xx Client Errors (Informational/Warnings) ---
    /// 300: PONG
    Pong = 300,
    /// 301: AUTHPONG
    Authpong = 301,
    /// 305: NO_SUCH_RESOURCE
    NoSuchResource = 305,
    /// 309: API_PASSWORD_NOT_DEFINED
    ApiPasswordNotDefined = 309,
    /// 310: FILE_ALREADY_IN_MYLIST
    FileAlreadyInMylist = 310,
    /// 311: MYLIST_ENTRY_EDITED
    MylistEntryEdited = 311,
    /// 312: MULTIPLE_MYLIST_ENTRIES
    MultipleMylistEntries = 312,
    /// 313: WATCHED
    Watched = 313,
    /// 314: SIZE_HASH_EXISTS
    SizeHashExists = 314,
    /// 315: INVALID_DATA
    InvalidData = 315,
    /// 316: STREAMNOID_USED
    StreamnoidUsed = 316,
    /// 317: EXPORT_NO_SUCH_TEMPLATE
    ExportNoSuchTemplate = 317,
    /// 318: EXPORT_ALREADY_IN_QUEUE
    ExportAlreadyInQueue = 318,
    /// 319: EXPORT_NO_EXPORT_QUEUED_OR_IS_PROCESSING
    ExportNoExportQueuedOrIsProcessing = 319,
    /// 320: NO_SUCH_FILE
    NoSuchFile = 320,
    /// 321: NO_SUCH_ENTRY (Context: MYLIST)
    NoSuchEntry = 321,
    /// 322: MULTIPLE_FILES_FOUND
    MultipleFilesFound = 322,
    /// 323: NO_SUCH_WISHLIST
    NoSuchWishlist = 323,
    /// 324: NO_SUCH_NOTIFICATION_ITEM (Context: NOTIFICATIONDEL)
    NoSuchNotificationItem = 324,
    /// 325: NO_GROUPS_FOUND (Context: GROUPSTATUS)
    NoGroupsFound = 325,
    /// 330: NO_SUCH_ANIME
    NoSuchAnime = 330,
    /// 333: NO_SUCH_DESCRIPTION
    NoSuchDescription = 333,
    /// 334: NO_SUCH_REVIEW
    NoSuchReview = 334,
    /// 335: NO_SUCH_CHARACTER
    NoSuchCharacter = 335,
    /// 336: NO_SUCH_SONG
    NoSuchSong = 336,
    /// 337: NO_SUCH_ANIMETAG
    NoSuchAnimetag = 337,
    /// 338: NO_SUCH_CHARACTERTAG
    NoSuchCharactertag = 338,
    /// 340: NO_SUCH_EPISODE
    NoSuchEpisode = 340,
    /// 343: NO_SUCH_UPDATES / NO_UPDATES
    NoUpdates = 343,
    /// 344: NO_SUCH_TITLES
    NoSuchTitles = 344,
    /// 345: NO_SUCH_CREATOR
    NoSuchCreator = 345,
    /// 350: NO_SUCH_GROUP
    NoSuchGroup = 350,
    /// 351: NO_SUCH_CATEGORY
    NoSuchCategory = 351,
    /// 355: BUDDY_ALREADY_ADDED
    BuddyAlreadyAdded = 355,
    /// 356: NO_SUCH_BUDDY
    NoSuchBuddy = 356,
    /// 357: BUDDY_ALREADY_ACCEPTED
    BuddyAlreadyAccepted = 357,
    /// 358: BUDDY_ALREADY_DENIED
    BuddyAlreadyDenied = 358,
    /// 360: NO_SUCH_VOTE
    NoSuchVote = 360,
    /// 361: INVALID_VOTE_TYPE
    InvalidVoteType = 361,
    /// 362: INVALID_VOTE_VALUE
    InvalidVoteValue = 362,
    /// 363: PERMVOTE_NOT_ALLOWED
    PermvoteNotAllowed = 363,
    /// 364: ALREADY_PERMVOTED
    AlreadyPermvoted = 364,
    /// 365: HOT_ANIME_EMPTY
    HotAnimeEmpty = 365,
    /// 366: RANDOM_RECOMMENDATION_EMPTY
    RandomRecommendationEmpty = 366,
    /// 367: RANDOM_SIMILAR_EMPTY
    RandomSimilarEmpty = 367,
    /// 370: NOTIFICATION_DISABLED
    NotificationDisabled = 370,
    /// 381: NO_SUCH_ENTRY (Context: NOTIFYACK Message)
    NoSuchEntryMessage = 381,
    /// 382: NO_SUCH_ENTRY (Context: NOTIFYACK Notification)
    NoSuchEntryNotification = 382,
    /// 392: NO_SUCH_MESSAGE (Context: NOTIFYGET)
    NoSuchMessage = 392,
    /// 393: NO_SUCH_NOTIFY (Context: NOTIFYGET)
    NoSuchNotify = 393,
    /// 394: NO_SUCH_USER
    NoSuchUser = 394,
    /// 397: CALENDAR_EMPTY
    CalendarEmpty = 397,
    /// 399: NO_CHANGES (Context: NOTIFICATIONADD)
    NoChanges = 399,

    // --- 4xx Client Errors (Fatal) ---
    /// 403: NOT_LOGGED_IN
    NotLoggedIn = 403,
    /// 410: NO_SUCH_MYLIST_FILE
    NoSuchMylistFile = 410,
    /// 411: NO_SUCH_MYLIST_ENTRY (Context: MYLISTADD/DEL Edit)
    NoSuchMylistEntry = 411,
    /// 412: MYLIST_UNAVAILABLE
    MylistUnavailable = 412,

    // --- 5xx Server Errors (Client Related) ---
    /// 500: LOGIN_FAILED
    LoginFailed = 500,
    /// 501: LOGIN_FIRST
    LoginFirst = 501,
    /// 502: ACCESS_DENIED
    AccessDenied = 502,
    /// 503: CLIENT_VERSION_OUTDATED
    ClientVersionOutdated = 503,
    /// 504: CLIENT_BANNED
    ClientBanned = 504,
    /// 505: ILLEGAL_INPUT_OR_ACCESS_DENIED
    IllegalInputOrAccessDenied = 505,
    /// 506: INVALID_SESSION
    InvalidSession = 506,
    /// 509: NO_SUCH_ENCRYPTION_TYPE
    NoSuchEncryptionType = 509,
    /// 519: ENCODING_NOT_SUPPORTED
    EncodingNotSupported = 519,
    /// 555: BANNED (IP Ban)
    Banned = 555,
    /// 598: UNKNOWN_COMMAND
    UnknownCommand = 598,

    // --- 6xx Server Errors (Internal/Maintenance) ---
    /// 600: INTERNAL_SERVER_ERROR
    InternalServerError = 600,
    /// 601: ANIDB_OUT_OF_SERVICE
    AnidbOutOfService = 601,
    /// 602: SERVER_BUSY
    ServerBusy = 602,
    /// 603: NO_DATA
    NoData = 603,
    /// 604: TIMEOUT_-_DELAY_AND_RESUBMIT
    TimeoutDelayAndResubmit = 604,
    /// 666: API_VIOLATION - (No longer in spec, but good to keep)
    ApiViolation = 666,

    // --- 7xx Push/Notify Specific ---
    /// 701: PUSHACK_CONFIRMED
    PushackConfirmed = 701,
    /// 702: NO_SUCH_PACKET_PENDING
    NoSuchPacketPending = 702,
    // Note: 720, 753, 794, 799 are incoming push notifications, not reply codes.

    // --- 9xx Meta ---
    /// 998: VERSION
    Version = 998,
}

/// Represents only the error status codes returned by the AniDB UDP API.
/// This includes codes in the 3xx, 4xx, 5xx, and 6xx ranges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ErrorCode {
    // --- 3xx Client Errors (Informational/Warnings) ---
    /// 305: NO_SUCH_RESOURCE
    NoSuchResource = 305,
    /// 309: API_PASSWORD_NOT_DEFINED
    ApiPasswordNotDefined = 309,
    /// 310: FILE_ALREADY_IN_MYLIST
    FileAlreadyInMylist = 310,
    /// 311: MYLIST_ENTRY_EDITED
    MylistEntryEdited = 311,
    /// 312: MULTIPLE_MYLIST_ENTRIES
    MultipleMylistEntries = 312,
    /// 313: WATCHED
    Watched = 313,
    /// 314: SIZE_HASH_EXISTS
    SizeHashExists = 314,
    /// 315: INVALID_DATA
    InvalidData = 315,
    /// 316: STREAMNOID_USED
    StreamnoidUsed = 316,
    /// 317: EXPORT_NO_SUCH_TEMPLATE
    ExportNoSuchTemplate = 317,
    /// 318: EXPORT_ALREADY_IN_QUEUE
    ExportAlreadyInQueue = 318,
    /// 319: EXPORT_NO_EXPORT_QUEUED_OR_IS_PROCESSING
    ExportNoExportQueuedOrIsProcessing = 319,
    /// 320: NO_SUCH_FILE
    NoSuchFile = 320,
    /// 321: NO_SUCH_ENTRY (Context: MYLIST)
    NoSuchEntry = 321,
    /// 322: MULTIPLE_FILES_FOUND
    MultipleFilesFound = 322,
    /// 323: NO_SUCH_WISHLIST
    NoSuchWishlist = 323,
    /// 324: NO_SUCH_NOTIFICATION_ITEM (Context: NOTIFICATIONDEL)
    NoSuchNotificationItem = 324,
    /// 325: NO_GROUPS_FOUND (Context: GROUPSTATUS)
    NoGroupsFound = 325,
    /// 330: NO_SUCH_ANIME
    NoSuchAnime = 330,
    /// 333: NO_SUCH_DESCRIPTION
    NoSuchDescription = 333,
    /// 334: NO_SUCH_REVIEW
    NoSuchReview = 334,
    /// 335: NO_SUCH_CHARACTER
    NoSuchCharacter = 335,
    /// 336: NO_SUCH_SONG
    NoSuchSong = 336,
    /// 337: NO_SUCH_ANIMETAG
    NoSuchAnimetag = 337,
    /// 338: NO_SUCH_CHARACTERTAG
    NoSuchCharactertag = 338,
    /// 340: NO_SUCH_EPISODE
    NoSuchEpisode = 340,
    /// 343: NO_SUCH_UPDATES / NO_UPDATES
    NoUpdates = 343,
    /// 344: NO_SUCH_TITLES
    NoSuchTitles = 344,
    /// 345: NO_SUCH_CREATOR
    NoSuchCreator = 345,
    /// 350: NO_SUCH_GROUP
    NoSuchGroup = 350,
    /// 351: NO_SUCH_CATEGORY
    NoSuchCategory = 351,
    /// 355: BUDDY_ALREADY_ADDED
    BuddyAlreadyAdded = 355,
    /// 356: NO_SUCH_BUDDY
    NoSuchBuddy = 356,
    /// 357: BUDDY_ALREADY_ACCEPTED
    BuddyAlreadyAccepted = 357,
    /// 358: BUDDY_ALREADY_DENIED
    BuddyAlreadyDenied = 358,
    /// 360: NO_SUCH_VOTE
    NoSuchVote = 360,
    /// 361: INVALID_VOTE_TYPE
    InvalidVoteType = 361,
    /// 362: INVALID_VOTE_VALUE
    InvalidVoteValue = 362,
    /// 363: PERMVOTE_NOT_ALLOWED
    PermvoteNotAllowed = 363,
    /// 364: ALREADY_PERMVOTED
    AlreadyPermvoted = 364,
    /// 365: HOT_ANIME_EMPTY
    HotAnimeEmpty = 365,
    /// 366: RANDOM_RECOMMENDATION_EMPTY
    RandomRecommendationEmpty = 366,
    /// 367: RANDOM_SIMILAR_EMPTY
    RandomSimilarEmpty = 367,
    /// 370: NOTIFICATION_DISABLED
    NotificationDisabled = 370,
    /// 381: NO_SUCH_ENTRY (Context: NOTIFYACK Message)
    NoSuchEntryMessage = 381,
    /// 382: NO_SUCH_ENTRY (Context: NOTIFYACK Notification)
    NoSuchEntryNotification = 382,
    /// 392: NO_SUCH_MESSAGE (Context: NOTIFYGET)
    NoSuchMessage = 392,
    /// 393: NO_SUCH_NOTIFY (Context: NOTIFYGET)
    NoSuchNotify = 393,
    /// 394: NO_SUCH_USER
    NoSuchUser = 394,
    /// 397: CALENDAR_EMPTY
    CalendarEmpty = 397,
    /// 399: NO_CHANGES (Context: NOTIFICATIONADD)
    NoChanges = 399,

    // --- 4xx Client Errors (Fatal) ---
    /// 403: NOT_LOGGED_IN
    NotLoggedIn = 403,
    /// 410: NO_SUCH_MYLIST_FILE
    NoSuchMylistFile = 410,
    /// 411: NO_SUCH_MYLIST_ENTRY (Context: MYLISTADD/DEL Edit)
    NoSuchMylistEntry = 411,
    /// 412: MYLIST_UNAVAILABLE
    MylistUnavailable = 412,

    // --- 5xx Server Errors (Client Related) ---
    /// 500: LOGIN_FAILED
    LoginFailed = 500,
    /// 501: LOGIN_FIRST
    LoginFirst = 501,
    /// 502: ACCESS_DENIED
    AccessDenied = 502,
    /// 503: CLIENT_VERSION_OUTDATED
    ClientVersionOutdated = 503,
    /// 504: CLIENT_BANNED
    ClientBanned = 504,
    /// 505: ILLEGAL_INPUT_OR_ACCESS_DENIED
    IllegalInputOrAccessDenied = 505,
    /// 506: INVALID_SESSION
    InvalidSession = 506,
    /// 509: NO_SUCH_ENCRYPTION_TYPE
    NoSuchEncryptionType = 509,
    /// 519: ENCODING_NOT_SUPPORTED
    EncodingNotSupported = 519,
    /// 555: BANNED (IP Ban)
    Banned = 555,
    /// 598: UNKNOWN_COMMAND
    UnknownCommand = 598,

    // --- 6xx Server Errors (Internal/Maintenance) ---
    /// 600: INTERNAL_SERVER_ERROR
    InternalServerError = 600,
    /// 601: ANIDB_OUT_OF_SERVICE
    AnidbOutOfService = 601,
    /// 602: SERVER_BUSY
    ServerBusy = 602,
    /// 603: NO_DATA
    NoData = 603,
    /// 604: TIMEOUT_-_DELAY_AND_RESUBMIT
    TimeoutDelayAndResubmit = 604,
    /// 666: API_VIOLATION - (No longer in spec, but good to keep)
    ApiViolation = 666,
}

impl StatusCode {
    /// Returns the numeric representation of the status code.
    pub fn as_u16(self) -> u16 {
        self as u16
    }

    /// Attempts to create a StatusCode from a numeric value.
    /// Returns None if the number does not correspond to a known status code.
    #[allow(clippy::too_many_lines)]
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            200 => Some(StatusCode::LoginAccepted),
            201 => Some(StatusCode::LoginAcceptedNewVersion),
            203 => Some(StatusCode::LoggedOut),
            205 => Some(StatusCode::Resource),
            206 => Some(StatusCode::Stats),
            207 => Some(StatusCode::Top),
            208 => Some(StatusCode::Uptime),
            209 => Some(StatusCode::EncryptionEnabled),
            210 => Some(StatusCode::MylistEntryAdded),
            211 => Some(StatusCode::MylistEntryDeleted),
            214 => Some(StatusCode::AddedFile),
            215 => Some(StatusCode::AddedStream),
            217 => Some(StatusCode::ExportQueued),
            218 => Some(StatusCode::ExportCancelled),
            219 => Some(StatusCode::EncodingChanged),
            220 => Some(StatusCode::File),
            221 => Some(StatusCode::Mylist),
            222 => Some(StatusCode::MylistStats),
            223 => Some(StatusCode::Wishlist),
            224 => Some(StatusCode::Notification),
            225 => Some(StatusCode::GroupStatus),
            226 => Some(StatusCode::WishlistEntryAdded),
            227 => Some(StatusCode::WishlistEntryDeleted),
            228 => Some(StatusCode::WishlistEntryUpdated),
            229 => Some(StatusCode::MultipleWishlist),
            230 => Some(StatusCode::Anime),
            231 => Some(StatusCode::AnimeBestMatch),
            232 => Some(StatusCode::RandomAnime),
            233 => Some(StatusCode::AnimeDescription),
            234 => Some(StatusCode::Review),
            235 => Some(StatusCode::Character),
            236 => Some(StatusCode::Song),
            237 => Some(StatusCode::Animetag),
            238 => Some(StatusCode::Charactertag),
            240 => Some(StatusCode::Episode),
            243 => Some(StatusCode::Updated),
            244 => Some(StatusCode::Title),
            245 => Some(StatusCode::Creator),
            246 => Some(StatusCode::NotificationEntryAdded),
            247 => Some(StatusCode::NotificationEntryDeleted),
            248 => Some(StatusCode::NotificationEntryUpdate),
            249 => Some(StatusCode::MultipleNotification),
            250 => Some(StatusCode::Group),
            251 => Some(StatusCode::Category),
            253 => Some(StatusCode::BuddyList),
            254 => Some(StatusCode::BuddyState),
            255 => Some(StatusCode::BuddyAdded),
            256 => Some(StatusCode::BuddyDeleted),
            257 => Some(StatusCode::BuddyAccepted),
            258 => Some(StatusCode::BuddyDenied),
            260 => Some(StatusCode::Voted),
            261 => Some(StatusCode::VoteFound),
            262 => Some(StatusCode::VoteUpdated),
            263 => Some(StatusCode::VoteRevoked),
            265 => Some(StatusCode::HotAnime),
            266 => Some(StatusCode::RandomRecommendation),
            267 => Some(StatusCode::RandomSimilar),
            270 => Some(StatusCode::NotificationEnabled),
            281 => Some(StatusCode::NotifyackSuccessfulMessage),
            282 => Some(StatusCode::NotifyackSuccessfulNotification),
            290 => Some(StatusCode::NotificationState),
            291 => Some(StatusCode::Notifylist),
            292 => Some(StatusCode::NotifygetMessage),
            293 => Some(StatusCode::NotifygetNotify),
            294 => Some(StatusCode::SendmessageSuccessful),
            295 => Some(StatusCode::UserId),
            297 => Some(StatusCode::Calendar),
            300 => Some(StatusCode::Pong),
            301 => Some(StatusCode::Authpong),
            305 => Some(StatusCode::NoSuchResource),
            309 => Some(StatusCode::ApiPasswordNotDefined),
            310 => Some(StatusCode::FileAlreadyInMylist),
            311 => Some(StatusCode::MylistEntryEdited),
            312 => Some(StatusCode::MultipleMylistEntries),
            313 => Some(StatusCode::Watched),
            314 => Some(StatusCode::SizeHashExists),
            315 => Some(StatusCode::InvalidData),
            316 => Some(StatusCode::StreamnoidUsed),
            317 => Some(StatusCode::ExportNoSuchTemplate),
            318 => Some(StatusCode::ExportAlreadyInQueue),
            319 => Some(StatusCode::ExportNoExportQueuedOrIsProcessing),
            320 => Some(StatusCode::NoSuchFile),
            321 => Some(StatusCode::NoSuchEntry),
            322 => Some(StatusCode::MultipleFilesFound),
            323 => Some(StatusCode::NoSuchWishlist),
            324 => Some(StatusCode::NoSuchNotificationItem),
            325 => Some(StatusCode::NoGroupsFound),
            330 => Some(StatusCode::NoSuchAnime),
            333 => Some(StatusCode::NoSuchDescription),
            334 => Some(StatusCode::NoSuchReview),
            335 => Some(StatusCode::NoSuchCharacter),
            336 => Some(StatusCode::NoSuchSong),
            337 => Some(StatusCode::NoSuchAnimetag),
            338 => Some(StatusCode::NoSuchCharactertag),
            340 => Some(StatusCode::NoSuchEpisode),
            343 => Some(StatusCode::NoUpdates),
            344 => Some(StatusCode::NoSuchTitles),
            345 => Some(StatusCode::NoSuchCreator),
            350 => Some(StatusCode::NoSuchGroup),
            351 => Some(StatusCode::NoSuchCategory),
            355 => Some(StatusCode::BuddyAlreadyAdded),
            356 => Some(StatusCode::NoSuchBuddy),
            357 => Some(StatusCode::BuddyAlreadyAccepted),
            358 => Some(StatusCode::BuddyAlreadyDenied),
            360 => Some(StatusCode::NoSuchVote),
            361 => Some(StatusCode::InvalidVoteType),
            362 => Some(StatusCode::InvalidVoteValue),
            363 => Some(StatusCode::PermvoteNotAllowed),
            364 => Some(StatusCode::AlreadyPermvoted),
            365 => Some(StatusCode::HotAnimeEmpty),
            366 => Some(StatusCode::RandomRecommendationEmpty),
            367 => Some(StatusCode::RandomSimilarEmpty),
            370 => Some(StatusCode::NotificationDisabled),
            381 => Some(StatusCode::NoSuchEntryMessage),
            382 => Some(StatusCode::NoSuchEntryNotification),
            392 => Some(StatusCode::NoSuchMessage),
            393 => Some(StatusCode::NoSuchNotify),
            394 => Some(StatusCode::NoSuchUser),
            397 => Some(StatusCode::CalendarEmpty),
            399 => Some(StatusCode::NoChanges),
            403 => Some(StatusCode::NotLoggedIn),
            410 => Some(StatusCode::NoSuchMylistFile),
            411 => Some(StatusCode::NoSuchMylistEntry),
            412 => Some(StatusCode::MylistUnavailable),
            500 => Some(StatusCode::LoginFailed),
            501 => Some(StatusCode::LoginFirst),
            502 => Some(StatusCode::AccessDenied),
            503 => Some(StatusCode::ClientVersionOutdated),
            504 => Some(StatusCode::ClientBanned),
            505 => Some(StatusCode::IllegalInputOrAccessDenied),
            506 => Some(StatusCode::InvalidSession),
            509 => Some(StatusCode::NoSuchEncryptionType),
            519 => Some(StatusCode::EncodingNotSupported),
            555 => Some(StatusCode::Banned),
            598 => Some(StatusCode::UnknownCommand),
            600 => Some(StatusCode::InternalServerError),
            601 => Some(StatusCode::AnidbOutOfService),
            602 => Some(StatusCode::ServerBusy),
            603 => Some(StatusCode::NoData),
            604 => Some(StatusCode::TimeoutDelayAndResubmit),
            666 => Some(StatusCode::ApiViolation),
            701 => Some(StatusCode::PushackConfirmed),
            702 => Some(StatusCode::NoSuchPacketPending),
            998 => Some(StatusCode::Version),
            _ => None,
        }
    }

    /// Checks if the status code represents an error (3xx-6xx range).
    ///
    /// Note that 3xx codes are often informational or warnings, but are
    /// categorized as errors by the API specification.
    pub fn is_error(self) -> bool {
        ErrorCode::try_from(self).is_ok()
    }

    /// Checks if the status code represents a success (not an error).
    pub fn is_success(self) -> bool {
        !self.is_error()
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl ErrorCode {
    /// Returns the numeric representation of the error code.
    pub fn as_u16(self) -> u16 {
        self as u16
    }

    /// Attempts to create an ErrorCode from a numeric value.
    /// Returns None if the number does not correspond to a known error code (3xx-6xx range).
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            305 => Some(ErrorCode::NoSuchResource),
            309 => Some(ErrorCode::ApiPasswordNotDefined),
            310 => Some(ErrorCode::FileAlreadyInMylist),
            311 => Some(ErrorCode::MylistEntryEdited),
            312 => Some(ErrorCode::MultipleMylistEntries),
            313 => Some(ErrorCode::Watched),
            314 => Some(ErrorCode::SizeHashExists),
            315 => Some(ErrorCode::InvalidData),
            316 => Some(ErrorCode::StreamnoidUsed),
            317 => Some(ErrorCode::ExportNoSuchTemplate),
            318 => Some(ErrorCode::ExportAlreadyInQueue),
            319 => Some(ErrorCode::ExportNoExportQueuedOrIsProcessing),
            320 => Some(ErrorCode::NoSuchFile),
            321 => Some(ErrorCode::NoSuchEntry),
            322 => Some(ErrorCode::MultipleFilesFound),
            323 => Some(ErrorCode::NoSuchWishlist),
            324 => Some(ErrorCode::NoSuchNotificationItem),
            325 => Some(ErrorCode::NoGroupsFound),
            330 => Some(ErrorCode::NoSuchAnime),
            333 => Some(ErrorCode::NoSuchDescription),
            334 => Some(ErrorCode::NoSuchReview),
            335 => Some(ErrorCode::NoSuchCharacter),
            336 => Some(ErrorCode::NoSuchSong),
            337 => Some(ErrorCode::NoSuchAnimetag),
            338 => Some(ErrorCode::NoSuchCharactertag),
            340 => Some(ErrorCode::NoSuchEpisode),
            343 => Some(ErrorCode::NoUpdates),
            344 => Some(ErrorCode::NoSuchTitles),
            345 => Some(ErrorCode::NoSuchCreator),
            350 => Some(ErrorCode::NoSuchGroup),
            351 => Some(ErrorCode::NoSuchCategory),
            355 => Some(ErrorCode::BuddyAlreadyAdded),
            356 => Some(ErrorCode::NoSuchBuddy),
            357 => Some(ErrorCode::BuddyAlreadyAccepted),
            358 => Some(ErrorCode::BuddyAlreadyDenied),
            360 => Some(ErrorCode::NoSuchVote),
            361 => Some(ErrorCode::InvalidVoteType),
            362 => Some(ErrorCode::InvalidVoteValue),
            363 => Some(ErrorCode::PermvoteNotAllowed),
            364 => Some(ErrorCode::AlreadyPermvoted),
            365 => Some(ErrorCode::HotAnimeEmpty),
            366 => Some(ErrorCode::RandomRecommendationEmpty),
            367 => Some(ErrorCode::RandomSimilarEmpty),
            370 => Some(ErrorCode::NotificationDisabled),
            381 => Some(ErrorCode::NoSuchEntryMessage),
            382 => Some(ErrorCode::NoSuchEntryNotification),
            392 => Some(ErrorCode::NoSuchMessage),
            393 => Some(ErrorCode::NoSuchNotify),
            394 => Some(ErrorCode::NoSuchUser),
            397 => Some(ErrorCode::CalendarEmpty),
            399 => Some(ErrorCode::NoChanges),
            403 => Some(ErrorCode::NotLoggedIn),
            410 => Some(ErrorCode::NoSuchMylistFile),
            411 => Some(ErrorCode::NoSuchMylistEntry),
            412 => Some(ErrorCode::MylistUnavailable),
            500 => Some(ErrorCode::LoginFailed),
            501 => Some(ErrorCode::LoginFirst),
            502 => Some(ErrorCode::AccessDenied),
            503 => Some(ErrorCode::ClientVersionOutdated),
            504 => Some(ErrorCode::ClientBanned),
            505 => Some(ErrorCode::IllegalInputOrAccessDenied),
            506 => Some(ErrorCode::InvalidSession),
            509 => Some(ErrorCode::NoSuchEncryptionType),
            519 => Some(ErrorCode::EncodingNotSupported),
            555 => Some(ErrorCode::Banned),
            598 => Some(ErrorCode::UnknownCommand),
            600 => Some(ErrorCode::InternalServerError),
            601 => Some(ErrorCode::AnidbOutOfService),
            602 => Some(ErrorCode::ServerBusy),
            603 => Some(ErrorCode::NoData),
            604 => Some(ErrorCode::TimeoutDelayAndResubmit),
            666 => Some(ErrorCode::ApiViolation),
            _ => None,
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

/// Error type for TryFrom<StatusCode> conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("Status code is not an error code")]
pub struct NotAnErrorCode;

/// Convert ErrorCode to StatusCode (always possible).
impl From<ErrorCode> for StatusCode {
    fn from(err: ErrorCode) -> Self {
        StatusCode::from_u16(err.as_u16()).unwrap()
    }
}

/// Try to convert StatusCode to ErrorCode (only possible if StatusCode is in 3xx-6xx range).
impl TryFrom<StatusCode> for ErrorCode {
    type Error = NotAnErrorCode;

    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        ErrorCode::from_u16(status.as_u16()).ok_or(NotAnErrorCode)
    }
}

#[cfg(test)]
mod test_status_error_codes {
    use super::*;

    #[test]
    fn test_status_code_conversion() {
        let code = 200;
        let status = StatusCode::from_u16(code).expect("Should be valid");
        assert_eq!(status, StatusCode::LoginAccepted);
        assert_eq!(status.as_u16(), code);

        let code = 9999;
        assert_eq!(StatusCode::from_u16(code), None);
    }

    #[test]
    fn test_error_code_conversion() {
        let code = 501;
        let err_code = ErrorCode::from_u16(code).expect("Should be valid error");
        assert_eq!(err_code, ErrorCode::LoginFirst);
        assert_eq!(err_code.as_u16(), code);

        let code = 200; // Not an error
        assert_eq!(ErrorCode::from_u16(code), None);
    }

    #[test]
    fn test_error_to_status() {
        let err = ErrorCode::LoginFailed;
        let status: StatusCode = err.into();
        assert_eq!(status, StatusCode::LoginFailed);
    }

    #[test]
    fn test_status_to_error() {
        let status_err = StatusCode::ClientBanned;
        let err = ErrorCode::try_from(status_err);
        assert_eq!(err, Ok(ErrorCode::ClientBanned));

        let status_ok = StatusCode::LoggedOut;
        let err = ErrorCode::try_from(status_ok);
        assert_eq!(err, Err(NotAnErrorCode));

        let status_pong = StatusCode::Pong; // 300, technically an error code here
        let err = ErrorCode::try_from(status_pong);
        assert!(
            err.is_err(),
            "Pong (300) is not listed as an error code variant"
        ); // Check if 300 was intentionally excluded from ErrorCode variants

        let status_no_such_res = StatusCode::NoSuchResource; // 305
        let err = ErrorCode::try_from(status_no_such_res);
        assert_eq!(err, Ok(ErrorCode::NoSuchResource));

        let status_push = StatusCode::PushackConfirmed; // 701
        let err = ErrorCode::try_from(status_push);
        assert_eq!(err, Err(NotAnErrorCode));
    }
}
