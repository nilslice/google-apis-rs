// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Games* crate version *0.1.0+20150309*, where *20150309* is the exact revision of the *games:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.0*.
//! 
//! Everything else about the *Games* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/games/services/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/games1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Games.html) ... 
//! 
//! * [achievement definitions](struct.AchievementDefinition.html)
//!  * [*list*](struct.AchievementDefinitionListCall.html)
//! * achievements
//!  * [*increment*](struct.AchievementIncrementCall.html), [*list*](struct.AchievementListCall.html), [*reveal*](struct.AchievementRevealCall.html), [*set steps at least*](struct.AchievementSetStepsAtLeastCall.html), [*unlock*](struct.AchievementUnlockCall.html) and [*update multiple*](struct.AchievementUpdateMultipleCall.html)
//! * [applications](struct.Application.html)
//!  * [*get*](struct.ApplicationGetCall.html) and [*played*](struct.ApplicationPlayedCall.html)
//! * events
//!  * [*list by player*](struct.EventListByPlayerCall.html), [*list definitions*](struct.EventListDefinitionCall.html) and [*record*](struct.EventRecordCall.html)
//! * [leaderboards](struct.Leaderboard.html)
//!  * [*get*](struct.LeaderboardGetCall.html) and [*list*](struct.LeaderboardListCall.html)
//! * metagame
//!  * [*get metagame config*](struct.MetagameGetMetagameConfigCall.html) and [*list categories by player*](struct.MetagameListCategoriesByPlayerCall.html)
//! * [players](struct.Player.html)
//!  * [*get*](struct.PlayerGetCall.html) and [*list*](struct.PlayerListCall.html)
//! * pushtokens
//!  * [*remove*](struct.PushtokenRemoveCall.html) and [*update*](struct.PushtokenUpdateCall.html)
//! * [quest milestones](struct.QuestMilestone.html)
//!  * [*claim*](struct.QuestMilestoneClaimCall.html)
//! * [quests](struct.Quest.html)
//!  * [*accept*](struct.QuestAcceptCall.html) and [*list*](struct.QuestListCall.html)
//! * revisions
//!  * [*check*](struct.RevisionCheckCall.html)
//! * [rooms](struct.Room.html)
//!  * [*create*](struct.RoomCreateCall.html), [*decline*](struct.RoomDeclineCall.html), [*dismiss*](struct.RoomDismisCall.html), [*get*](struct.RoomGetCall.html), [*join*](struct.RoomJoinCall.html), [*leave*](struct.RoomLeaveCall.html), [*list*](struct.RoomListCall.html) and [*report status*](struct.RoomReportStatuCall.html)
//! * scores
//!  * [*get*](struct.ScoreGetCall.html), [*list*](struct.ScoreListCall.html), [*list window*](struct.ScoreListWindowCall.html), [*submit*](struct.ScoreSubmitCall.html) and [*submit multiple*](struct.ScoreSubmitMultipleCall.html)
//! * [snapshots](struct.Snapshot.html)
//!  * [*get*](struct.SnapshotGetCall.html) and [*list*](struct.SnapshotListCall.html)
//! * turn based matches
//!  * [*cancel*](struct.TurnBasedMatcheCancelCall.html), [*create*](struct.TurnBasedMatcheCreateCall.html), [*decline*](struct.TurnBasedMatcheDeclineCall.html), [*dismiss*](struct.TurnBasedMatcheDismisCall.html), [*finish*](struct.TurnBasedMatcheFinishCall.html), [*get*](struct.TurnBasedMatcheGetCall.html), [*join*](struct.TurnBasedMatcheJoinCall.html), [*leave*](struct.TurnBasedMatcheLeaveCall.html), [*leave turn*](struct.TurnBasedMatcheLeaveTurnCall.html), [*list*](struct.TurnBasedMatcheListCall.html), [*rematch*](struct.TurnBasedMatcheRematchCall.html), [*sync*](struct.TurnBasedMatcheSyncCall.html) and [*take turn*](struct.TurnBasedMatcheTakeTurnCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Games.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.turn_based_matches().decline(...).doit()
//! let r = hub.turn_based_matches().leave(...).doit()
//! let r = hub.turn_based_matches().finish(...).doit()
//! let r = hub.turn_based_matches().take_turn(...).doit()
//! let r = hub.turn_based_matches().create(...).doit()
//! let r = hub.turn_based_matches().join(...).doit()
//! let r = hub.turn_based_matches().leave_turn(...).doit()
//! let r = hub.turn_based_matches().get(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-games1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "google-games1" as games1;
//! use games1::Result;
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use games1::Games;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Games::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.turn_based_matches().leave_turn("matchId", -18)
//!              .pending_participant_id("kasd")
//!              .language("accusam")
//!              .doit();
//! 
//! match result {
//!     Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!     Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!     Result::MissingToken => println!("OAuth2: Missing Token"),
//!     Result::Cancelled => println!("Operation cancelled by user"),
//!     Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!     Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!     Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!     Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     Result::Success(_) => println!("Success (value doesn't print)"),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downlods
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via json. Optionals are used to indicate that partial requests are responses are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifyable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(core,io,thread_sleep)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate "yup-oauth2" as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, ResourceMethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// Know your basic profile info and list of people in your circles.
    PluLogin,

    /// Share your Google+ profile information and view and manage your game activity
    Full,

    /// View and manage its own configuration data in your Google Drive
    DriveAppdata,
}

impl Str for Scope {
    fn as_slice(&self) -> &str {
        match *self {
            Scope::PluLogin => "https://www.googleapis.com/auth/plus.login",
            Scope::Full => "https://www.googleapis.com/auth/games",
            Scope::DriveAppdata => "https://www.googleapis.com/auth/drive.appdata",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Games related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// use games1::Result;
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().leave_turn("matchId", -70)
///              .pending_participant_id("amet.")
///              .language("erat")
///              .doit();
/// 
/// match result {
///     Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
///     Result::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///     Result::MissingToken => println!("OAuth2: Missing Token"),
///     Result::Cancelled => println!("Operation cancelled by user"),
///     Result::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///     Result::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///     Result::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///     Result::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     Result::Success(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct Games<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for Games<C, NC, A> {}

impl<'a, C, NC, A> Games<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Games<C, NC, A> {
        Games {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.0".to_string(),
            _m: PhantomData
        }
    }

    pub fn achievement_definitions(&'a self) -> AchievementDefinitionMethods<'a, C, NC, A> {
        AchievementDefinitionMethods { hub: &self }
    }
    pub fn achievements(&'a self) -> AchievementMethods<'a, C, NC, A> {
        AchievementMethods { hub: &self }
    }
    pub fn applications(&'a self) -> ApplicationMethods<'a, C, NC, A> {
        ApplicationMethods { hub: &self }
    }
    pub fn events(&'a self) -> EventMethods<'a, C, NC, A> {
        EventMethods { hub: &self }
    }
    pub fn leaderboards(&'a self) -> LeaderboardMethods<'a, C, NC, A> {
        LeaderboardMethods { hub: &self }
    }
    pub fn metagame(&'a self) -> MetagameMethods<'a, C, NC, A> {
        MetagameMethods { hub: &self }
    }
    pub fn players(&'a self) -> PlayerMethods<'a, C, NC, A> {
        PlayerMethods { hub: &self }
    }
    pub fn pushtokens(&'a self) -> PushtokenMethods<'a, C, NC, A> {
        PushtokenMethods { hub: &self }
    }
    pub fn quest_milestones(&'a self) -> QuestMilestoneMethods<'a, C, NC, A> {
        QuestMilestoneMethods { hub: &self }
    }
    pub fn quests(&'a self) -> QuestMethods<'a, C, NC, A> {
        QuestMethods { hub: &self }
    }
    pub fn revisions(&'a self) -> RevisionMethods<'a, C, NC, A> {
        RevisionMethods { hub: &self }
    }
    pub fn rooms(&'a self) -> RoomMethods<'a, C, NC, A> {
        RoomMethods { hub: &self }
    }
    pub fn scores(&'a self) -> ScoreMethods<'a, C, NC, A> {
        ScoreMethods { hub: &self }
    }
    pub fn snapshots(&'a self) -> SnapshotMethods<'a, C, NC, A> {
        SnapshotMethods { hub: &self }
    }
    pub fn turn_based_matches(&'a self) -> TurnBasedMatcheMethods<'a, C, NC, A> {
        TurnBasedMatcheMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// This is a JSON template for data related to individual game categories.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Category {
    /// The category name.    
    pub category: String,
    /// Experience points earned in this category.    
    #[serde(alias="experiencePoints")]
    pub experience_points: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#category.    
    pub kind: String,
}

impl Part for Category {}


/// This is a JSON template for a third party player list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list players](struct.PlayerListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerListResponse {
    /// Token corresponding to the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The players.    
    pub items: Vec<Player>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerListResponse.    
    pub kind: String,
}

impl ResponseResult for PlayerListResponse {}


/// This is a JSON template for a player score.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerScore {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerScore.    
    pub kind: String,
    /// The numerical value for this player score.    
    pub score: String,
    /// The formatted score for this player score.    
    #[serde(alias="formattedScore")]
    pub formatted_score: String,
    /// Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.    
    #[serde(alias="scoreTag")]
    pub score_tag: String,
    /// The time span for this player score.
    /// Possible values are:  
    /// - "ALL_TIME" - The score is an all-time score. 
    /// - "WEEKLY" - The score is a weekly score. 
    /// - "DAILY" - The score is a daily score.
    #[serde(alias="timeSpan")]
    pub time_span: String,
}

impl Part for PlayerScore {}


/// This is a JSON template for the Instance resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Instance {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#instance.    
    pub kind: String,
    /// URI which shows where a user can acquire this instance.    
    #[serde(alias="acquisitionUri")]
    pub acquisition_uri: String,
    /// Localized display name.    
    pub name: String,
    /// Flag to show if this game instance supports turn based play.    
    #[serde(alias="turnBasedPlay")]
    pub turn_based_play: bool,
    /// Platform dependent details for Web.    
    #[serde(alias="webInstance")]
    pub web_instance: InstanceWebDetails,
    /// Platform dependent details for Android.    
    #[serde(alias="androidInstance")]
    pub android_instance: InstanceAndroidDetails,
    /// Platform dependent details for iOS.    
    #[serde(alias="iosInstance")]
    pub ios_instance: InstanceIosDetails,
    /// The platform type.
    /// Possible values are:  
    /// - "ANDROID" - Instance is for Android. 
    /// - "IOS" - Instance is for iOS 
    /// - "WEB_APP" - Instance is for Web App.
    #[serde(alias="platformType")]
    pub platform_type: String,
    /// Flag to show if this game instance supports realtime play.    
    #[serde(alias="realtimePlay")]
    pub realtime_play: bool,
}

impl Part for Instance {}


/// This is a JSON template for an achievement definition object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list achievement definitions](struct.AchievementDefinitionListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AchievementDefinition {
    /// The total steps for an incremental achievement.    
    #[serde(alias="totalSteps")]
    pub total_steps: Option<i32>,
    /// The type of the achievement.
    /// Possible values are:  
    /// - "STANDARD" - Achievement is either locked or unlocked. 
    /// - "INCREMENTAL" - Achievement is incremental.
    #[serde(alias="achievementType")]
    pub achievement_type: Option<String>,
    /// The description of the achievement.    
    pub description: Option<String>,
    /// The total steps for an incremental achievement as a string.    
    #[serde(alias="formattedTotalSteps")]
    pub formatted_total_steps: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#achievementDefinition.    
    pub kind: Option<String>,
    /// The initial state of the achievement.
    /// Possible values are:  
    /// - "HIDDEN" - Achievement is hidden. 
    /// - "REVEALED" - Achievement is revealed. 
    /// - "UNLOCKED" - Achievement is unlocked.
    #[serde(alias="initialState")]
    pub initial_state: Option<String>,
    /// Experience points which will be earned when unlocking this achievement.    
    #[serde(alias="experiencePoints")]
    pub experience_points: Option<String>,
    /// The ID of the achievement.    
    pub id: Option<String>,
    /// Indicates whether the revealed icon image being returned is a default image, or is provided by the game.    
    #[serde(alias="isRevealedIconUrlDefault")]
    pub is_revealed_icon_url_default: Option<bool>,
    /// The image URL for the unlocked achievement icon.    
    #[serde(alias="unlockedIconUrl")]
    pub unlocked_icon_url: Option<String>,
    /// The image URL for the revealed achievement icon.    
    #[serde(alias="revealedIconUrl")]
    pub revealed_icon_url: Option<String>,
    /// Indicates whether the unlocked icon image being returned is a default image, or is game-provided.    
    #[serde(alias="isUnlockedIconUrlDefault")]
    pub is_unlocked_icon_url_default: Option<bool>,
    /// The name of the achievement.    
    pub name: Option<String>,
}

impl Resource for AchievementDefinition {}


/// This is a JSON template for an achievement unlock response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [unlock achievements](struct.AchievementUnlockCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AchievementUnlockResponse {
    /// Whether this achievement was newly unlocked (that is, whether the unlock request for the achievement was the first for the player).    
    #[serde(alias="newlyUnlocked")]
    pub newly_unlocked: bool,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#achievementUnlockResponse.    
    pub kind: String,
}

impl ResponseResult for AchievementUnlockResponse {}


/// This is a JSON template for a room auto-match criteria object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoomAutoMatchingCriteria {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomAutoMatchingCriteria.    
    pub kind: String,
    /// The minimum number of players that should be added to the room by auto-matching.    
    #[serde(alias="minAutoMatchingPlayers")]
    pub min_auto_matching_players: i32,
    /// A bitmask indicating when auto-matches are valid. When ANDed with other exclusive bitmasks, the result must be zero. Can be used to support exclusive roles within a game.    
    #[serde(alias="exclusiveBitmask")]
    pub exclusive_bitmask: String,
    /// The maximum number of players that should be added to the room by auto-matching.    
    #[serde(alias="maxAutoMatchingPlayers")]
    pub max_auto_matching_players: i32,
}

impl Part for RoomAutoMatchingCriteria {}


/// This is a JSON template for an event period update resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct EventUpdateRequest {
    /// The ID of the event being modified in this update.    
    #[serde(alias="definitionId")]
    pub definition_id: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#eventUpdateRequest.    
    pub kind: String,
    /// The number of times this event occurred in this time period.    
    #[serde(alias="updateCount")]
    pub update_count: i64,
}

impl Part for EventUpdateRequest {}


/// This is a JSON template for a list of turn-based matches.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list turn based matches](struct.TurnBasedMatcheListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct TurnBasedMatchList {
    /// The pagination token for the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The matches.    
    pub items: Vec<TurnBasedMatch>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchList.    
    pub kind: String,
}

impl ResponseResult for TurnBasedMatchList {}


/// This is a JSON template for an achievement unlock response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update multiple achievements](struct.AchievementUpdateMultipleCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AchievementUpdateMultipleResponse {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#achievementUpdateListResponse.    
    pub kind: String,
    /// The updated state of the achievements.    
    #[serde(alias="updatedAchievements")]
    pub updated_achievements: Vec<AchievementUpdateResponse>,
}

impl ResponseResult for AchievementUpdateMultipleResponse {}


/// This is a JSON template for an event child relationship resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct EventChild {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#eventChild.    
    pub kind: String,
    /// The ID of the child event.    
    #[serde(alias="childId")]
    pub child_id: String,
}

impl Part for EventChild {}


/// This is a JSON template for an event update failure resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct EventRecordFailure {
    /// The ID of the event that was not updated.    
    #[serde(alias="eventId")]
    pub event_id: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#eventRecordFailure.    
    pub kind: String,
    /// The cause for the update failure.
    /// Possible values are:  
    /// - "NOT_FOUND" - An attempt was made to set an event that was not defined. 
    /// - "INVALID_UPDATE_VALUE" - An attempt was made to increment an event by a non-positive value.
    #[serde(alias="failureCause")]
    pub failure_cause: String,
}

impl Part for EventRecordFailure {}


/// This is a JSON template for an achievement update response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AchievementUpdateResponse {
    /// The current steps recorded for this achievement if it is incremental.    
    #[serde(alias="currentSteps")]
    pub current_steps: i32,
    /// Whether this achievement was newly unlocked (that is, whether the unlock request for the achievement was the first for the player).    
    #[serde(alias="newlyUnlocked")]
    pub newly_unlocked: bool,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#achievementUpdateResponse.    
    pub kind: String,
    /// The achievement this update is was applied to.    
    #[serde(alias="achievementId")]
    pub achievement_id: String,
    /// Whether the requested updates actually affected the achievement.    
    #[serde(alias="updateOccurred")]
    pub update_occurred: bool,
    /// The current state of the achievement.
    /// Possible values are:  
    /// - "HIDDEN" - Achievement is hidden. 
    /// - "REVEALED" - Achievement is revealed. 
    /// - "UNLOCKED" - Achievement is unlocked.
    #[serde(alias="currentState")]
    pub current_state: String,
}

impl Part for AchievementUpdateResponse {}


/// This is a JSON template for room modification metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RoomModification {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomModification.    
    pub kind: String,
    /// The timestamp at which they modified the room, in milliseconds since the epoch in UTC.    
    #[serde(alias="modifiedTimestampMillis")]
    pub modified_timestamp_millis: String,
    /// The ID of the participant that modified the room.    
    #[serde(alias="participantId")]
    pub participant_id: String,
}

impl Part for RoomModification {}


/// This is a JSON template for a ListDefinitions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list definitions events](struct.EventListDefinitionCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct EventDefinitionListResponse {
    /// The pagination token for the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The event definitions.    
    pub items: Vec<EventDefinition>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#eventDefinitionListResponse.    
    pub kind: String,
}

impl ResponseResult for EventDefinitionListResponse {}


/// This is a JSON template for a list of leaderboard entry resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [submit scores](struct.ScoreSubmitCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerScoreResponse {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerScoreResponse.    
    pub kind: String,
    /// The time spans where the submitted score is better than the existing score for that time span.
    /// Possible values are:  
    /// - "ALL_TIME" - The score is an all-time score. 
    /// - "WEEKLY" - The score is a weekly score. 
    /// - "DAILY" - The score is a daily score.
    #[serde(alias="beatenScoreTimeSpans")]
    pub beaten_score_time_spans: Vec<String>,
    /// Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.    
    #[serde(alias="scoreTag")]
    pub score_tag: String,
    /// The scores in time spans that have not been beaten. As an example, the submitted score may be better than the player's DAILY score, but not better than the player's scores for the WEEKLY or ALL_TIME time spans.    
    #[serde(alias="unbeatenScores")]
    pub unbeaten_scores: Vec<PlayerScore>,
    /// The leaderboard ID that this score was submitted to.    
    #[serde(alias="leaderboardId")]
    pub leaderboard_id: String,
    /// The formatted value of the submitted score.    
    #[serde(alias="formattedScore")]
    pub formatted_score: String,
}

impl ResponseResult for PlayerScoreResponse {}


/// This is a JSON template for an image asset object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ImageAsset {
    /// The URL of the asset.    
    pub url: String,
    /// The width of the asset.    
    pub width: i32,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#imageAsset.    
    pub kind: String,
    /// The name of the asset.    
    pub name: String,
    /// The height of the asset.    
    pub height: i32,
}

impl Part for ImageAsset {}


/// This is a JSON template for an update on the status of peers in a room.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report status rooms](struct.RoomReportStatuCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct RoomP2PStatuses {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomP2PStatuses.    
    pub kind: Option<String>,
    /// The updates for the peers.    
    pub updates: Option<Vec<RoomP2PStatus>>,
}

impl RequestValue for RoomP2PStatuses {}


/// This is a JSON template for an achievement increment response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [increment achievements](struct.AchievementIncrementCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AchievementIncrementResponse {
    /// The current steps recorded for this incremental achievement.    
    #[serde(alias="currentSteps")]
    pub current_steps: i32,
    /// Whether the the current steps for the achievement has reached the number of steps required to unlock.    
    #[serde(alias="newlyUnlocked")]
    pub newly_unlocked: bool,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#achievementIncrementResponse.    
    pub kind: String,
}

impl ResponseResult for AchievementIncrementResponse {}


/// This is a JSON template for a list of turn-based matches returned from a sync.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sync turn based matches](struct.TurnBasedMatcheSyncCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct TurnBasedMatchSync {
    /// The pagination token for the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The matches.    
    pub items: Vec<TurnBasedMatch>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchSync.    
    pub kind: String,
    /// True if there were more matches available to fetch at the time the response was generated (which were not returned due to page size limits.)    
    #[serde(alias="moreAvailable")]
    pub more_available: bool,
}

impl ResponseResult for TurnBasedMatchSync {}


/// This is a JSON template for a turn-based match creation request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create turn based matches](struct.TurnBasedMatcheCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct TurnBasedMatchCreateRequest {
    /// The player ids to invite to the match.    
    #[serde(alias="invitedPlayerIds")]
    pub invited_player_ids: Option<Vec<String>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchCreateRequest.    
    pub kind: Option<String>,
    /// Criteria for auto-matching players into this match.    
    #[serde(alias="autoMatchingCriteria")]
    pub auto_matching_criteria: Option<TurnBasedAutoMatchingCriteria>,
    /// The variant / mode of the application to be played. This can be any integer value, or left blank. You should use a small number of variants to keep the auto-matching pool as large as possible.    
    pub variant: Option<i32>,
    /// A randomly generated numeric ID. This number is used at the server to ensure that the request is handled correctly across retries.    
    #[serde(alias="requestId")]
    pub request_id: Option<String>,
}

impl RequestValue for TurnBasedMatchCreateRequest {}


/// This is a JSON template for a Quest resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accept quests](struct.QuestAcceptCall.html) (response)
/// * [list quests](struct.QuestListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Quest {
    /// The description of the quest.    
    pub description: String,
    /// The banner image URL for the quest.    
    #[serde(alias="bannerUrl")]
    pub banner_url: String,
    /// The timestamp at which the quest was last updated by the user in milliseconds since the epoch in UTC. Only present if the player has accepted the quest.    
    #[serde(alias="lastUpdatedTimestampMillis")]
    pub last_updated_timestamp_millis: String,
    /// The timestamp at which the user accepted the quest in milliseconds since the epoch in UTC. Only present if the player has accepted the quest.    
    #[serde(alias="acceptedTimestampMillis")]
    pub accepted_timestamp_millis: String,
    /// The icon image URL for the quest.    
    #[serde(alias="iconUrl")]
    pub icon_url: String,
    /// The timestamp at which the user should be notified that the quest will end soon in milliseconds since the epoch in UTC.    
    #[serde(alias="notifyTimestampMillis")]
    pub notify_timestamp_millis: String,
    /// The ID of the application this quest is part of.    
    #[serde(alias="applicationId")]
    pub application_id: String,
    /// The ID of the quest.    
    pub id: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#quest.    
    pub kind: String,
    /// The quest milestones.    
    pub milestones: Vec<QuestMilestone>,
    /// The name of the quest.    
    pub name: String,
    /// The timestamp at which the quest becomes active in milliseconds since the epoch in UTC.    
    #[serde(alias="startTimestampMillis")]
    pub start_timestamp_millis: String,
    /// The timestamp at which the quest ceases to be active in milliseconds since the epoch in UTC.    
    #[serde(alias="endTimestampMillis")]
    pub end_timestamp_millis: String,
    /// The state of the quest.
    /// Possible values are:  
    /// - "UPCOMING": The quest is upcoming. The user can see the quest, but cannot accept it until it is open. 
    /// - "OPEN": The quest is currently open and may be accepted at this time. 
    /// - "ACCEPTED": The user is currently participating in this quest. 
    /// - "COMPLETED": The user has completed the quest. 
    /// - "FAILED": The quest was attempted but was not completed before the deadline expired. 
    /// - "EXPIRED": The quest has expired and was not accepted. 
    /// - "DELETED": The quest should be deleted from the local database.
    pub state: String,
    /// Indicates whether the banner image being returned is a default image, or is game-provided.    
    #[serde(alias="isDefaultBannerUrl")]
    pub is_default_banner_url: bool,
    /// Indicates whether the icon image being returned is a default image, or is game-provided.    
    #[serde(alias="isDefaultIconUrl")]
    pub is_default_icon_url: bool,
}

impl Resource for Quest {}
impl ResponseResult for Quest {}


/// This is a JSON template for aggregate stats.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct AggregateStats {
    /// The number of messages sent between a pair of peers.    
    pub count: String,
    /// The maximum amount.    
    pub max: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#aggregateStats.    
    pub kind: String,
    /// The total number of bytes sent for messages between a pair of peers.    
    pub sum: String,
    /// The minimum amount.    
    pub min: String,
}

impl Part for AggregateStats {}


/// This is a JSON template for an achievement set steps at least response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set steps at least achievements](struct.AchievementSetStepsAtLeastCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AchievementSetStepsAtLeastResponse {
    /// The current steps recorded for this incremental achievement.    
    #[serde(alias="currentSteps")]
    pub current_steps: i32,
    /// Whether the the current steps for the achievement has reached the number of steps required to unlock.    
    #[serde(alias="newlyUnlocked")]
    pub newly_unlocked: bool,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#achievementSetStepsAtLeastResponse.    
    pub kind: String,
}

impl ResponseResult for AchievementSetStepsAtLeastResponse {}


/// This is a JSON template for the metagame config resource
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get metagame config metagame](struct.MetagameGetMetagameConfigCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct MetagameConfig {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#metagameConfig.    
    pub kind: String,
    /// Current version of the metagame configuration data. When this data is updated, the version number will be increased by one.    
    #[serde(alias="currentVersion")]
    pub current_version: i32,
    /// The list of player levels.    
    #[serde(alias="playerLevels")]
    pub player_levels: Vec<PlayerLevel>,
}

impl ResponseResult for MetagameConfig {}


/// This is a JSON template for a list of achievement definition objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list achievement definitions](struct.AchievementDefinitionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AchievementDefinitionsListResponse {
    /// Token corresponding to the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The achievement definitions.    
    pub items: Vec<AchievementDefinition>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#achievementDefinitionsListResponse.    
    pub kind: String,
}

impl ResponseResult for AchievementDefinitionsListResponse {}


/// This is a JSON template for peer session diagnostics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PeerSessionDiagnostics {
    /// Unreliable channel diagnostics.    
    #[serde(alias="unreliableChannel")]
    pub unreliable_channel: PeerChannelDiagnostics,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#peerSessionDiagnostics.    
    pub kind: String,
    /// Reliable channel diagnostics.    
    #[serde(alias="reliableChannel")]
    pub reliable_channel: PeerChannelDiagnostics,
    /// Connected time in milliseconds.    
    #[serde(alias="connectedTimestampMillis")]
    pub connected_timestamp_millis: String,
    /// The participant ID of the peer.    
    #[serde(alias="participantId")]
    pub participant_id: String,
}

impl Part for PeerSessionDiagnostics {}


/// This is a JSON template for sending a turn-based match data object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct TurnBasedMatchDataRequest {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchDataRequest.    
    pub kind: String,
    /// The byte representation of the data (limited to 128 kB), as a Base64-encoded string with the URL_SAFE encoding option.    
    pub data: String,
}

impl Part for TurnBasedMatchDataRequest {}


/// This is a JSON template for an event period time range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventPeriodRange {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#eventPeriodRange.    
    pub kind: String,
    /// The time when this update period begins, in millis, since 1970 UTC (Unix Epoch).    
    #[serde(alias="periodStartMillis")]
    pub period_start_millis: String,
    /// The time when this update period ends, in millis, since 1970 UTC (Unix Epoch).    
    #[serde(alias="periodEndMillis")]
    pub period_end_millis: String,
}

impl Part for EventPeriodRange {}


/// This is a JSON template for a result for a match participant.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParticipantResult {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#participantResult.    
    pub kind: String,
    /// The placement or ranking of the participant in the match results; a number from one to the number of participants in the match. Multiple participants may have the same placing value in case of a type.    
    pub placing: i32,
    /// The ID of the participant.    
    #[serde(alias="participantId")]
    pub participant_id: String,
    /// The result of the participant for this match.
    /// Possible values are:  
    /// - "MATCH_RESULT_WIN" - The participant won the match. 
    /// - "MATCH_RESULT_LOSS" - The participant lost the match. 
    /// - "MATCH_RESULT_TIE" - The participant tied the match. 
    /// - "MATCH_RESULT_NONE" - There was no winner for the match (nobody wins or loses this kind of game.) 
    /// - "MATCH_RESULT_DISCONNECT" - The participant disconnected / left during the match. 
    /// - "MATCH_RESULT_DISAGREED" - Different clients reported different results for this participant.
    pub result: String,
}

impl Part for ParticipantResult {}


/// This is a JSON template for network diagnostics reported for a client.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct NetworkDiagnostics {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#networkDiagnostics.    
    pub kind: String,
    /// The name of the carrier of the client's network connection. On Android: http://developer.android.com/reference/android/telephony/TelephonyManager.html#getNetworkOperatorName() On iOS: https://developer.apple.com/library/ios/documentation/NetworkingInternet/Reference/CTCarrier/Reference/Reference.html#//apple_ref/occ/instp/CTCarrier/carrierName    
    #[serde(alias="networkOperatorName")]
    pub network_operator_name: String,
    /// The amount of time in milliseconds it took for the client to establish a connection with the XMPP server.    
    #[serde(alias="registrationLatencyMillis")]
    pub registration_latency_millis: i32,
    /// iOS network type as defined in Reachability.h.    
    #[serde(alias="iosNetworkType")]
    pub ios_network_type: i32,
    /// The MCC+MNC code for the client's network connection. On Android: http://developer.android.com/reference/android/telephony/TelephonyManager.html#getNetworkOperator() On iOS, see: https://developer.apple.com/library/ios/documentation/NetworkingInternet/Reference/CTCarrier/Reference/Reference.html    
    #[serde(alias="networkOperatorCode")]
    pub network_operator_code: String,
    /// The Android network subtype.    
    #[serde(alias="androidNetworkSubtype")]
    pub android_network_subtype: i32,
    /// The Android network type.    
    #[serde(alias="androidNetworkType")]
    pub android_network_type: i32,
}

impl Part for NetworkDiagnostics {}


/// This is a JSON template for a push token ID resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [remove pushtokens](struct.PushtokenRemoveCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PushTokenId {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#pushTokenId.    
    pub kind: Option<String>,
    /// A push token ID for iOS devices.    
    pub ios: Option<PushTokenIdIos>,
}

impl RequestValue for PushTokenId {}


/// This is a JSON template for a batch update failure resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct EventBatchRecordFailure {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#eventBatchRecordFailure.    
    pub kind: String,
    /// The time range which was rejected; empty for a request-wide failure.    
    pub range: EventPeriodRange,
    /// The cause for the update failure.
    /// Possible values are:  
    /// - "TOO_LARGE": A batch request was issued with more events than are allowed in a single batch. 
    /// - "TIME_PERIOD_EXPIRED": A batch was sent with data too far in the past to record. 
    /// - "TIME_PERIOD_SHORT": A batch was sent with a time range that was too short. 
    /// - "TIME_PERIOD_LONG": A batch was sent with a time range that was too long. 
    /// - "ALREADY_UPDATED": An attempt was made to record a batch of data which was already seen. 
    /// - "RECORD_RATE_HIGH": An attempt was made to record data faster than the server will apply updates.
    #[serde(alias="failureCause")]
    pub failure_cause: String,
}

impl Part for EventBatchRecordFailure {}


/// This is a JSON template for the client address when setting up a room.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoomClientAddress {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomClientAddress.    
    pub kind: String,
    /// The XMPP address of the client on the Google Games XMPP network.    
    #[serde(alias="xmppAddress")]
    pub xmpp_address: String,
}

impl Part for RoomClientAddress {}


/// This is a JSON template for 1P/3P metadata about the player's experience.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerExperienceInfo {
    /// The current number of experience points for the player.    
    #[serde(alias="currentExperiencePoints")]
    pub current_experience_points: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerExperienceInfo.    
    pub kind: String,
    /// The timestamp when the player was leveled up, in millis since Unix epoch UTC.    
    #[serde(alias="lastLevelUpTimestampMillis")]
    pub last_level_up_timestamp_millis: String,
    /// The next level of the player. If the current level is the maximum level, this should be same as the current level.    
    #[serde(alias="nextLevel")]
    pub next_level: PlayerLevel,
    /// The current level of the player.    
    #[serde(alias="currentLevel")]
    pub current_level: PlayerLevel,
}

impl Part for PlayerExperienceInfo {}


/// This is a JSON template for a ListScores response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list window scores](struct.ScoreListWindowCall.html) (response)
/// * [list scores](struct.ScoreListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct LeaderboardScores {
    /// The pagination token for the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#leaderboardScores.    
    pub kind: String,
    /// The pagination token for the previous page of results.    
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// The total number of scores in the leaderboard.    
    #[serde(alias="numScores")]
    pub num_scores: String,
    /// The scores in the leaderboard.    
    pub items: Vec<LeaderboardEntry>,
    /// The score of the requesting player on the leaderboard. The player's score may appear both here and in the list of scores above. If you are viewing a public leaderboard and the player is not sharing their gameplay information publicly, the scoreRank and formattedScoreRank values will not be present.    
    #[serde(alias="playerScore")]
    pub player_score: LeaderboardEntry,
}

impl ResponseResult for LeaderboardScores {}


/// This is a JSON template for a participant in a room.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RoomParticipant {
    /// True if this participant was auto-matched with the requesting player.    
    #[serde(alias="autoMatched")]
    pub auto_matched: bool,
    /// The status of the participant with respect to the room.
    /// Possible values are:  
    /// - "PARTICIPANT_INVITED" - The participant has been invited to join the room, but has not yet responded. 
    /// - "PARTICIPANT_JOINED" - The participant has joined the room (either after creating it or accepting an invitation.) 
    /// - "PARTICIPANT_DECLINED" - The participant declined an invitation to join the room. 
    /// - "PARTICIPANT_LEFT" - The participant joined the room and then left it.
    pub status: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomParticipant.    
    pub kind: String,
    /// Information about a player that has been anonymously auto-matched against the requesting player. (Either player or autoMatchedPlayer will be set.)    
    #[serde(alias="autoMatchedPlayer")]
    pub auto_matched_player: AnonymousPlayer,
    /// Client address for the participant.    
    #[serde(alias="clientAddress")]
    pub client_address: RoomClientAddress,
    /// The capabilities which can be used when communicating with this participant.    
    pub capabilities: Vec<String>,
    /// Information about the player. Not populated if this player was anonymously auto-matched against the requesting player. (Either player or autoMatchedPlayer will be set.)    
    pub player: Player,
    /// The reason the participant left the room; populated if the participant status is PARTICIPANT_LEFT.
    /// Possible values are:  
    /// - "PLAYER_LEFT" - The player explicitly chose to leave the room. 
    /// - "GAME_LEFT" - The game chose to remove the player from the room. 
    /// - "ABANDONED" - The player switched to another application and abandoned the room.
    /// - "PEER_CONNECTION_FAILURE" - The client was unable to establish or maintain a connection to other peer(s) in the room.
    /// - "SERVER_ERROR" - The client received an error response when it tried to communicate with the server. 
    /// - "TIMEOUT" - The client timed out while waiting for players to join and connect. 
    /// - "PRESENCE_FAILURE" - The client's XMPP connection ended abruptly.
    #[serde(alias="leaveReason")]
    pub leave_reason: String,
    /// True if this participant is in the fully connected set of peers in the room.    
    pub connected: bool,
    /// An identifier for the participant in the scope of the room. Cannot be used to identify a player across rooms or in other contexts.    
    pub id: String,
}

impl Part for RoomParticipant {}


/// This is a JSON template for an snapshot object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list snapshots](struct.SnapshotListCall.html) (none)
/// * [get snapshots](struct.SnapshotGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Snapshot {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#snapshot.    
    pub kind: String,
    /// The description of this snapshot.    
    pub description: String,
    /// The title of this snapshot.    
    pub title: String,
    /// The cover image of this snapshot. May be absent if there is no image.    
    #[serde(alias="coverImage")]
    pub cover_image: SnapshotImage,
    /// The timestamp (in millis since Unix epoch) of the last modification to this snapshot.    
    #[serde(alias="lastModifiedMillis")]
    pub last_modified_millis: String,
    /// The ID of the snapshot.    
    pub id: String,
    /// The ID of the file underlying this snapshot in the Drive API. Only present if the snapshot is a view on a Drive file and the file is owned by the caller.    
    #[serde(alias="driveId")]
    pub drive_id: String,
    /// The duration associated with this snapshot, in millis.    
    #[serde(alias="durationMillis")]
    pub duration_millis: String,
    /// The unique name provided when the snapshot was created.    
    #[serde(alias="uniqueName")]
    pub unique_name: String,
    /// The type of this snapshot.
    /// Possible values are:  
    /// - "SAVE_GAME" - A snapshot representing a save game.
    #[serde(alias="type")]
    pub type_: String,
    /// The progress value (64-bit integer set by developer) associated with this snapshot.    
    #[serde(alias="progressValue")]
    pub progress_value: String,
}

impl Resource for Snapshot {}
impl ResponseResult for Snapshot {}


/// This is a JSON template for a leave room request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [leave rooms](struct.RoomLeaveCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct RoomLeaveRequest {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomLeaveRequest.    
    pub kind: Option<String>,
    /// Reason for leaving the match.
    /// Possible values are:  
    /// - "PLAYER_LEFT" - The player chose to leave the room.. 
    /// - "GAME_LEFT" - The game chose to remove the player from the room. 
    /// - "REALTIME_ABANDONED" - The player switched to another application and abandoned the room. 
    /// - "REALTIME_PEER_CONNECTION_FAILURE" - The client was unable to establish a connection to other peer(s). 
    /// - "REALTIME_SERVER_CONNECTION_FAILURE" - The client was unable to communicate with the server. 
    /// - "REALTIME_SERVER_ERROR" - The client received an error response when it tried to communicate with the server. 
    /// - "REALTIME_TIMEOUT" - The client timed out while waiting for a room. 
    /// - "REALTIME_CLIENT_DISCONNECTING" - The client disconnects without first calling Leave. 
    /// - "REALTIME_SIGN_OUT" - The user signed out of G+ while in the room. 
    /// - "REALTIME_GAME_CRASHED" - The game crashed. 
    /// - "REALTIME_ROOM_SERVICE_CRASHED" - RoomAndroidService crashed. 
    /// - "REALTIME_DIFFERENT_CLIENT_ROOM_OPERATION" - Another client is trying to enter a room. 
    /// - "REALTIME_SAME_CLIENT_ROOM_OPERATION" - The same client is trying to enter a new room.
    pub reason: Option<String>,
    /// Diagnostics for a player leaving the room.    
    #[serde(alias="leaveDiagnostics")]
    pub leave_diagnostics: Option<RoomLeaveDiagnostics>,
}

impl RequestValue for RoomLeaveRequest {}


/// This is a JSON template for a list of score submission statuses.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [submit multiple scores](struct.ScoreSubmitMultipleCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerScoreListResponse {
    /// The score submissions statuses.    
    #[serde(alias="submittedScores")]
    pub submitted_scores: Vec<PlayerScoreResponse>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerScoreListResponse.    
    pub kind: String,
}

impl ResponseResult for PlayerScoreListResponse {}


/// This is a JSON template for room leave diagnostics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct RoomLeaveDiagnostics {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomLeaveDiagnostics.    
    pub kind: String,
    /// Whether or not sockets were used.    
    #[serde(alias="socketsUsed")]
    pub sockets_used: bool,
    /// iOS network type as defined in Reachability.h.    
    #[serde(alias="iosNetworkType")]
    pub ios_network_type: i32,
    /// The MCC+MNC code for the client's network connection. On Android: http://developer.android.com/reference/android/telephony/TelephonyManager.html#getNetworkOperator() On iOS, see: https://developer.apple.com/library/ios/documentation/NetworkingInternet/Reference/CTCarrier/Reference/Reference.html    
    #[serde(alias="networkOperatorCode")]
    pub network_operator_code: String,
    /// Diagnostics about all peer sessions.    
    #[serde(alias="peerSession")]
    pub peer_session: Vec<PeerSessionDiagnostics>,
    /// Android network subtype. http://developer.android.com/reference/android/net/NetworkInfo.html#getSubtype()    
    #[serde(alias="androidNetworkSubtype")]
    pub android_network_subtype: i32,
    /// The name of the carrier of the client's network connection. On Android: http://developer.android.com/reference/android/telephony/TelephonyManager.html#getNetworkOperatorName() On iOS: https://developer.apple.com/library/ios/documentation/NetworkingInternet/Reference/CTCarrier/Reference/Reference.html#//apple_ref/occ/instp/CTCarrier/carrierName    
    #[serde(alias="networkOperatorName")]
    pub network_operator_name: String,
    /// Android network type. http://developer.android.com/reference/android/net/NetworkInfo.html#getType()    
    #[serde(alias="androidNetworkType")]
    pub android_network_type: i32,
}

impl Part for RoomLeaveDiagnostics {}


/// This is a JSON template for a Quest Milestone resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [claim quest milestones](struct.QuestMilestoneClaimCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct QuestMilestone {
    /// The current state of the milestone.
    /// Possible values are:  
    /// - "COMPLETED_NOT_CLAIMED" - The milestone is complete, but has not yet been claimed. 
    /// - "CLAIMED" - The milestone is complete and has been claimed. 
    /// - "NOT_COMPLETED" - The milestone has not yet been completed. 
    /// - "NOT_STARTED" - The milestone is for a quest that has not yet been accepted.
    pub state: Option<String>,
    /// The completion reward data of the milestone, represented as a Base64-encoded string. This is a developer-specified binary blob with size between 0 and 2 KB before encoding.    
    #[serde(alias="completionRewardData")]
    pub completion_reward_data: Option<String>,
    /// The milestone ID.    
    pub id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#questMilestone.    
    pub kind: Option<String>,
    /// The criteria of the milestone.    
    pub criteria: Option<Vec<QuestCriterion>>,
}

impl Resource for QuestMilestone {}


/// This is a JSON template for a rematch response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rematch turn based matches](struct.TurnBasedMatcheRematchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct TurnBasedMatchRematch {
    /// The newly created match; a rematch of the old match with the same participants.    
    pub rematch: TurnBasedMatch,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchRematch.    
    pub kind: String,
    /// The old match that the rematch was created from; will be updated such that the rematchId field will point at the new match.    
    #[serde(alias="previousMatch")]
    pub previous_match: TurnBasedMatch,
}

impl ResponseResult for TurnBasedMatchRematch {}


/// This is a JSON template for a list of player leaderboard scores.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get scores](struct.ScoreGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerLeaderboardScoreListResponse {
    /// The pagination token for the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The leaderboard scores.    
    pub items: Vec<PlayerLeaderboardScore>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerLeaderboardScoreListResponse.    
    pub kind: String,
    /// The Player resources for the owner of this score.    
    pub player: Player,
}

impl ResponseResult for PlayerLeaderboardScoreListResponse {}


/// This is a JSON template for the Application resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get applications](struct.ApplicationGetCall.html) (response)
/// * [played applications](struct.ApplicationPlayedCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Application {
    /// The category of the application.    
    pub category: ApplicationCategory,
    /// A hint to the client UI for what color to use as an app-themed color. The color is given as an RGB triplet (e.g. "E0E0E0").    
    #[serde(alias="themeColor")]
    pub theme_color: String,
    /// The description of the application.    
    pub description: String,
    /// The author of the application.    
    pub author: String,
    /// The last updated timestamp of the application.    
    #[serde(alias="lastUpdatedTimestamp")]
    pub last_updated_timestamp: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#application.    
    pub kind: String,
    /// The instances of the application.    
    pub instances: Vec<Instance>,
    /// A list of features that have been enabled for the application.
    /// Possible values are:  
    /// - "SNAPSHOTS" - Snapshots has been enabled
    #[serde(alias="enabledFeatures")]
    pub enabled_features: Vec<String>,
    /// The number of achievements visible to the currently authenticated player.    
    pub achievement_count: i32,
    /// The number of leaderboards visible to the currently authenticated player.    
    pub leaderboard_count: i32,
    /// The assets of the application.    
    pub assets: Vec<ImageAsset>,
    /// The ID of the application.    
    pub id: String,
    /// The name of the application.    
    pub name: String,
}

impl Resource for Application {}
impl ResponseResult for Application {}


/// This is a JSON template for metadata about a player playing a game with the currently authenticated user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Played {
    /// True if the player was auto-matched with the currently authenticated user.    
    #[serde(alias="autoMatched")]
    pub auto_matched: bool,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#played.    
    pub kind: String,
    /// The last time the player played the game in milliseconds since the epoch in UTC.    
    #[serde(alias="timeMillis")]
    pub time_millis: String,
}

impl Part for Played {}


/// This is a JSON template for a Quest Criterion Contribution resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct QuestContribution {
    /// The formatted value of the contribution as a string. Format depends on the configuration for the associated event definition in the Play Games Developer Console.    
    #[serde(alias="formattedValue")]
    pub formatted_value: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#questContribution.    
    pub kind: String,
    /// The value of the contribution.    
    pub value: String,
}

impl Part for QuestContribution {}


/// This is a JSON template for a Quest Criterion resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct QuestCriterion {
    /// The ID of the event the criterion corresponds to.    
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The total number of times the associated event must be incremented for the player to complete this quest.    
    #[serde(alias="completionContribution")]
    pub completion_contribution: QuestContribution,
    /// The value of the event associated with this quest at the time that the quest was accepted. This value may change if event increments that took place before the start of quest are uploaded after the quest starts.
    /// There will be no initialPlayerProgress until the player has accepted the quest.
    #[serde(alias="initialPlayerProgress")]
    pub initial_player_progress: QuestContribution,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#questCriterion.    
    pub kind: String,
    /// The number of increments the player has made toward the completion count event increments required to complete the quest. This value will not exceed the completion contribution.
    /// There will be no currentContribution until the player has accepted the quest.
    #[serde(alias="currentContribution")]
    pub current_contribution: QuestContribution,
}

impl Part for QuestCriterion {}


/// This is a JSON template for a list of rooms.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list rooms](struct.RoomListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RoomList {
    /// The pagination token for the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The rooms.    
    pub items: Vec<Room>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomList.    
    pub kind: String,
}

impl ResponseResult for RoomList {}


/// This is a JSON template for 1P/3P metadata about a user's level.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerLevel {
    /// The maximum experience points for this level.    
    #[serde(alias="maxExperiencePoints")]
    pub max_experience_points: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerLevel.    
    pub kind: String,
    /// The minimum experience points for this level.    
    #[serde(alias="minExperiencePoints")]
    pub min_experience_points: String,
    /// The level for the user.    
    pub level: i32,
}

impl Part for PlayerLevel {}


/// This is a JSON template for status of room automatching that is in progress.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RoomAutoMatchStatus {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomAutoMatchStatus.    
    pub kind: String,
    /// An estimate for the amount of time (in seconds) that auto-matching is expected to take to complete.    
    #[serde(alias="waitEstimateSeconds")]
    pub wait_estimate_seconds: i32,
}

impl Part for RoomAutoMatchStatus {}


/// This is a JSON template for a turn-based match data object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct TurnBasedMatchData {
    /// The byte representation of the data (limited to 128 kB), as a Base64-encoded string with the URL_SAFE encoding option.    
    pub data: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchData.    
    pub kind: String,
    /// True if this match has data available but it wasn't returned in a list response; fetching the match individually will retrieve this data.    
    #[serde(alias="dataAvailable")]
    pub data_available: bool,
}

impl Part for TurnBasedMatchData {}


/// This is a JSON template for an event definition resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct EventDefinition {
    /// Indicates whether the icon image being returned is a default image, or is game-provided.    
    #[serde(alias="isDefaultImageUrl")]
    pub is_default_image_url: bool,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#eventDefinition.    
    pub kind: String,
    /// The name to display for the event.    
    #[serde(alias="displayName")]
    pub display_name: String,
    /// Description of what this event represents.    
    pub description: String,
    /// The base URL for the image that represents the event.    
    #[serde(alias="imageUrl")]
    pub image_url: String,
    /// The visibility of event being tracked in this definition.
    /// Possible values are:  
    /// - "REVEALED": This event should be visible to all users. 
    /// - "HIDDEN": This event should only be shown to users that have recorded this event at least once.
    pub visibility: String,
    /// A list of events that are a child of this event.    
    #[serde(alias="childEvents")]
    pub child_events: Vec<EventChild>,
    /// The ID of the event.    
    pub id: String,
}

impl Part for EventDefinition {}


/// This is a JSON template for the iOS details resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct InstanceIosDetails {
    /// Indicates that this instance is the default for new installations on iPhone devices.    
    #[serde(alias="preferredForIphone")]
    pub preferred_for_iphone: bool,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#instanceIosDetails.    
    pub kind: String,
    /// Flag to indicate if this instance supports iPhone.    
    #[serde(alias="supportIphone")]
    pub support_iphone: bool,
    /// Indicates that this instance is the default for new installations on iPad devices.    
    #[serde(alias="preferredForIpad")]
    pub preferred_for_ipad: bool,
    /// iTunes App ID.    
    #[serde(alias="itunesAppId")]
    pub itunes_app_id: String,
    /// Bundle identifier.    
    #[serde(alias="bundleIdentifier")]
    pub bundle_identifier: String,
    /// Flag to indicate if this instance supports iPad.    
    #[serde(alias="supportIpad")]
    pub support_ipad: bool,
}

impl Part for InstanceIosDetails {}


/// This is a JSON template for the Android instance details resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct InstanceAndroidDetails {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#instanceAndroidDetails.    
    pub kind: String,
    /// Flag indicating whether the anti-piracy check is enabled.    
    #[serde(alias="enablePiracyCheck")]
    pub enable_piracy_check: bool,
    /// Android package name which maps to Google Play URL.    
    #[serde(alias="packageName")]
    pub package_name: String,
    /// Indicates that this instance is the default for new installations.    
    pub preferred: bool,
}

impl Part for InstanceAndroidDetails {}


/// This is a JSON template for an achievement reveal response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reveal achievements](struct.AchievementRevealCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AchievementRevealResponse {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#achievementRevealResponse.    
    pub kind: String,
    /// The current state of the achievement for which a reveal was attempted. This might be UNLOCKED if the achievement was already unlocked.
    /// Possible values are:  
    /// - "REVEALED" - Achievement is revealed. 
    /// - "UNLOCKED" - Achievement is unlocked.
    #[serde(alias="currentState")]
    pub current_state: String,
}

impl ResponseResult for AchievementRevealResponse {}


/// This is a JSON template for a room resource object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [decline rooms](struct.RoomDeclineCall.html) (response)
/// * [leave rooms](struct.RoomLeaveCall.html) (response)
/// * [dismiss rooms](struct.RoomDismisCall.html) (none)
/// * [list rooms](struct.RoomListCall.html) (none)
/// * [create rooms](struct.RoomCreateCall.html) (response)
/// * [get rooms](struct.RoomGetCall.html) (response)
/// * [join rooms](struct.RoomJoinCall.html) (response)
/// * [report status rooms](struct.RoomReportStatuCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Room {
    /// The status of the room.
    /// Possible values are:  
    /// - "ROOM_INVITING" - One or more players have been invited and not responded. 
    /// - "ROOM_AUTO_MATCHING" - One or more slots need to be filled by auto-matching. 
    /// - "ROOM_CONNECTING" - Players have joined and are connecting to each other (either before or after auto-matching). 
    /// - "ROOM_ACTIVE" - All players have joined and connected to each other. 
    /// - "ROOM_DELETED" - The room should no longer be shown on the client. Returned in sync calls when a player joins a room (as a tombstone), or for rooms where all joined participants have left.
    pub status: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#room.    
    pub kind: String,
    /// Criteria for auto-matching players into this room.    
    #[serde(alias="autoMatchingCriteria")]
    pub auto_matching_criteria: RoomAutoMatchingCriteria,
    /// Details about the room creation.    
    #[serde(alias="creationDetails")]
    pub creation_details: RoomModification,
    /// This short description is generated by our servers and worded relative to the player requesting the room. It is intended to be displayed when the room is shown in a list (that is, an invitation to a room.)    
    pub description: String,
    /// The version of the room status: an increasing counter, used by the client to ignore out-of-order updates to room status.    
    #[serde(alias="roomStatusVersion")]
    pub room_status_version: i32,
    /// Auto-matching status for this room. Not set if the room is not currently in the auto-matching queue.    
    #[serde(alias="autoMatchingStatus")]
    pub auto_matching_status: RoomAutoMatchStatus,
    /// Details about the last update to the room.    
    #[serde(alias="lastUpdateDetails")]
    pub last_update_details: RoomModification,
    /// The variant / mode of the application being played; can be any integer value, or left blank.    
    pub variant: i32,
    /// The participants involved in the room, along with their statuses. Includes participants who have left or declined invitations.    
    pub participants: Vec<RoomParticipant>,
    /// Globally unique ID for a room.    
    #[serde(alias="roomId")]
    pub room_id: String,
    /// The ID of the application being played.    
    #[serde(alias="applicationId")]
    pub application_id: String,
    /// The ID of the participant that invited the user to the room. Not set if the user was not invited to the room.    
    #[serde(alias="inviterId")]
    pub inviter_id: String,
}

impl Resource for Room {}
impl ResponseResult for Room {}


/// This is a JSON template for the Leaderboard Entry resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct LeaderboardEntry {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#leaderboardEntry.    
    pub kind: String,
    /// The numerical value of this score.    
    #[serde(alias="scoreValue")]
    pub score_value: String,
    /// Additional information about the score. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.    
    #[serde(alias="scoreTag")]
    pub score_tag: String,
    /// The time span of this high score.
    /// Possible values are:  
    /// - "ALL_TIME" - The score is an all-time high score. 
    /// - "WEEKLY" - The score is a weekly high score. 
    /// - "DAILY" - The score is a daily high score.
    #[serde(alias="timeSpan")]
    pub time_span: String,
    /// The localized string for the numerical value of this score.    
    #[serde(alias="formattedScore")]
    pub formatted_score: String,
    /// The player who holds this score.    
    pub player: Player,
    /// The localized string for the rank of this score for this leaderboard.    
    #[serde(alias="formattedScoreRank")]
    pub formatted_score_rank: String,
    /// The rank of this score for this leaderboard.    
    #[serde(alias="scoreRank")]
    pub score_rank: String,
    /// The timestamp at which this score was recorded, in milliseconds since the epoch in UTC.    
    #[serde(alias="writeTimestampMillis")]
    pub write_timestamp_millis: String,
}

impl Part for LeaderboardEntry {}


/// This is a JSON template for an turn-based auto-match criteria object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TurnBasedAutoMatchingCriteria {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedAutoMatchingCriteria.    
    pub kind: String,
    /// The minimum number of players that should be added to the match by auto-matching.    
    #[serde(alias="minAutoMatchingPlayers")]
    pub min_auto_matching_players: i32,
    /// A bitmask indicating when auto-matches are valid. When ANDed with other exclusive bitmasks, the result must be zero. Can be used to support exclusive roles within a game.    
    #[serde(alias="exclusiveBitmask")]
    pub exclusive_bitmask: String,
    /// The maximum number of players that should be added to the match by auto-matching.    
    #[serde(alias="maxAutoMatchingPlayers")]
    pub max_auto_matching_players: i32,
}

impl Part for TurnBasedAutoMatchingCriteria {}


/// This is a JSON template for a room creation request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create rooms](struct.RoomCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct RoomCreateRequest {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomCreateRequest.    
    pub kind: Option<String>,
    /// Criteria for auto-matching players into this room.    
    #[serde(alias="autoMatchingCriteria")]
    pub auto_matching_criteria: Option<RoomAutoMatchingCriteria>,
    /// The player IDs to invite to the room.    
    #[serde(alias="invitedPlayerIds")]
    pub invited_player_ids: Option<Vec<String>>,
    /// The variant / mode of the application to be played. This can be any integer value, or left blank. You should use a small number of variants to keep the auto-matching pool as large as possible.    
    pub variant: Option<i32>,
    /// The capabilities that this client supports for realtime communication.    
    pub capabilities: Option<Vec<String>>,
    /// Network diagnostics for the client creating the room.    
    #[serde(alias="networkDiagnostics")]
    pub network_diagnostics: Option<NetworkDiagnostics>,
    /// Client address for the player creating the room.    
    #[serde(alias="clientAddress")]
    pub client_address: Option<RoomClientAddress>,
    /// A randomly generated numeric ID. This number is used at the server to ensure that the request is handled correctly across retries.    
    #[serde(alias="requestId")]
    pub request_id: Option<String>,
}

impl RequestValue for RoomCreateRequest {}


/// This is a JSON template for a list of quest objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list quests](struct.QuestListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct QuestListResponse {
    /// Token corresponding to the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The quests.    
    pub items: Vec<Quest>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#questListResponse.    
    pub kind: String,
}

impl ResponseResult for QuestListResponse {}


/// This is a JSON template for a ListByPlayer response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list by player events](struct.EventListByPlayerCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerEventListResponse {
    /// The pagination token for the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The player events.    
    pub items: Vec<PlayerEvent>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerEventListResponse.    
    pub kind: String,
}

impl ResponseResult for PlayerEventListResponse {}


/// This is a JSON template for the payload to request to increment an achievement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct GamesAchievementIncrement {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#GamesAchievementIncrement.    
    pub kind: String,
    /// The number of steps to be incremented.    
    pub steps: i32,
    /// The requestId associated with an increment to an achievement.    
    #[serde(alias="requestId")]
    pub request_id: String,
}

impl Part for GamesAchievementIncrement {}


/// This is a JSON template for turn-based match modification metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct TurnBasedMatchModification {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchModification.    
    pub kind: String,
    /// The timestamp at which they modified the match, in milliseconds since the epoch in UTC.    
    #[serde(alias="modifiedTimestampMillis")]
    pub modified_timestamp_millis: String,
    /// The ID of the participant that modified the match.    
    #[serde(alias="participantId")]
    pub participant_id: String,
}

impl Part for TurnBasedMatchModification {}


/// This is a JSON template for a list of achievement objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list achievements](struct.AchievementListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerAchievementListResponse {
    /// Token corresponding to the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The achievements.    
    pub items: Vec<PlayerAchievement>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerAchievementListResponse.    
    pub kind: String,
}

impl ResponseResult for PlayerAchievementListResponse {}


/// This is a JSON template for a push token resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update pushtokens](struct.PushtokenUpdateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PushToken {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#pushToken.    
    pub kind: Option<String>,
    /// The revision of the client SDK used by your application, in the same format that's used by revisions.check. Used to send backward compatible messages. Format: [PLATFORM_TYPE]:[VERSION_NUMBER]. Possible values of PLATFORM_TYPE are:  
    /// - IOS - Push token is for iOS
    #[serde(alias="clientRevision")]
    pub client_revision: Option<String>,
    /// Unique identifier for this push token.    
    pub id: Option<PushTokenId>,
    /// The preferred language for notifications that are sent using this token.    
    pub language: Option<String>,
}

impl RequestValue for PushToken {}
impl Resource for PushToken {}


/// This is a JSON template for a player leaderboard score object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerLeaderboardScore {
    /// The timestamp at which this score was recorded, in milliseconds since the epoch in UTC.    
    #[serde(alias="writeTimestamp")]
    pub write_timestamp: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerLeaderboardScore.    
    pub kind: String,
    /// The public rank of the score in this leaderboard. This object will not be present if the user is not sharing their scores publicly.    
    #[serde(alias="publicRank")]
    pub public_rank: LeaderboardScoreRank,
    /// Additional information about the score. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.    
    #[serde(alias="scoreTag")]
    pub score_tag: String,
    /// The time span of this score.
    /// Possible values are:  
    /// - "ALL_TIME" - The score is an all-time score. 
    /// - "WEEKLY" - The score is a weekly score. 
    /// - "DAILY" - The score is a daily score.
    #[serde(alias="timeSpan")]
    pub time_span: String,
    /// The formatted value of this score.    
    #[serde(alias="scoreString")]
    pub score_string: String,
    /// The ID of the leaderboard this score is in.    
    pub leaderboard_id: String,
    /// The numerical value of this score.    
    #[serde(alias="scoreValue")]
    pub score_value: String,
    /// The social rank of the score in this leaderboard.    
    #[serde(alias="socialRank")]
    pub social_rank: LeaderboardScoreRank,
}

impl Part for PlayerLeaderboardScore {}


/// This is a JSON template for a list of score submission requests
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [submit multiple scores](struct.ScoreSubmitMultipleCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PlayerScoreSubmissionList {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerScoreSubmissionList.    
    pub kind: Option<String>,
    /// The score submissions.    
    pub scores: Option<Vec<ScoreSubmission>>,
}

impl RequestValue for PlayerScoreSubmissionList {}


/// This is a JSON template for an image of a snapshot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SnapshotImage {
    /// The URL of the image. This URL may be invalidated at any time and should not be cached.    
    pub url: String,
    /// The width of the image.    
    pub width: i32,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#snapshotImage.    
    pub kind: String,
    /// The MIME type of the image.    
    pub mime_type: String,
    /// The height of the image.    
    pub height: i32,
}

impl Part for SnapshotImage {}


/// This is a JSON template for an anonymous player
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct AnonymousPlayer {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#anonymousPlayer.    
    pub kind: String,
    /// The base URL for the image to display for the anonymous player.    
    #[serde(alias="avatarImageUrl")]
    pub avatar_image_url: String,
    /// The name to display for the anonymous player.    
    #[serde(alias="displayName")]
    pub display_name: String,
}

impl Part for AnonymousPlayer {}


/// This is a JSON template for peer channel diagnostics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PeerChannelDiagnostics {
    /// Number of bytes received.    
    #[serde(alias="bytesReceived")]
    pub bytes_received: AggregateStats,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#peerChannelDiagnostics.    
    pub kind: String,
    /// Number of bytes sent.    
    #[serde(alias="bytesSent")]
    pub bytes_sent: AggregateStats,
    /// Number of send failures.    
    #[serde(alias="numSendFailures")]
    pub num_send_failures: i32,
    /// Number of messages lost.    
    #[serde(alias="numMessagesLost")]
    pub num_messages_lost: i32,
    /// Number of messages received.    
    #[serde(alias="numMessagesReceived")]
    pub num_messages_received: i32,
    /// Number of messages sent.    
    #[serde(alias="numMessagesSent")]
    pub num_messages_sent: i32,
    /// Roundtrip latency stats in milliseconds.    
    #[serde(alias="roundtripLatencyMillis")]
    pub roundtrip_latency_millis: AggregateStats,
}

impl Part for PeerChannelDiagnostics {}


/// This is a JSON template for a join room request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [join rooms](struct.RoomJoinCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct RoomJoinRequest {
    /// Network diagnostics for the client joining the room.    
    #[serde(alias="networkDiagnostics")]
    pub network_diagnostics: Option<NetworkDiagnostics>,
    /// Client address for the player joining the room.    
    #[serde(alias="clientAddress")]
    pub client_address: Option<RoomClientAddress>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomJoinRequest.    
    pub kind: Option<String>,
    /// The capabilities that this client supports for realtime communication.    
    pub capabilities: Option<Vec<String>>,
}

impl RequestValue for RoomJoinRequest {}


/// This is a JSON template for the status of a room that the player has joined.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report status rooms](struct.RoomReportStatuCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RoomStatus {
    /// The status of the room.
    /// Possible values are:  
    /// - "ROOM_INVITING" - One or more players have been invited and not responded. 
    /// - "ROOM_AUTO_MATCHING" - One or more slots need to be filled by auto-matching. 
    /// - "ROOM_CONNECTING" - Players have joined are connecting to each other (either before or after auto-matching). 
    /// - "ROOM_ACTIVE" - All players have joined and connected to each other. 
    /// - "ROOM_DELETED" - All joined players have left.
    pub status: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomStatus.    
    pub kind: String,
    /// The version of the status for the room: an increasing counter, used by the client to ignore out-of-order updates to room status.    
    #[serde(alias="statusVersion")]
    pub status_version: i32,
    /// Globally unique ID for a room.    
    #[serde(alias="roomId")]
    pub room_id: String,
    /// Auto-matching status for this room. Not set if the room is not currently in the automatching queue.    
    #[serde(alias="autoMatchingStatus")]
    pub auto_matching_status: RoomAutoMatchStatus,
    /// The participants involved in the room, along with their statuses. Includes participants who have left or declined invitations.    
    pub participants: Vec<RoomParticipant>,
}

impl ResponseResult for RoomStatus {}


/// A push token ID for iOS devices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct PushTokenIdIos {
    /// Device token supplied by an iOS system call to register for remote notifications. Encode this field as web-safe base64.    
    pub apns_device_token: String,
    /// Indicates whether this token should be used for the production or sandbox APNS server.    
    pub apns_environment: String,
}

impl NestedType for PushTokenIdIos {}
impl Part for PushTokenIdIos {}


/// This is a JSON template for the Leaderboard resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list leaderboards](struct.LeaderboardListCall.html) (none)
/// * [get leaderboards](struct.LeaderboardGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Leaderboard {
    /// The icon for the leaderboard.    
    #[serde(alias="iconUrl")]
    pub icon_url: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#leaderboard.    
    pub kind: String,
    /// The name of the leaderboard.    
    pub name: String,
    /// The leaderboard ID.    
    pub id: String,
    /// Indicates whether the icon image being returned is a default image, or is game-provided.    
    #[serde(alias="isIconUrlDefault")]
    pub is_icon_url_default: bool,
    /// How scores are ordered.
    /// Possible values are:  
    /// - "LARGER_IS_BETTER" - Larger values are better; scores are sorted in descending order. 
    /// - "SMALLER_IS_BETTER" - Smaller values are better; scores are sorted in ascending order.
    pub order: String,
}

impl Resource for Leaderboard {}
impl ResponseResult for Leaderboard {}


/// This is a JSON template for a participant in a turn-based match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct TurnBasedMatchParticipant {
    /// True if this participant was auto-matched with the requesting player.    
    #[serde(alias="autoMatched")]
    pub auto_matched: bool,
    /// The status of the participant with respect to the match.
    /// Possible values are:  
    /// - "PARTICIPANT_NOT_INVITED_YET" - The participant is slated to be invited to the match, but the invitation has not been sent; the invite will be sent when it becomes their turn. 
    /// - "PARTICIPANT_INVITED" - The participant has been invited to join the match, but has not yet responded. 
    /// - "PARTICIPANT_JOINED" - The participant has joined the match (either after creating it or accepting an invitation.) 
    /// - "PARTICIPANT_DECLINED" - The participant declined an invitation to join the match. 
    /// - "PARTICIPANT_LEFT" - The participant joined the match and then left it. 
    /// - "PARTICIPANT_FINISHED" - The participant finished playing in the match. 
    /// - "PARTICIPANT_UNRESPONSIVE" - The participant did not take their turn in the allotted time.
    pub status: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchParticipant.    
    pub kind: String,
    /// Information about a player that has been anonymously auto-matched against the requesting player. (Either player or autoMatchedPlayer will be set.)    
    #[serde(alias="autoMatchedPlayer")]
    pub auto_matched_player: AnonymousPlayer,
    /// Information about the player. Not populated if this player was anonymously auto-matched against the requesting player. (Either player or autoMatchedPlayer will be set.)    
    pub player: Player,
    /// An identifier for the participant in the scope of the match. Cannot be used to identify a player across matches or in other contexts.    
    pub id: String,
}

impl Part for TurnBasedMatchParticipant {}


/// This is a JSON template for a score rank in a leaderboard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct LeaderboardScoreRank {
    /// The number of scores in the leaderboard.    
    #[serde(alias="numScores")]
    pub num_scores: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#leaderboardScoreRank.    
    pub kind: String,
    /// The rank in the leaderboard as a string.    
    #[serde(alias="formattedRank")]
    pub formatted_rank: String,
    /// The rank in the leaderboard.    
    pub rank: String,
    /// The number of scores in the leaderboard as a string.    
    #[serde(alias="formattedNumScores")]
    pub formatted_num_scores: String,
}

impl Part for LeaderboardScoreRank {}


/// An object representation of the individual components of the player's name. For some players, these fields may not be present.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerName {
    /// The given name of this player. In some places, this is known as the first name.    
    #[serde(alias="givenName")]
    pub given_name: String,
    /// The family name of this player. In some places, this is known as the last name.    
    #[serde(alias="familyName")]
    pub family_name: String,
}

impl NestedType for PlayerName {}
impl Part for PlayerName {}


/// This is a JSON template for an achievement object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerAchievement {
    /// The state of the achievement.
    /// Possible values are:  
    /// - "HIDDEN" - Achievement is hidden. 
    /// - "REVEALED" - Achievement is revealed. 
    /// - "UNLOCKED" - Achievement is unlocked.
    #[serde(alias="achievementState")]
    pub achievement_state: String,
    /// The current steps for an incremental achievement.    
    #[serde(alias="currentSteps")]
    pub current_steps: i32,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerAchievement.    
    pub kind: String,
    /// Experience points earned for the achievement. This field is absent for achievements that have not yet been unlocked and 0 for achievements that have been unlocked by testers but that are unpublished.    
    #[serde(alias="experiencePoints")]
    pub experience_points: String,
    /// The current steps for an incremental achievement as a string.    
    #[serde(alias="formattedCurrentStepsString")]
    pub formatted_current_steps_string: String,
    /// The timestamp of the last modification to this achievement's state.    
    #[serde(alias="lastUpdatedTimestamp")]
    pub last_updated_timestamp: String,
    /// The ID of the achievement.    
    pub id: String,
}

impl Part for PlayerAchievement {}


/// This is a JSON template for an update on the status of a peer in a room.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct RoomP2PStatus {
    /// The status of the peer in the room.
    /// Possible values are:  
    /// - "CONNECTION_ESTABLISHED" - The client established a P2P connection with the peer. 
    /// - "CONNECTION_FAILED" - The client failed to establish directed presence with the peer.
    pub status: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#roomP2PStatus.    
    pub kind: String,
    /// The ID of the participant.    
    #[serde(alias="participantId")]
    pub participant_id: String,
    /// The error code in event of a failure.
    /// Possible values are:  
    /// - "P2P_FAILED" - The client failed to establish a P2P connection with the peer. 
    /// - "PRESENCE_FAILED" - The client failed to register to receive P2P connections. 
    /// - "RELAY_SERVER_FAILED" - The client received an error when trying to use the relay server to establish a P2P connection with the peer.
    pub error: String,
    /// More detailed diagnostic message returned in event of a failure.    
    pub error_reason: String,
    /// The amount of time in milliseconds it took to send packets back and forth on the unreliable channel with this peer.    
    #[serde(alias="unreliableRoundtripLatencyMillis")]
    pub unreliable_roundtrip_latency_millis: i32,
    /// The amount of time in milliseconds it took to establish connections with this peer.    
    #[serde(alias="connectionSetupLatencyMillis")]
    pub connection_setup_latency_millis: i32,
}

impl Part for RoomP2PStatus {}


/// This is a JSON template for an event period update resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct EventPeriodUpdate {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#eventPeriodUpdate.    
    pub kind: String,
    /// The time period being covered by this update.    
    #[serde(alias="timePeriod")]
    pub time_period: EventPeriodRange,
    /// The updates being made for this time period.    
    pub updates: Vec<EventUpdateRequest>,
}

impl Part for EventPeriodUpdate {}


/// This is a JSON template for a list of achievement update requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update multiple achievements](struct.AchievementUpdateMultipleCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct AchievementUpdateMultipleRequest {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#achievementUpdateMultipleRequest.    
    pub kind: Option<String>,
    /// The individual achievement update requests.    
    pub updates: Option<Vec<AchievementUpdateRequest>>,
}

impl RequestValue for AchievementUpdateMultipleRequest {}


/// This is a JSON template for a list of snapshot objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list snapshots](struct.SnapshotListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SnapshotListResponse {
    /// Token corresponding to the next page of results. If there are no more results, the token is omitted.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The snapshots.    
    pub items: Vec<Snapshot>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#snapshotListResponse.    
    pub kind: String,
}

impl ResponseResult for SnapshotListResponse {}


/// This is a JSON template for an application category object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ApplicationCategory {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#applicationCategory.    
    pub kind: String,
    /// The primary category.    
    pub primary: String,
    /// The secondary category.    
    pub secondary: String,
}

impl Part for ApplicationCategory {}


/// This is a JSON template for a request to submit a score to leaderboards.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct ScoreSubmission {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#scoreSubmission.    
    pub kind: String,
    /// The leaderboard this score is being submitted to.    
    #[serde(alias="leaderboardId")]
    pub leaderboard_id: String,
    /// The new score being submitted.    
    pub score: String,
    /// Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.    
    #[serde(alias="scoreTag")]
    pub score_tag: String,
    /// Signature Values will contain URI-safe characters as defined by section 2.3 of RFC 3986.    
    pub signature: String,
}

impl Part for ScoreSubmission {}


/// This is a JSON template for a turn-based match results object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [finish turn based matches](struct.TurnBasedMatcheFinishCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct TurnBasedMatchResults {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchResults.    
    pub kind: Option<String>,
    /// The final match data.    
    pub data: Option<TurnBasedMatchDataRequest>,
    /// The version of the match being updated.    
    #[serde(alias="matchVersion")]
    pub match_version: Option<i32>,
    /// The match results for the participants in the match.    
    pub results: Option<Vec<ParticipantResult>>,
}

impl RequestValue for TurnBasedMatchResults {}


/// This is a JSON template for the object representing a turn.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [take turn turn based matches](struct.TurnBasedMatcheTakeTurnCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct TurnBasedMatchTurn {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatchTurn.    
    pub kind: Option<String>,
    /// The ID of the participant who should take their turn next. May be set to the current player's participant ID to update match state without changing the turn. If not set, the match will wait for other player(s) to join via automatching; this is only valid if automatch criteria is set on the match with remaining slots for automatched players.    
    #[serde(alias="pendingParticipantId")]
    pub pending_participant_id: Option<String>,
    /// The shared game state data after the turn is over.    
    pub data: Option<TurnBasedMatchDataRequest>,
    /// The version of this match: an increasing counter, used to avoid out-of-date updates to the match.    
    #[serde(alias="matchVersion")]
    pub match_version: Option<i32>,
    /// The match results for the participants in the match.    
    pub results: Option<Vec<ParticipantResult>>,
}

impl RequestValue for TurnBasedMatchTurn {}


/// This is a JSON template for a turn-based match resource object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [decline turn based matches](struct.TurnBasedMatcheDeclineCall.html) (response)
/// * [leave turn based matches](struct.TurnBasedMatcheLeaveCall.html) (response)
/// * [finish turn based matches](struct.TurnBasedMatcheFinishCall.html) (response)
/// * [take turn turn based matches](struct.TurnBasedMatcheTakeTurnCall.html) (response)
/// * [create turn based matches](struct.TurnBasedMatcheCreateCall.html) (response)
/// * [join turn based matches](struct.TurnBasedMatcheJoinCall.html) (response)
/// * [leave turn turn based matches](struct.TurnBasedMatcheLeaveTurnCall.html) (response)
/// * [get turn based matches](struct.TurnBasedMatcheGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct TurnBasedMatch {
    /// The status of the match.
    /// Possible values are:  
    /// - "MATCH_AUTO_MATCHING" - One or more slots need to be filled by auto-matching; the match cannot be established until they are filled. 
    /// - "MATCH_ACTIVE" - The match has started. 
    /// - "MATCH_COMPLETE" - The match has finished. 
    /// - "MATCH_CANCELED" - The match was canceled. 
    /// - "MATCH_EXPIRED" - The match expired due to inactivity. 
    /// - "MATCH_DELETED" - The match should no longer be shown on the client. Returned only for tombstones for matches when sync is called.
    pub status: String,
    /// Criteria for auto-matching players into this match.    
    #[serde(alias="autoMatchingCriteria")]
    pub auto_matching_criteria: TurnBasedAutoMatchingCriteria,
    /// The ID of the participant that invited the user to the match. Not set if the user was not invited to the match.    
    #[serde(alias="inviterId")]
    pub inviter_id: String,
    /// The version of this match: an increasing counter, used to avoid out-of-date updates to the match.    
    #[serde(alias="matchVersion")]
    pub match_version: i32,
    /// The variant / mode of the application being played; can be any integer value, or left blank.    
    pub variant: i32,
    /// Globally unique ID for a turn-based match.    
    #[serde(alias="matchId")]
    pub match_id: String,
    /// The ID of a rematch of this match. Only set for completed matches that have been rematched.    
    #[serde(alias="rematchId")]
    pub rematch_id: String,
    /// The results reported for this match.    
    pub results: Vec<ParticipantResult>,
    /// The number of the match in a chain of rematches. Will be set to 1 for the first match and incremented by 1 for each rematch.    
    #[serde(alias="matchNumber")]
    pub match_number: i32,
    /// The data / game state for the previous match; set for the first turn of rematches only.    
    #[serde(alias="previousMatchData")]
    pub previous_match_data: TurnBasedMatchData,
    /// The ID of the application being played.    
    #[serde(alias="applicationId")]
    pub application_id: String,
    /// This short description is generated by our servers based on turn state and is localized and worded relative to the player requesting the match. It is intended to be displayed when the match is shown in a list.    
    pub description: String,
    /// The ID of another participant in the match that can be used when describing the participants the user is playing with.    
    #[serde(alias="withParticipantId")]
    pub with_participant_id: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#turnBasedMatch.    
    pub kind: String,
    /// Details about the match creation.    
    #[serde(alias="creationDetails")]
    pub creation_details: TurnBasedMatchModification,
    /// The status of the current user in the match. Derived from the match type, match status, the user's participant status, and the pending participant for the match.
    /// Possible values are:  
    /// - "USER_INVITED" - The user has been invited to join the match and has not responded yet. 
    /// - "USER_AWAITING_TURN" - The user is waiting for their turn. 
    /// - "USER_TURN" - The user has an action to take in the match. 
    /// - "USER_MATCH_COMPLETED" - The match has ended (it is completed, canceled, or expired.)
    #[serde(alias="userMatchStatus")]
    pub user_match_status: String,
    /// The data / game state for this match.    
    pub data: TurnBasedMatchData,
    /// The participants involved in the match, along with their statuses. Includes participants who have left or declined invitations.    
    pub participants: Vec<TurnBasedMatchParticipant>,
    /// The ID of the participant that is taking a turn.    
    #[serde(alias="pendingParticipantId")]
    pub pending_participant_id: String,
    /// Details about the last update to the match.    
    #[serde(alias="lastUpdateDetails")]
    pub last_update_details: TurnBasedMatchModification,
}

impl ResponseResult for TurnBasedMatch {}


/// This is a JSON template for an event period update resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [record events](struct.EventRecordCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct EventRecordRequest {
    /// A list of the time period updates being made in this request.    
    #[serde(alias="timePeriods")]
    pub time_periods: Option<Vec<EventPeriodUpdate>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#eventRecordRequest.    
    pub kind: Option<String>,
    /// The request ID used to identify this attempt to record events.    
    #[serde(alias="requestId")]
    pub request_id: Option<String>,
    /// The current time when this update was sent, in milliseconds, since 1970 UTC (Unix Epoch).    
    #[serde(alias="currentTimeMillis")]
    pub current_time_millis: Option<String>,
}

impl RequestValue for EventRecordRequest {}


/// This is a JSON template for the Web details resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct InstanceWebDetails {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#instanceWebDetails.    
    pub kind: String,
    /// Launch URL for the game.    
    #[serde(alias="launchUrl")]
    pub launch_url: String,
    /// Indicates that this instance is the default for new installations.    
    pub preferred: bool,
}

impl Part for InstanceWebDetails {}


/// This is a JSON template for a list of category data objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list categories by player metagame](struct.MetagameListCategoriesByPlayerCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct CategoryListResponse {
    /// Token corresponding to the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The list of categories with usage data.    
    pub items: Vec<Category>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#categoryListResponse.    
    pub kind: String,
}

impl ResponseResult for CategoryListResponse {}


/// This is a JSON template for an event status resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlayerEvent {
    /// The ID of the event definition.    
    #[serde(alias="definitionId")]
    pub definition_id: String,
    /// The ID of the player.    
    #[serde(alias="playerId")]
    pub player_id: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#playerEvent.    
    pub kind: String,
    /// The current number of times this event has occurred.    
    #[serde(alias="numEvents")]
    pub num_events: String,
    /// The current number of times this event has occurred, as a string. The formatting of this string depends on the configuration of your event in the Play Games Developer Console.    
    #[serde(alias="formattedNumEvents")]
    pub formatted_num_events: String,
}

impl Part for PlayerEvent {}


/// This is a JSON template for a Player resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list players](struct.PlayerListCall.html) (none)
/// * [get players](struct.PlayerGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct Player {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#player.    
    pub kind: String,
    /// The name to display for the player.    
    #[serde(alias="displayName")]
    pub display_name: String,
    /// An object representation of the individual components of the player's name. For some players, these fields may not be present.    
    pub name: PlayerName,
    /// The player's title rewarded for their game activities.    
    pub title: String,
    /// The ID of the player.    
    #[serde(alias="playerId")]
    pub player_id: String,
    /// Details about the last time this player played a multiplayer game with the currently authenticated player. Populated for PLAYED_WITH player collection members.    
    #[serde(alias="lastPlayedWith")]
    pub last_played_with: Played,
    /// An object to represent Play Game experience information for the player.    
    #[serde(alias="experienceInfo")]
    pub experience_info: PlayerExperienceInfo,
    /// The base URL for the image that represents the player.    
    #[serde(alias="avatarImageUrl")]
    pub avatar_image_url: String,
}

impl Resource for Player {}
impl ResponseResult for Player {}


/// This is a JSON template for an event period update resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [record events](struct.EventRecordCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct EventUpdateResponse {
    /// The current status of any updated events    
    #[serde(alias="playerEvents")]
    pub player_events: Vec<PlayerEvent>,
    /// Any batch-wide failures which occurred applying updates.    
    #[serde(alias="batchFailures")]
    pub batch_failures: Vec<EventBatchRecordFailure>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#eventUpdateResponse.    
    pub kind: String,
    /// Any failures updating a particular event.    
    #[serde(alias="eventFailures")]
    pub event_failures: Vec<EventRecordFailure>,
}

impl ResponseResult for EventUpdateResponse {}


/// This is a JSON template for the result of checking a revision.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check revisions](struct.RevisionCheckCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct RevisionCheckResponse {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#revisionCheckResponse.    
    pub kind: String,
    /// The result of the revision check.
    /// Possible values are:  
    /// - "OK" - The revision being used is current. 
    /// - "DEPRECATED" - There is currently a newer version available, but the revision being used still works. 
    /// - "INVALID" - The revision being used is not supported in any released version.
    #[serde(alias="revisionStatus")]
    pub revision_status: String,
    /// The version of the API this client revision should use when calling API methods.    
    #[serde(alias="apiVersion")]
    pub api_version: String,
}

impl ResponseResult for RevisionCheckResponse {}


/// This is a JSON template for a request to update an achievement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct AchievementUpdateRequest {
    /// The achievement this update is being applied to.    
    #[serde(alias="achievementId")]
    pub achievement_id: String,
    /// The type of update being applied.
    /// Possible values are:  
    /// - "REVEAL" - Achievement is revealed. 
    /// - "UNLOCK" - Achievement is unlocked. 
    /// - "INCREMENT" - Achievement is incremented. 
    /// - "SET_STEPS_AT_LEAST" - Achievement progress is set to at least the passed value.
    #[serde(alias="updateType")]
    pub update_type: String,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#achievementUpdateRequest.    
    pub kind: String,
    /// The payload if an update of type SET_STEPS_AT_LEAST was requested for the achievement.    
    #[serde(alias="setStepsAtLeastPayload")]
    pub set_steps_at_least_payload: GamesAchievementSetStepsAtLeast,
    /// The payload if an update of type INCREMENT was requested for the achievement.    
    #[serde(alias="incrementPayload")]
    pub increment_payload: GamesAchievementIncrement,
}

impl Part for AchievementUpdateRequest {}


/// This is a JSON template for a list of leaderboard objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list leaderboards](struct.LeaderboardListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct LeaderboardListResponse {
    /// Token corresponding to the next page of results.    
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// The leaderboards.    
    pub items: Vec<Leaderboard>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#leaderboardListResponse.    
    pub kind: String,
}

impl ResponseResult for LeaderboardListResponse {}


/// This is a JSON template for the payload to request to increment an achievement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct GamesAchievementSetStepsAtLeast {
    /// Uniquely identifies the type of this resource. Value is always the fixed string games#GamesAchievementSetStepsAtLeast.    
    pub kind: String,
    /// The minimum number of steps for the achievement to be set to.    
    pub steps: i32,
}

impl Part for GamesAchievementSetStepsAtLeast {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *achievement* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `increment(...)`, `list(...)`, `reveal(...)`, `set_steps_at_least(...)`, `unlock(...)` and `update_multiple(...)`
/// // to build up your call.
/// let rb = hub.achievements();
/// # }
/// ```
pub struct AchievementMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for AchievementMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> AchievementMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the progress for all your application's achievements for the currently authenticated player.    
    pub fn list(&self, player_id: &str) -> AchievementListCall<'a, C, NC, A> {
        AchievementListCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _state: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Increments the steps of the achievement with the given ID for the currently authenticated player.    
    pub fn increment(&self, achievement_id: &str, steps_to_increment: i32) -> AchievementIncrementCall<'a, C, NC, A> {
        AchievementIncrementCall {
            hub: self.hub,
            _achievement_id: achievement_id.to_string(),
            _steps_to_increment: steps_to_increment,
            _request_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the steps for the currently authenticated player towards unlocking an achievement. If the steps parameter is less than the current number of steps that the player already gained for the achievement, the achievement is not modified.    
    pub fn set_steps_at_least(&self, achievement_id: &str, steps: i32) -> AchievementSetStepsAtLeastCall<'a, C, NC, A> {
        AchievementSetStepsAtLeastCall {
            hub: self.hub,
            _achievement_id: achievement_id.to_string(),
            _steps: steps,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates multiple achievements for the currently authenticated player.    
    pub fn update_multiple(&self, request: &AchievementUpdateMultipleRequest) -> AchievementUpdateMultipleCall<'a, C, NC, A> {
        AchievementUpdateMultipleCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the state of the achievement with the given ID to REVEALED for the currently authenticated player.    
    pub fn reveal(&self, achievement_id: &str) -> AchievementRevealCall<'a, C, NC, A> {
        AchievementRevealCall {
            hub: self.hub,
            _achievement_id: achievement_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unlocks this achievement for the currently authenticated player.    
    pub fn unlock(&self, achievement_id: &str) -> AchievementUnlockCall<'a, C, NC, A> {
        AchievementUnlockCall {
            hub: self.hub,
            _achievement_id: achievement_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *leaderboard* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.leaderboards();
/// # }
/// ```
pub struct LeaderboardMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for LeaderboardMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> LeaderboardMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the metadata of the leaderboard with the given ID.    
    pub fn get(&self, leaderboard_id: &str) -> LeaderboardGetCall<'a, C, NC, A> {
        LeaderboardGetCall {
            hub: self.hub,
            _leaderboard_id: leaderboard_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the leaderboard metadata for your application.    
    pub fn list(&self) -> LeaderboardListCall<'a, C, NC, A> {
        LeaderboardListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *metagame* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_metagame_config(...)` and `list_categories_by_player(...)`
/// // to build up your call.
/// let rb = hub.metagame();
/// # }
/// ```
pub struct MetagameMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for MetagameMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> MetagameMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List play data aggregated per category for the player corresponding to playerId.    
    pub fn list_categories_by_player(&self, player_id: &str, collection: &str) -> MetagameListCategoriesByPlayerCall<'a, C, NC, A> {
        MetagameListCategoriesByPlayerCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _collection: collection.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return the metagame configuration data for the calling application.    
    pub fn get_metagame_config(&self) -> MetagameGetMetagameConfigCall<'a, C, NC, A> {
        MetagameGetMetagameConfigCall {
            hub: self.hub,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *player* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.players();
/// # }
/// ```
pub struct PlayerMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for PlayerMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> PlayerMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the collection of players for the currently authenticated user.    
    pub fn list(&self, collection: &str) -> PlayerListCall<'a, C, NC, A> {
        PlayerListCall {
            hub: self.hub,
            _collection: collection.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the Player resource with the given ID. To retrieve the player for the currently authenticated user, set playerId to me.    
    pub fn get(&self, player_id: &str) -> PlayerGetCall<'a, C, NC, A> {
        PlayerGetCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *quest* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `accept(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.quests();
/// # }
/// ```
pub struct QuestMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for QuestMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> QuestMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Indicates that the currently authorized user will participate in the quest.    
    pub fn accept(&self, quest_id: &str) -> QuestAcceptCall<'a, C, NC, A> {
        QuestAcceptCall {
            hub: self.hub,
            _quest_id: quest_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a list of quests for your application and the currently authenticated player.    
    pub fn list(&self, player_id: &str) -> QuestListCall<'a, C, NC, A> {
        QuestListCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *snapshot* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.snapshots();
/// # }
/// ```
pub struct SnapshotMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for SnapshotMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> SnapshotMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the metadata for a given snapshot ID.    
    pub fn get(&self, snapshot_id: &str) -> SnapshotGetCall<'a, C, NC, A> {
        SnapshotGetCall {
            hub: self.hub,
            _snapshot_id: snapshot_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of snapshots created by your application for the player corresponding to the player ID.    
    pub fn list(&self, player_id: &str) -> SnapshotListCall<'a, C, NC, A> {
        SnapshotListCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *turnBasedMatche* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `create(...)`, `decline(...)`, `dismiss(...)`, `finish(...)`, `get(...)`, `join(...)`, `leave(...)`, `leave_turn(...)`, `list(...)`, `rematch(...)`, `sync(...)` and `take_turn(...)`
/// // to build up your call.
/// let rb = hub.turn_based_matches();
/// # }
/// ```
pub struct TurnBasedMatcheMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for TurnBasedMatcheMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Dismiss a turn-based match from the match list. The match will no longer show up in the list and will not generate notifications.    
    pub fn dismiss(&self, match_id: &str) -> TurnBasedMatcheDismisCall<'a, C, NC, A> {
        TurnBasedMatcheDismisCall {
            hub: self.hub,
            _match_id: match_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns turn-based matches the player is or was involved in that changed since the last sync call, with the least recent changes coming first. Matches that should be removed from the local cache will have a status of MATCH_DELETED.    
    pub fn sync(&self) -> TurnBasedMatcheSyncCall<'a, C, NC, A> {
        TurnBasedMatcheSyncCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _max_completed_matches: Default::default(),
            _language: Default::default(),
            _include_match_data: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Decline an invitation to play a turn-based match.    
    pub fn decline(&self, match_id: &str) -> TurnBasedMatcheDeclineCall<'a, C, NC, A> {
        TurnBasedMatcheDeclineCall {
            hub: self.hub,
            _match_id: match_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the data for a turn-based match.    
    pub fn get(&self, match_id: &str) -> TurnBasedMatcheGetCall<'a, C, NC, A> {
        TurnBasedMatcheGetCall {
            hub: self.hub,
            _match_id: match_id.to_string(),
            _language: Default::default(),
            _include_match_data: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a turn-based match.    
    pub fn create(&self, request: &TurnBasedMatchCreateRequest) -> TurnBasedMatcheCreateCall<'a, C, NC, A> {
        TurnBasedMatcheCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Join a turn-based match.    
    pub fn join(&self, match_id: &str) -> TurnBasedMatcheJoinCall<'a, C, NC, A> {
        TurnBasedMatcheJoinCall {
            hub: self.hub,
            _match_id: match_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Leave a turn-based match during the current player's turn, without canceling the match.    
    pub fn leave_turn(&self, match_id: &str, match_version: i32) -> TurnBasedMatcheLeaveTurnCall<'a, C, NC, A> {
        TurnBasedMatcheLeaveTurnCall {
            hub: self.hub,
            _match_id: match_id.to_string(),
            _match_version: match_version,
            _pending_participant_id: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancel a turn-based match.    
    pub fn cancel(&self, match_id: &str) -> TurnBasedMatcheCancelCall<'a, C, NC, A> {
        TurnBasedMatcheCancelCall {
            hub: self.hub,
            _match_id: match_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finish a turn-based match. Each player should make this call once, after all results are in. Only the player whose turn it is may make the first call to Finish, and can pass in the final match state.    
    pub fn finish(&self, request: &TurnBasedMatchResults, match_id: &str) -> TurnBasedMatcheFinishCall<'a, C, NC, A> {
        TurnBasedMatcheFinishCall {
            hub: self.hub,
            _request: request.clone(),
            _match_id: match_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Leave a turn-based match when it is not the current player's turn, without canceling the match.    
    pub fn leave(&self, match_id: &str) -> TurnBasedMatcheLeaveCall<'a, C, NC, A> {
        TurnBasedMatcheLeaveCall {
            hub: self.hub,
            _match_id: match_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns turn-based matches the player is or was involved in.    
    pub fn list(&self) -> TurnBasedMatcheListCall<'a, C, NC, A> {
        TurnBasedMatcheListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _max_completed_matches: Default::default(),
            _language: Default::default(),
            _include_match_data: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Commit the results of a player turn.    
    pub fn take_turn(&self, request: &TurnBasedMatchTurn, match_id: &str) -> TurnBasedMatcheTakeTurnCall<'a, C, NC, A> {
        TurnBasedMatcheTakeTurnCall {
            hub: self.hub,
            _request: request.clone(),
            _match_id: match_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a rematch of a match that was previously completed, with the same participants. This can be called by only one player on a match still in their list; the player must have called Finish first. Returns the newly created match; it will be the caller's turn.    
    pub fn rematch(&self, match_id: &str) -> TurnBasedMatcheRematchCall<'a, C, NC, A> {
        TurnBasedMatcheRematchCall {
            hub: self.hub,
            _match_id: match_id.to_string(),
            _request_id: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *application* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `played(...)`
/// // to build up your call.
/// let rb = hub.applications();
/// # }
/// ```
pub struct ApplicationMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ApplicationMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ApplicationMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the metadata of the application with the given ID. If the requested application is not available for the specified platformType, the returned response will not include any instance data.    
    pub fn get(&self, application_id: &str) -> ApplicationGetCall<'a, C, NC, A> {
        ApplicationGetCall {
            hub: self.hub,
            _application_id: application_id.to_string(),
            _platform_type: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Indicate that the the currently authenticated user is playing your application.    
    pub fn played(&self) -> ApplicationPlayedCall<'a, C, NC, A> {
        ApplicationPlayedCall {
            hub: self.hub,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *room* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `decline(...)`, `dismiss(...)`, `get(...)`, `join(...)`, `leave(...)`, `list(...)` and `report_status(...)`
/// // to build up your call.
/// let rb = hub.rooms();
/// # }
/// ```
pub struct RoomMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for RoomMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> RoomMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the data for a room.    
    pub fn get(&self, room_id: &str) -> RoomGetCall<'a, C, NC, A> {
        RoomGetCall {
            hub: self.hub,
            _room_id: room_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Leave a room. For internal use by the Games SDK only. Calling this method directly is unsupported.    
    pub fn leave(&self, request: &RoomLeaveRequest, room_id: &str) -> RoomLeaveCall<'a, C, NC, A> {
        RoomLeaveCall {
            hub: self.hub,
            _request: request.clone(),
            _room_id: room_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns invitations to join rooms.    
    pub fn list(&self) -> RoomListCall<'a, C, NC, A> {
        RoomListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates sent by a client reporting the status of peers in a room. For internal use by the Games SDK only. Calling this method directly is unsupported.    
    pub fn report_status(&self, request: &RoomP2PStatuses, room_id: &str) -> RoomReportStatuCall<'a, C, NC, A> {
        RoomReportStatuCall {
            hub: self.hub,
            _request: request.clone(),
            _room_id: room_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a room. For internal use by the Games SDK only. Calling this method directly is unsupported.    
    pub fn create(&self, request: &RoomCreateRequest) -> RoomCreateCall<'a, C, NC, A> {
        RoomCreateCall {
            hub: self.hub,
            _request: request.clone(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Decline an invitation to join a room. For internal use by the Games SDK only. Calling this method directly is unsupported.    
    pub fn decline(&self, room_id: &str) -> RoomDeclineCall<'a, C, NC, A> {
        RoomDeclineCall {
            hub: self.hub,
            _room_id: room_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Dismiss an invitation to join a room. For internal use by the Games SDK only. Calling this method directly is unsupported.    
    pub fn dismiss(&self, room_id: &str) -> RoomDismisCall<'a, C, NC, A> {
        RoomDismisCall {
            hub: self.hub,
            _room_id: room_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Join a room. For internal use by the Games SDK only. Calling this method directly is unsupported.    
    pub fn join(&self, request: &RoomJoinRequest, room_id: &str) -> RoomJoinCall<'a, C, NC, A> {
        RoomJoinCall {
            hub: self.hub,
            _request: request.clone(),
            _room_id: room_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *score* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `list_window(...)`, `submit(...)` and `submit_multiple(...)`
/// // to build up your call.
/// let rb = hub.scores();
/// # }
/// ```
pub struct ScoreMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ScoreMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ScoreMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submits a score to the specified leaderboard.    
    pub fn submit(&self, leaderboard_id: &str, score: &str) -> ScoreSubmitCall<'a, C, NC, A> {
        ScoreSubmitCall {
            hub: self.hub,
            _leaderboard_id: leaderboard_id.to_string(),
            _score: score.to_string(),
            _score_tag: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the scores in a leaderboard, starting from the top.    
    pub fn list(&self, leaderboard_id: &str, collection: &str, time_span: &str) -> ScoreListCall<'a, C, NC, A> {
        ScoreListCall {
            hub: self.hub,
            _leaderboard_id: leaderboard_id.to_string(),
            _collection: collection.to_string(),
            _time_span: time_span.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get high scores, and optionally ranks, in leaderboards for the currently authenticated player. For a specific time span, leaderboardId can be set to ALL to retrieve data for all leaderboards in a given time span.
    /// NOTE: You cannot ask for 'ALL' leaderboards and 'ALL' timeSpans in the same request; only one parameter may be set to 'ALL'.
    pub fn get(&self, player_id: &str, leaderboard_id: &str, time_span: &str) -> ScoreGetCall<'a, C, NC, A> {
        ScoreGetCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _leaderboard_id: leaderboard_id.to_string(),
            _time_span: time_span.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _include_rank_type: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submits multiple scores to leaderboards.    
    pub fn submit_multiple(&self, request: &PlayerScoreSubmissionList) -> ScoreSubmitMultipleCall<'a, C, NC, A> {
        ScoreSubmitMultipleCall {
            hub: self.hub,
            _request: request.clone(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the scores in a leaderboard around (and including) a player's score.    
    pub fn list_window(&self, leaderboard_id: &str, collection: &str, time_span: &str) -> ScoreListWindowCall<'a, C, NC, A> {
        ScoreListWindowCall {
            hub: self.hub,
            _leaderboard_id: leaderboard_id.to_string(),
            _collection: collection.to_string(),
            _time_span: time_span.to_string(),
            _return_top_if_absent: Default::default(),
            _results_above: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *pushtoken* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `remove(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.pushtokens();
/// # }
/// ```
pub struct PushtokenMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for PushtokenMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> PushtokenMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a push token for the current user and application. Removing a non-existent push token will report success.    
    pub fn remove(&self, request: &PushTokenId) -> PushtokenRemoveCall<'a, C, NC, A> {
        PushtokenRemoveCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Registers a push token for the current user and application.    
    pub fn update(&self, request: &PushToken) -> PushtokenUpdateCall<'a, C, NC, A> {
        PushtokenUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *revision* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `check(...)`
/// // to build up your call.
/// let rb = hub.revisions();
/// # }
/// ```
pub struct RevisionMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for RevisionMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> RevisionMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks whether the games client is out of date.    
    pub fn check(&self, client_revision: &str) -> RevisionCheckCall<'a, C, NC, A> {
        RevisionCheckCall {
            hub: self.hub,
            _client_revision: client_revision.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *event* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_by_player(...)`, `list_definitions(...)` and `record(...)`
/// // to build up your call.
/// let rb = hub.events();
/// # }
/// ```
pub struct EventMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for EventMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> EventMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of the event definitions in this application.    
    pub fn list_definitions(&self) -> EventListDefinitionCall<'a, C, NC, A> {
        EventListDefinitionCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Records a batch of changes to the number of times events have occurred for the currently authenticated user of this application.    
    pub fn record(&self, request: &EventRecordRequest) -> EventRecordCall<'a, C, NC, A> {
        EventRecordCall {
            hub: self.hub,
            _request: request.clone(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list showing the current progress on events in this application for the currently authenticated user.    
    pub fn list_by_player(&self) -> EventListByPlayerCall<'a, C, NC, A> {
        EventListByPlayerCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *questMilestone* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `claim(...)`
/// // to build up your call.
/// let rb = hub.quest_milestones();
/// # }
/// ```
pub struct QuestMilestoneMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for QuestMilestoneMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> QuestMilestoneMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Report that a reward for the milestone corresponding to milestoneId for the quest corresponding to questId has been claimed by the currently authorized user.    
    pub fn claim(&self, quest_id: &str, milestone_id: &str, request_id: &str) -> QuestMilestoneClaimCall<'a, C, NC, A> {
        QuestMilestoneClaimCall {
            hub: self.hub,
            _quest_id: quest_id.to_string(),
            _milestone_id: milestone_id.to_string(),
            _request_id: request_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *achievementDefinition* resources.
/// It is not used directly, but through the `Games` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-games1" as games1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use games1::Games;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Games::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.achievement_definitions();
/// # }
/// ```
pub struct AchievementDefinitionMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for AchievementDefinitionMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> AchievementDefinitionMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the achievement definitions for your application.    
    pub fn list(&self) -> AchievementDefinitionListCall<'a, C, NC, A> {
        AchievementDefinitionListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Lists the progress for all your application's achievements for the currently authenticated player.
///
/// A builder for the *list* method supported by a *achievement* resource.
/// It is not used directly, but through a `AchievementMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.achievements().list("playerId")
///              .state("sea")
///              .page_token("nonumy")
///              .max_results(-19)
///              .language("gubergren")
///              .doit();
/// # }
/// ```
pub struct AchievementListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _player_id: String,
    _state: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AchievementListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AchievementListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PlayerAchievementListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.achievements.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("playerId", self._player_id.to_string()));
        if let Some(value) = self._state {
            params.push(("state", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "playerId", "state", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/players/{playerId}/achievements".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{playerId}", "playerId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["playerId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *player id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A player ID. A value of me may be used in place of the authenticated player's ID.    
    pub fn player_id(mut self, new_value: &str) -> AchievementListCall<'a, C, NC, A> {
        self._player_id = new_value.to_string();
        self
    }
    /// Sets the *state* query property to the given value.
    ///
    /// 
    /// Tells the server to return only achievements with the specified state. If this parameter isn't specified, all achievements are returned.    
    pub fn state(mut self, new_value: &str) -> AchievementListCall<'a, C, NC, A> {
        self._state = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> AchievementListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of achievement resources to return in the response, used for paging. For any response, the actual number of achievement resources returned may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> AchievementListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> AchievementListCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AchievementListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> AchievementListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AchievementListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Increments the steps of the achievement with the given ID for the currently authenticated player.
///
/// A builder for the *increment* method supported by a *achievement* resource.
/// It is not used directly, but through a `AchievementMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.achievements().increment("achievementId", -31)
///              .request_id("ea")
///              .doit();
/// # }
/// ```
pub struct AchievementIncrementCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _achievement_id: String,
    _steps_to_increment: i32,
    _request_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AchievementIncrementCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AchievementIncrementCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AchievementIncrementResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.achievements.increment", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("achievementId", self._achievement_id.to_string()));
        params.push(("stepsToIncrement", self._steps_to_increment.to_string()));
        if let Some(value) = self._request_id {
            params.push(("requestId", value.to_string()));
        }
        for &field in ["alt", "achievementId", "stepsToIncrement", "requestId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/achievements/{achievementId}/increment".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{achievementId}", "achievementId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["achievementId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *achievement id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the achievement used by this method.    
    pub fn achievement_id(mut self, new_value: &str) -> AchievementIncrementCall<'a, C, NC, A> {
        self._achievement_id = new_value.to_string();
        self
    }
    /// Sets the *steps to increment* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The number of steps to increment.    
    pub fn steps_to_increment(mut self, new_value: i32) -> AchievementIncrementCall<'a, C, NC, A> {
        self._steps_to_increment = new_value;
        self
    }
    /// Sets the *request id* query property to the given value.
    ///
    /// 
    /// A randomly generated numeric ID for each request specified by the caller. This number is used at the server to ensure that the request is handled correctly across retries.    
    pub fn request_id(mut self, new_value: &str) -> AchievementIncrementCall<'a, C, NC, A> {
        self._request_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AchievementIncrementCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> AchievementIncrementCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AchievementIncrementCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Sets the steps for the currently authenticated player towards unlocking an achievement. If the steps parameter is less than the current number of steps that the player already gained for the achievement, the achievement is not modified.
///
/// A builder for the *setStepsAtLeast* method supported by a *achievement* resource.
/// It is not used directly, but through a `AchievementMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.achievements().set_steps_at_least("achievementId", -21)
///              .doit();
/// # }
/// ```
pub struct AchievementSetStepsAtLeastCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _achievement_id: String,
    _steps: i32,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AchievementSetStepsAtLeastCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AchievementSetStepsAtLeastCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AchievementSetStepsAtLeastResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.achievements.setStepsAtLeast", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("achievementId", self._achievement_id.to_string()));
        params.push(("steps", self._steps.to_string()));
        for &field in ["alt", "achievementId", "steps"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/achievements/{achievementId}/setStepsAtLeast".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{achievementId}", "achievementId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["achievementId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *achievement id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the achievement used by this method.    
    pub fn achievement_id(mut self, new_value: &str) -> AchievementSetStepsAtLeastCall<'a, C, NC, A> {
        self._achievement_id = new_value.to_string();
        self
    }
    /// Sets the *steps* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The minimum value to set the steps to.    
    pub fn steps(mut self, new_value: i32) -> AchievementSetStepsAtLeastCall<'a, C, NC, A> {
        self._steps = new_value;
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AchievementSetStepsAtLeastCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> AchievementSetStepsAtLeastCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AchievementSetStepsAtLeastCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates multiple achievements for the currently authenticated player.
///
/// A builder for the *updateMultiple* method supported by a *achievement* resource.
/// It is not used directly, but through a `AchievementMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::AchievementUpdateMultipleRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: AchievementUpdateMultipleRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.achievements().update_multiple(&req)
///              .doit();
/// # }
/// ```
pub struct AchievementUpdateMultipleCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: AchievementUpdateMultipleRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AchievementUpdateMultipleCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AchievementUpdateMultipleCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AchievementUpdateMultipleResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.achievements.updateMultiple", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/achievements/updateMultiple".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &AchievementUpdateMultipleRequest) -> AchievementUpdateMultipleCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AchievementUpdateMultipleCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> AchievementUpdateMultipleCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AchievementUpdateMultipleCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Sets the state of the achievement with the given ID to REVEALED for the currently authenticated player.
///
/// A builder for the *reveal* method supported by a *achievement* resource.
/// It is not used directly, but through a `AchievementMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.achievements().reveal("achievementId")
///              .doit();
/// # }
/// ```
pub struct AchievementRevealCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _achievement_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AchievementRevealCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AchievementRevealCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AchievementRevealResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.achievements.reveal", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("achievementId", self._achievement_id.to_string()));
        for &field in ["alt", "achievementId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/achievements/{achievementId}/reveal".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{achievementId}", "achievementId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["achievementId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *achievement id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the achievement used by this method.    
    pub fn achievement_id(mut self, new_value: &str) -> AchievementRevealCall<'a, C, NC, A> {
        self._achievement_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AchievementRevealCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> AchievementRevealCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AchievementRevealCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Unlocks this achievement for the currently authenticated player.
///
/// A builder for the *unlock* method supported by a *achievement* resource.
/// It is not used directly, but through a `AchievementMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.achievements().unlock("achievementId")
///              .doit();
/// # }
/// ```
pub struct AchievementUnlockCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _achievement_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AchievementUnlockCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AchievementUnlockCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AchievementUnlockResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.achievements.unlock", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("achievementId", self._achievement_id.to_string()));
        for &field in ["alt", "achievementId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/achievements/{achievementId}/unlock".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{achievementId}", "achievementId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["achievementId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *achievement id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the achievement used by this method.    
    pub fn achievement_id(mut self, new_value: &str) -> AchievementUnlockCall<'a, C, NC, A> {
        self._achievement_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AchievementUnlockCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> AchievementUnlockCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AchievementUnlockCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves the metadata of the leaderboard with the given ID.
///
/// A builder for the *get* method supported by a *leaderboard* resource.
/// It is not used directly, but through a `LeaderboardMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.leaderboards().get("leaderboardId")
///              .language("diam")
///              .doit();
/// # }
/// ```
pub struct LeaderboardGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _leaderboard_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LeaderboardGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LeaderboardGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Leaderboard)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.leaderboards.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("leaderboardId", self._leaderboard_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "leaderboardId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/leaderboards/{leaderboardId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{leaderboardId}", "leaderboardId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["leaderboardId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *leaderboard id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the leaderboard.    
    pub fn leaderboard_id(mut self, new_value: &str) -> LeaderboardGetCall<'a, C, NC, A> {
        self._leaderboard_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> LeaderboardGetCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LeaderboardGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LeaderboardGetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LeaderboardGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists all the leaderboard metadata for your application.
///
/// A builder for the *list* method supported by a *leaderboard* resource.
/// It is not used directly, but through a `LeaderboardMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.leaderboards().list()
///              .page_token("ipsum")
///              .max_results(-5)
///              .language("et")
///              .doit();
/// # }
/// ```
pub struct LeaderboardListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LeaderboardListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LeaderboardListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LeaderboardListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.leaderboards.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/leaderboards".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> LeaderboardListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of leaderboards to return in the response. For any response, the actual number of leaderboards returned may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> LeaderboardListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> LeaderboardListCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LeaderboardListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LeaderboardListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LeaderboardListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// List play data aggregated per category for the player corresponding to playerId.
///
/// A builder for the *listCategoriesByPlayer* method supported by a *metagame* resource.
/// It is not used directly, but through a `MetagameMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.metagame().list_categories_by_player("playerId", "collection")
///              .page_token("sea")
///              .max_results(-55)
///              .language("eos")
///              .doit();
/// # }
/// ```
pub struct MetagameListCategoriesByPlayerCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _player_id: String,
    _collection: String,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MetagameListCategoriesByPlayerCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MetagameListCategoriesByPlayerCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, CategoryListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.metagame.listCategoriesByPlayer", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("playerId", self._player_id.to_string()));
        params.push(("collection", self._collection.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "playerId", "collection", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/players/{playerId}/categories/{collection}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{playerId}", "playerId"), ("{collection}", "collection")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["playerId", "collection"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *player id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A player ID. A value of me may be used in place of the authenticated player's ID.    
    pub fn player_id(mut self, new_value: &str) -> MetagameListCategoriesByPlayerCall<'a, C, NC, A> {
        self._player_id = new_value.to_string();
        self
    }
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The collection of categories for which data will be returned.    
    pub fn collection(mut self, new_value: &str) -> MetagameListCategoriesByPlayerCall<'a, C, NC, A> {
        self._collection = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> MetagameListCategoriesByPlayerCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of category resources to return in the response, used for paging. For any response, the actual number of category resources returned may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> MetagameListCategoriesByPlayerCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> MetagameListCategoriesByPlayerCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MetagameListCategoriesByPlayerCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MetagameListCategoriesByPlayerCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MetagameListCategoriesByPlayerCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Return the metagame configuration data for the calling application.
///
/// A builder for the *getMetagameConfig* method supported by a *metagame* resource.
/// It is not used directly, but through a `MetagameMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.metagame().get_metagame_config()
///              .doit();
/// # }
/// ```
pub struct MetagameGetMetagameConfigCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for MetagameGetMetagameConfigCall<'a, C, NC, A> {}

impl<'a, C, NC, A> MetagameGetMetagameConfigCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, MetagameConfig)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.metagame.getMetagameConfig", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/metagameConfig".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> MetagameGetMetagameConfigCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> MetagameGetMetagameConfigCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> MetagameGetMetagameConfigCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Get the collection of players for the currently authenticated user.
///
/// A builder for the *list* method supported by a *player* resource.
/// It is not used directly, but through a `PlayerMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.players().list("collection")
///              .page_token("sadipscing")
///              .max_results(-48)
///              .language("eirmod")
///              .doit();
/// # }
/// ```
pub struct PlayerListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _collection: String,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PlayerListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PlayerListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PlayerListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.players.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("collection", self._collection.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "collection", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/players/me/players/{collection}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{collection}", "collection")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["collection"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Collection of players being retrieved    
    pub fn collection(mut self, new_value: &str) -> PlayerListCall<'a, C, NC, A> {
        self._collection = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> PlayerListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of player resources to return in the response, used for paging. For any response, the actual number of player resources returned may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> PlayerListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> PlayerListCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlayerListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PlayerListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PlayerListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves the Player resource with the given ID. To retrieve the player for the currently authenticated user, set playerId to me.
///
/// A builder for the *get* method supported by a *player* resource.
/// It is not used directly, but through a `PlayerMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.players().get("playerId")
///              .language("amet")
///              .doit();
/// # }
/// ```
pub struct PlayerGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _player_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PlayerGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PlayerGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Player)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.players.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("playerId", self._player_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "playerId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/players/{playerId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{playerId}", "playerId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["playerId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *player id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A player ID. A value of me may be used in place of the authenticated player's ID.    
    pub fn player_id(mut self, new_value: &str) -> PlayerGetCall<'a, C, NC, A> {
        self._player_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> PlayerGetCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlayerGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PlayerGetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PlayerGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Indicates that the currently authorized user will participate in the quest.
///
/// A builder for the *accept* method supported by a *quest* resource.
/// It is not used directly, but through a `QuestMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.quests().accept("questId")
///              .language("labore")
///              .doit();
/// # }
/// ```
pub struct QuestAcceptCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _quest_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for QuestAcceptCall<'a, C, NC, A> {}

impl<'a, C, NC, A> QuestAcceptCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Quest)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.quests.accept", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("questId", self._quest_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "questId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/quests/{questId}/accept".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{questId}", "questId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["questId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *quest id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the quest.    
    pub fn quest_id(mut self, new_value: &str) -> QuestAcceptCall<'a, C, NC, A> {
        self._quest_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> QuestAcceptCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> QuestAcceptCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> QuestAcceptCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> QuestAcceptCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Get a list of quests for your application and the currently authenticated player.
///
/// A builder for the *list* method supported by a *quest* resource.
/// It is not used directly, but through a `QuestMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.quests().list("playerId")
///              .page_token("dolore")
///              .max_results(-37)
///              .language("aliquyam")
///              .doit();
/// # }
/// ```
pub struct QuestListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _player_id: String,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for QuestListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> QuestListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, QuestListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.quests.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("playerId", self._player_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "playerId", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/players/{playerId}/quests".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{playerId}", "playerId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["playerId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *player id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A player ID. A value of me may be used in place of the authenticated player's ID.    
    pub fn player_id(mut self, new_value: &str) -> QuestListCall<'a, C, NC, A> {
        self._player_id = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> QuestListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of quest resources to return in the response, used for paging. For any response, the actual number of quest resources returned may be less than the specified maxResults. Acceptable values are 1 to 50, inclusive. (Default: 50).    
    pub fn max_results(mut self, new_value: i32) -> QuestListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> QuestListCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> QuestListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> QuestListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> QuestListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves the metadata for a given snapshot ID.
///
/// A builder for the *get* method supported by a *snapshot* resource.
/// It is not used directly, but through a `SnapshotMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.snapshots().get("snapshotId")
///              .language("Lorem")
///              .doit();
/// # }
/// ```
pub struct SnapshotGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _snapshot_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for SnapshotGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> SnapshotGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Snapshot)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.snapshots.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("snapshotId", self._snapshot_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "snapshotId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/snapshots/{snapshotId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{snapshotId}", "snapshotId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["snapshotId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *snapshot id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the snapshot.    
    pub fn snapshot_id(mut self, new_value: &str) -> SnapshotGetCall<'a, C, NC, A> {
        self._snapshot_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> SnapshotGetCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SnapshotGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> SnapshotGetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> SnapshotGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves a list of snapshots created by your application for the player corresponding to the player ID.
///
/// A builder for the *list* method supported by a *snapshot* resource.
/// It is not used directly, but through a `SnapshotMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.snapshots().list("playerId")
///              .page_token("et")
///              .max_results(-70)
///              .language("et")
///              .doit();
/// # }
/// ```
pub struct SnapshotListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _player_id: String,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for SnapshotListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> SnapshotListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SnapshotListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.snapshots.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("playerId", self._player_id.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "playerId", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/players/{playerId}/snapshots".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{playerId}", "playerId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["playerId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *player id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A player ID. A value of me may be used in place of the authenticated player's ID.    
    pub fn player_id(mut self, new_value: &str) -> SnapshotListCall<'a, C, NC, A> {
        self._player_id = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> SnapshotListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of snapshot resources to return in the response, used for paging. For any response, the actual number of snapshot resources returned may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> SnapshotListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> SnapshotListCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SnapshotListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> SnapshotListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> SnapshotListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Dismiss a turn-based match from the match list. The match will no longer show up in the list and will not generate notifications.
///
/// A builder for the *dismiss* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().dismiss("matchId")
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheDismisCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _match_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheDismisCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheDismisCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.dismiss", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        params.push(("matchId", self._match_id.to_string()));
        for &field in ["matchId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/{matchId}/dismiss".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{matchId}", "matchId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["matchId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *match id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the match.    
    pub fn match_id(mut self, new_value: &str) -> TurnBasedMatcheDismisCall<'a, C, NC, A> {
        self._match_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheDismisCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheDismisCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheDismisCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns turn-based matches the player is or was involved in that changed since the last sync call, with the least recent changes coming first. Matches that should be removed from the local cache will have a status of MATCH_DELETED.
///
/// A builder for the *sync* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().sync()
///              .page_token("sanctus")
///              .max_results(-22)
///              .max_completed_matches(-46)
///              .language("et")
///              .include_match_data(true)
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheSyncCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _max_completed_matches: Option<i32>,
    _language: Option<String>,
    _include_match_data: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheSyncCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheSyncCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatchSync)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.sync", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._max_completed_matches {
            params.push(("maxCompletedMatches", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        if let Some(value) = self._include_match_data {
            params.push(("includeMatchData", value.to_string()));
        }
        for &field in ["alt", "pageToken", "maxResults", "maxCompletedMatches", "language", "includeMatchData"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/sync".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> TurnBasedMatcheSyncCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of matches to return in the response, used for paging. For any response, the actual number of matches to return may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> TurnBasedMatcheSyncCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *max completed matches* query property to the given value.
    ///
    /// 
    /// The maximum number of completed or canceled matches to return in the response. If not set, all matches returned could be completed or canceled.    
    pub fn max_completed_matches(mut self, new_value: i32) -> TurnBasedMatcheSyncCall<'a, C, NC, A> {
        self._max_completed_matches = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheSyncCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *include match data* query property to the given value.
    ///
    /// 
    /// True if match data should be returned in the response. Note that not all data will necessarily be returned if include_match_data is true; the server may decide to only return data for some of the matches to limit download size for the client. The remainder of the data for these matches will be retrievable on request.    
    pub fn include_match_data(mut self, new_value: bool) -> TurnBasedMatcheSyncCall<'a, C, NC, A> {
        self._include_match_data = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheSyncCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheSyncCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheSyncCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Decline an invitation to play a turn-based match.
///
/// A builder for the *decline* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().decline("matchId")
///              .language("ea")
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheDeclineCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _match_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheDeclineCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheDeclineCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatch)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.decline", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("matchId", self._match_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "matchId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/{matchId}/decline".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{matchId}", "matchId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["matchId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *match id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the match.    
    pub fn match_id(mut self, new_value: &str) -> TurnBasedMatcheDeclineCall<'a, C, NC, A> {
        self._match_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheDeclineCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheDeclineCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheDeclineCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheDeclineCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Get the data for a turn-based match.
///
/// A builder for the *get* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().get("matchId")
///              .language("dolor")
///              .include_match_data(true)
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _match_id: String,
    _language: Option<String>,
    _include_match_data: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatch)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("matchId", self._match_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        if let Some(value) = self._include_match_data {
            params.push(("includeMatchData", value.to_string()));
        }
        for &field in ["alt", "matchId", "language", "includeMatchData"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/{matchId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{matchId}", "matchId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["matchId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *match id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the match.    
    pub fn match_id(mut self, new_value: &str) -> TurnBasedMatcheGetCall<'a, C, NC, A> {
        self._match_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheGetCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *include match data* query property to the given value.
    ///
    /// 
    /// Get match data along with metadata.    
    pub fn include_match_data(mut self, new_value: bool) -> TurnBasedMatcheGetCall<'a, C, NC, A> {
        self._include_match_data = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheGetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Create a turn-based match.
///
/// A builder for the *create* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::TurnBasedMatchCreateRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: TurnBasedMatchCreateRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().create(&req)
///              .language("dolor")
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: TurnBasedMatchCreateRequest,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatch)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/create".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &TurnBasedMatchCreateRequest) -> TurnBasedMatcheCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheCreateCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheCreateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheCreateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheCreateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Join a turn-based match.
///
/// A builder for the *join* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().join("matchId")
///              .language("consetetur")
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheJoinCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _match_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheJoinCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheJoinCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatch)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.join", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("matchId", self._match_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "matchId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/{matchId}/join".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{matchId}", "matchId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["matchId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *match id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the match.    
    pub fn match_id(mut self, new_value: &str) -> TurnBasedMatcheJoinCall<'a, C, NC, A> {
        self._match_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheJoinCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheJoinCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheJoinCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheJoinCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Leave a turn-based match during the current player's turn, without canceling the match.
///
/// A builder for the *leaveTurn* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().leave_turn("matchId", -27)
///              .pending_participant_id("Lorem")
///              .language("gubergren")
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheLeaveTurnCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _match_id: String,
    _match_version: i32,
    _pending_participant_id: Option<String>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheLeaveTurnCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheLeaveTurnCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatch)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.leaveTurn", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("matchId", self._match_id.to_string()));
        params.push(("matchVersion", self._match_version.to_string()));
        if let Some(value) = self._pending_participant_id {
            params.push(("pendingParticipantId", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "matchId", "matchVersion", "pendingParticipantId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/{matchId}/leaveTurn".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{matchId}", "matchId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["matchId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *match id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the match.    
    pub fn match_id(mut self, new_value: &str) -> TurnBasedMatcheLeaveTurnCall<'a, C, NC, A> {
        self._match_id = new_value.to_string();
        self
    }
    /// Sets the *match version* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The version of the match being updated.    
    pub fn match_version(mut self, new_value: i32) -> TurnBasedMatcheLeaveTurnCall<'a, C, NC, A> {
        self._match_version = new_value;
        self
    }
    /// Sets the *pending participant id* query property to the given value.
    ///
    /// 
    /// The ID of another participant who should take their turn next. If not set, the match will wait for other player(s) to join via automatching; this is only valid if automatch criteria is set on the match with remaining slots for automatched players.    
    pub fn pending_participant_id(mut self, new_value: &str) -> TurnBasedMatcheLeaveTurnCall<'a, C, NC, A> {
        self._pending_participant_id = Some(new_value.to_string());
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheLeaveTurnCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheLeaveTurnCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheLeaveTurnCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheLeaveTurnCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Cancel a turn-based match.
///
/// A builder for the *cancel* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().cancel("matchId")
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheCancelCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _match_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheCancelCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheCancelCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.cancel", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        params.push(("matchId", self._match_id.to_string()));
        for &field in ["matchId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/{matchId}/cancel".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{matchId}", "matchId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["matchId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *match id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the match.    
    pub fn match_id(mut self, new_value: &str) -> TurnBasedMatcheCancelCall<'a, C, NC, A> {
        self._match_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheCancelCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheCancelCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheCancelCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Finish a turn-based match. Each player should make this call once, after all results are in. Only the player whose turn it is may make the first call to Finish, and can pass in the final match state.
///
/// A builder for the *finish* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::TurnBasedMatchResults;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: TurnBasedMatchResults = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().finish(&req, "matchId")
///              .language("vero")
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheFinishCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: TurnBasedMatchResults,
    _match_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheFinishCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheFinishCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatch)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.finish", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("matchId", self._match_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "matchId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/{matchId}/finish".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{matchId}", "matchId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["matchId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &TurnBasedMatchResults) -> TurnBasedMatcheFinishCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *match id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the match.    
    pub fn match_id(mut self, new_value: &str) -> TurnBasedMatcheFinishCall<'a, C, NC, A> {
        self._match_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheFinishCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheFinishCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheFinishCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheFinishCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Leave a turn-based match when it is not the current player's turn, without canceling the match.
///
/// A builder for the *leave* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().leave("matchId")
///              .language("rebum.")
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheLeaveCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _match_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheLeaveCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheLeaveCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatch)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.leave", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("matchId", self._match_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "matchId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/{matchId}/leave".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{matchId}", "matchId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["matchId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *match id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the match.    
    pub fn match_id(mut self, new_value: &str) -> TurnBasedMatcheLeaveCall<'a, C, NC, A> {
        self._match_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheLeaveCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheLeaveCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheLeaveCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheLeaveCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns turn-based matches the player is or was involved in.
///
/// A builder for the *list* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().list()
///              .page_token("consetetur")
///              .max_results(-44)
///              .max_completed_matches(-76)
///              .language("sadipscing")
///              .include_match_data(false)
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _max_completed_matches: Option<i32>,
    _language: Option<String>,
    _include_match_data: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatchList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._max_completed_matches {
            params.push(("maxCompletedMatches", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        if let Some(value) = self._include_match_data {
            params.push(("includeMatchData", value.to_string()));
        }
        for &field in ["alt", "pageToken", "maxResults", "maxCompletedMatches", "language", "includeMatchData"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> TurnBasedMatcheListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of matches to return in the response, used for paging. For any response, the actual number of matches to return may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> TurnBasedMatcheListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *max completed matches* query property to the given value.
    ///
    /// 
    /// The maximum number of completed or canceled matches to return in the response. If not set, all matches returned could be completed or canceled.    
    pub fn max_completed_matches(mut self, new_value: i32) -> TurnBasedMatcheListCall<'a, C, NC, A> {
        self._max_completed_matches = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheListCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *include match data* query property to the given value.
    ///
    /// 
    /// True if match data should be returned in the response. Note that not all data will necessarily be returned if include_match_data is true; the server may decide to only return data for some of the matches to limit download size for the client. The remainder of the data for these matches will be retrievable on request.    
    pub fn include_match_data(mut self, new_value: bool) -> TurnBasedMatcheListCall<'a, C, NC, A> {
        self._include_match_data = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Commit the results of a player turn.
///
/// A builder for the *takeTurn* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::TurnBasedMatchTurn;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: TurnBasedMatchTurn = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().take_turn(&req, "matchId")
///              .language("dolore")
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheTakeTurnCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: TurnBasedMatchTurn,
    _match_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheTakeTurnCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheTakeTurnCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatch)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.takeTurn", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("matchId", self._match_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "matchId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/{matchId}/turn".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{matchId}", "matchId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["matchId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &TurnBasedMatchTurn) -> TurnBasedMatcheTakeTurnCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *match id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the match.    
    pub fn match_id(mut self, new_value: &str) -> TurnBasedMatcheTakeTurnCall<'a, C, NC, A> {
        self._match_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheTakeTurnCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheTakeTurnCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheTakeTurnCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheTakeTurnCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Create a rematch of a match that was previously completed, with the same participants. This can be called by only one player on a match still in their list; the player must have called Finish first. Returns the newly created match; it will be the caller's turn.
///
/// A builder for the *rematch* method supported by a *turnBasedMatche* resource.
/// It is not used directly, but through a `TurnBasedMatcheMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.turn_based_matches().rematch("matchId")
///              .request_id("aliquyam")
///              .language("Lorem")
///              .doit();
/// # }
/// ```
pub struct TurnBasedMatcheRematchCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _match_id: String,
    _request_id: Option<String>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for TurnBasedMatcheRematchCall<'a, C, NC, A> {}

impl<'a, C, NC, A> TurnBasedMatcheRematchCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, TurnBasedMatchRematch)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.turnBasedMatches.rematch", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("matchId", self._match_id.to_string()));
        if let Some(value) = self._request_id {
            params.push(("requestId", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "matchId", "requestId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/turnbasedmatches/{matchId}/rematch".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{matchId}", "matchId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["matchId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *match id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the match.    
    pub fn match_id(mut self, new_value: &str) -> TurnBasedMatcheRematchCall<'a, C, NC, A> {
        self._match_id = new_value.to_string();
        self
    }
    /// Sets the *request id* query property to the given value.
    ///
    /// 
    /// A randomly generated numeric ID for each request specified by the caller. This number is used at the server to ensure that the request is handled correctly across retries.    
    pub fn request_id(mut self, new_value: &str) -> TurnBasedMatcheRematchCall<'a, C, NC, A> {
        self._request_id = Some(new_value.to_string());
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> TurnBasedMatcheRematchCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> TurnBasedMatcheRematchCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> TurnBasedMatcheRematchCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> TurnBasedMatcheRematchCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves the metadata of the application with the given ID. If the requested application is not available for the specified platformType, the returned response will not include any instance data.
///
/// A builder for the *get* method supported by a *application* resource.
/// It is not used directly, but through a `ApplicationMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.applications().get("applicationId")
///              .platform_type("clita")
///              .language("consetetur")
///              .doit();
/// # }
/// ```
pub struct ApplicationGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _application_id: String,
    _platform_type: Option<String>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ApplicationGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ApplicationGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Application)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.applications.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("applicationId", self._application_id.to_string()));
        if let Some(value) = self._platform_type {
            params.push(("platformType", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "applicationId", "platformType", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/applications/{applicationId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{applicationId}", "applicationId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["applicationId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *application id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The application ID from the Google Play developer console.    
    pub fn application_id(mut self, new_value: &str) -> ApplicationGetCall<'a, C, NC, A> {
        self._application_id = new_value.to_string();
        self
    }
    /// Sets the *platform type* query property to the given value.
    ///
    /// 
    /// Restrict application details returned to the specific platform.    
    pub fn platform_type(mut self, new_value: &str) -> ApplicationGetCall<'a, C, NC, A> {
        self._platform_type = Some(new_value.to_string());
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> ApplicationGetCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ApplicationGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ApplicationGetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ApplicationGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Indicate that the the currently authenticated user is playing your application.
///
/// A builder for the *played* method supported by a *application* resource.
/// It is not used directly, but through a `ApplicationMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.applications().played()
///              .doit();
/// # }
/// ```
pub struct ApplicationPlayedCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ApplicationPlayedCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ApplicationPlayedCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.applications.played", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((1 + self._additional_params.len()));
        for &field in [].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/games/v1/applications/played".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ApplicationPlayedCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ApplicationPlayedCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ApplicationPlayedCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Get the data for a room.
///
/// A builder for the *get* method supported by a *room* resource.
/// It is not used directly, but through a `RoomMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.rooms().get("roomId")
///              .language("nonumy")
///              .doit();
/// # }
/// ```
pub struct RoomGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _room_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for RoomGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RoomGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Room)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.rooms.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("roomId", self._room_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "roomId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/rooms/{roomId}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{roomId}", "roomId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["roomId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *room id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the room.    
    pub fn room_id(mut self, new_value: &str) -> RoomGetCall<'a, C, NC, A> {
        self._room_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> RoomGetCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RoomGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RoomGetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> RoomGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Leave a room. For internal use by the Games SDK only. Calling this method directly is unsupported.
///
/// A builder for the *leave* method supported by a *room* resource.
/// It is not used directly, but through a `RoomMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::RoomLeaveRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: RoomLeaveRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.rooms().leave(&req, "roomId")
///              .language("sanctus")
///              .doit();
/// # }
/// ```
pub struct RoomLeaveCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: RoomLeaveRequest,
    _room_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for RoomLeaveCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RoomLeaveCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Room)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.rooms.leave", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("roomId", self._room_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "roomId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/rooms/{roomId}/leave".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{roomId}", "roomId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["roomId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &RoomLeaveRequest) -> RoomLeaveCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *room id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the room.    
    pub fn room_id(mut self, new_value: &str) -> RoomLeaveCall<'a, C, NC, A> {
        self._room_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> RoomLeaveCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RoomLeaveCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RoomLeaveCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> RoomLeaveCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns invitations to join rooms.
///
/// A builder for the *list* method supported by a *room* resource.
/// It is not used directly, but through a `RoomMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.rooms().list()
///              .page_token("takimata")
///              .max_results(-27)
///              .language("labore")
///              .doit();
/// # }
/// ```
pub struct RoomListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for RoomListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RoomListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RoomList)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.rooms.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/rooms".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> RoomListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of rooms to return in the response, used for paging. For any response, the actual number of rooms to return may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> RoomListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> RoomListCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RoomListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RoomListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> RoomListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates sent by a client reporting the status of peers in a room. For internal use by the Games SDK only. Calling this method directly is unsupported.
///
/// A builder for the *reportStatus* method supported by a *room* resource.
/// It is not used directly, but through a `RoomMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::RoomP2PStatuses;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: RoomP2PStatuses = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.rooms().report_status(&req, "roomId")
///              .language("ea")
///              .doit();
/// # }
/// ```
pub struct RoomReportStatuCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: RoomP2PStatuses,
    _room_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for RoomReportStatuCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RoomReportStatuCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RoomStatus)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.rooms.reportStatus", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("roomId", self._room_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "roomId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/rooms/{roomId}/reportstatus".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{roomId}", "roomId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["roomId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &RoomP2PStatuses) -> RoomReportStatuCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *room id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the room.    
    pub fn room_id(mut self, new_value: &str) -> RoomReportStatuCall<'a, C, NC, A> {
        self._room_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> RoomReportStatuCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RoomReportStatuCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RoomReportStatuCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> RoomReportStatuCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Create a room. For internal use by the Games SDK only. Calling this method directly is unsupported.
///
/// A builder for the *create* method supported by a *room* resource.
/// It is not used directly, but through a `RoomMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::RoomCreateRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: RoomCreateRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.rooms().create(&req)
///              .language("sadipscing")
///              .doit();
/// # }
/// ```
pub struct RoomCreateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: RoomCreateRequest,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for RoomCreateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RoomCreateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Room)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.rooms.create", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/rooms/create".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &RoomCreateRequest) -> RoomCreateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> RoomCreateCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RoomCreateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RoomCreateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> RoomCreateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Decline an invitation to join a room. For internal use by the Games SDK only. Calling this method directly is unsupported.
///
/// A builder for the *decline* method supported by a *room* resource.
/// It is not used directly, but through a `RoomMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.rooms().decline("roomId")
///              .language("dolore")
///              .doit();
/// # }
/// ```
pub struct RoomDeclineCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _room_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for RoomDeclineCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RoomDeclineCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Room)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.rooms.decline", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("roomId", self._room_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "roomId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/rooms/{roomId}/decline".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{roomId}", "roomId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["roomId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *room id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the room.    
    pub fn room_id(mut self, new_value: &str) -> RoomDeclineCall<'a, C, NC, A> {
        self._room_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> RoomDeclineCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RoomDeclineCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RoomDeclineCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> RoomDeclineCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Dismiss an invitation to join a room. For internal use by the Games SDK only. Calling this method directly is unsupported.
///
/// A builder for the *dismiss* method supported by a *room* resource.
/// It is not used directly, but through a `RoomMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.rooms().dismiss("roomId")
///              .doit();
/// # }
/// ```
pub struct RoomDismisCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _room_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for RoomDismisCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RoomDismisCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.rooms.dismiss", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        params.push(("roomId", self._room_id.to_string()));
        for &field in ["roomId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/games/v1/rooms/{roomId}/dismiss".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{roomId}", "roomId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["roomId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *room id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the room.    
    pub fn room_id(mut self, new_value: &str) -> RoomDismisCall<'a, C, NC, A> {
        self._room_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RoomDismisCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RoomDismisCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> RoomDismisCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Join a room. For internal use by the Games SDK only. Calling this method directly is unsupported.
///
/// A builder for the *join* method supported by a *room* resource.
/// It is not used directly, but through a `RoomMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::RoomJoinRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: RoomJoinRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.rooms().join(&req, "roomId")
///              .language("aliquyam")
///              .doit();
/// # }
/// ```
pub struct RoomJoinCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: RoomJoinRequest,
    _room_id: String,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for RoomJoinCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RoomJoinCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Room)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.rooms.join", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("roomId", self._room_id.to_string()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "roomId", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/rooms/{roomId}/join".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{roomId}", "roomId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["roomId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &RoomJoinRequest) -> RoomJoinCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *room id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the room.    
    pub fn room_id(mut self, new_value: &str) -> RoomJoinCall<'a, C, NC, A> {
        self._room_id = new_value.to_string();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> RoomJoinCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RoomJoinCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RoomJoinCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> RoomJoinCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Submits a score to the specified leaderboard.
///
/// A builder for the *submit* method supported by a *score* resource.
/// It is not used directly, but through a `ScoreMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.scores().submit("leaderboardId", "score")
///              .score_tag("consetetur")
///              .language("labore")
///              .doit();
/// # }
/// ```
pub struct ScoreSubmitCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _leaderboard_id: String,
    _score: String,
    _score_tag: Option<String>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ScoreSubmitCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ScoreSubmitCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PlayerScoreResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.scores.submit", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("leaderboardId", self._leaderboard_id.to_string()));
        params.push(("score", self._score.to_string()));
        if let Some(value) = self._score_tag {
            params.push(("scoreTag", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "leaderboardId", "score", "scoreTag", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/leaderboards/{leaderboardId}/scores".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{leaderboardId}", "leaderboardId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["leaderboardId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *leaderboard id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the leaderboard.    
    pub fn leaderboard_id(mut self, new_value: &str) -> ScoreSubmitCall<'a, C, NC, A> {
        self._leaderboard_id = new_value.to_string();
        self
    }
    /// Sets the *score* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The score you're submitting. The submitted score is ignored if it is worse than a previously submitted score, where worse depends on the leaderboard sort order. The meaning of the score value depends on the leaderboard format type. For fixed-point, the score represents the raw value. For time, the score represents elapsed time in milliseconds. For currency, the score represents a value in micro units.    
    pub fn score(mut self, new_value: &str) -> ScoreSubmitCall<'a, C, NC, A> {
        self._score = new_value.to_string();
        self
    }
    /// Sets the *score tag* query property to the given value.
    ///
    /// 
    /// Additional information about the score you're submitting. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.    
    pub fn score_tag(mut self, new_value: &str) -> ScoreSubmitCall<'a, C, NC, A> {
        self._score_tag = Some(new_value.to_string());
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> ScoreSubmitCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ScoreSubmitCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ScoreSubmitCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ScoreSubmitCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists the scores in a leaderboard, starting from the top.
///
/// A builder for the *list* method supported by a *score* resource.
/// It is not used directly, but through a `ScoreMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.scores().list("leaderboardId", "collection", "timeSpan")
///              .page_token("aliquyam")
///              .max_results(-24)
///              .language("tempor")
///              .doit();
/// # }
/// ```
pub struct ScoreListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _leaderboard_id: String,
    _collection: String,
    _time_span: String,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ScoreListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ScoreListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LeaderboardScores)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.scores.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("leaderboardId", self._leaderboard_id.to_string()));
        params.push(("collection", self._collection.to_string()));
        params.push(("timeSpan", self._time_span.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "leaderboardId", "collection", "timeSpan", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/leaderboards/{leaderboardId}/scores/{collection}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{leaderboardId}", "leaderboardId"), ("{collection}", "collection")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["leaderboardId", "collection"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *leaderboard id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the leaderboard.    
    pub fn leaderboard_id(mut self, new_value: &str) -> ScoreListCall<'a, C, NC, A> {
        self._leaderboard_id = new_value.to_string();
        self
    }
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The collection of scores you're requesting.    
    pub fn collection(mut self, new_value: &str) -> ScoreListCall<'a, C, NC, A> {
        self._collection = new_value.to_string();
        self
    }
    /// Sets the *time span* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The time span for the scores and ranks you're requesting.    
    pub fn time_span(mut self, new_value: &str) -> ScoreListCall<'a, C, NC, A> {
        self._time_span = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> ScoreListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of leaderboard scores to return in the response. For any response, the actual number of leaderboard scores returned may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> ScoreListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> ScoreListCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ScoreListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ScoreListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ScoreListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Get high scores, and optionally ranks, in leaderboards for the currently authenticated player. For a specific time span, leaderboardId can be set to ALL to retrieve data for all leaderboards in a given time span.
/// NOTE: You cannot ask for 'ALL' leaderboards and 'ALL' timeSpans in the same request; only one parameter may be set to 'ALL'.
///
/// A builder for the *get* method supported by a *score* resource.
/// It is not used directly, but through a `ScoreMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.scores().get("playerId", "leaderboardId", "timeSpan")
///              .page_token("aliquyam")
///              .max_results(-69)
///              .language("sit")
///              .include_rank_type("diam")
///              .doit();
/// # }
/// ```
pub struct ScoreGetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _player_id: String,
    _leaderboard_id: String,
    _time_span: String,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _include_rank_type: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ScoreGetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ScoreGetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PlayerLeaderboardScoreListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.scores.get", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("playerId", self._player_id.to_string()));
        params.push(("leaderboardId", self._leaderboard_id.to_string()));
        params.push(("timeSpan", self._time_span.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        if let Some(value) = self._include_rank_type {
            params.push(("includeRankType", value.to_string()));
        }
        for &field in ["alt", "playerId", "leaderboardId", "timeSpan", "pageToken", "maxResults", "language", "includeRankType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/players/{playerId}/leaderboards/{leaderboardId}/scores/{timeSpan}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{playerId}", "playerId"), ("{leaderboardId}", "leaderboardId"), ("{timeSpan}", "timeSpan")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["playerId", "leaderboardId", "timeSpan"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *player id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A player ID. A value of me may be used in place of the authenticated player's ID.    
    pub fn player_id(mut self, new_value: &str) -> ScoreGetCall<'a, C, NC, A> {
        self._player_id = new_value.to_string();
        self
    }
    /// Sets the *leaderboard id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the leaderboard. Can be set to 'ALL' to retrieve data for all leaderboards for this application.    
    pub fn leaderboard_id(mut self, new_value: &str) -> ScoreGetCall<'a, C, NC, A> {
        self._leaderboard_id = new_value.to_string();
        self
    }
    /// Sets the *time span* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The time span for the scores and ranks you're requesting.    
    pub fn time_span(mut self, new_value: &str) -> ScoreGetCall<'a, C, NC, A> {
        self._time_span = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> ScoreGetCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of leaderboard scores to return in the response. For any response, the actual number of leaderboard scores returned may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> ScoreGetCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> ScoreGetCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *include rank type* query property to the given value.
    ///
    /// 
    /// The types of ranks to return. If the parameter is omitted, no ranks will be returned.    
    pub fn include_rank_type(mut self, new_value: &str) -> ScoreGetCall<'a, C, NC, A> {
        self._include_rank_type = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ScoreGetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ScoreGetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ScoreGetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Submits multiple scores to leaderboards.
///
/// A builder for the *submitMultiple* method supported by a *score* resource.
/// It is not used directly, but through a `ScoreMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::PlayerScoreSubmissionList;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PlayerScoreSubmissionList = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.scores().submit_multiple(&req)
///              .language("ut")
///              .doit();
/// # }
/// ```
pub struct ScoreSubmitMultipleCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: PlayerScoreSubmissionList,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ScoreSubmitMultipleCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ScoreSubmitMultipleCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PlayerScoreListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.scores.submitMultiple", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/leaderboards/scores".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &PlayerScoreSubmissionList) -> ScoreSubmitMultipleCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> ScoreSubmitMultipleCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ScoreSubmitMultipleCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ScoreSubmitMultipleCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ScoreSubmitMultipleCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists the scores in a leaderboard around (and including) a player's score.
///
/// A builder for the *listWindow* method supported by a *score* resource.
/// It is not used directly, but through a `ScoreMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.scores().list_window("leaderboardId", "collection", "timeSpan")
///              .return_top_if_absent(true)
///              .results_above(-13)
///              .page_token("diam")
///              .max_results(-71)
///              .language("est")
///              .doit();
/// # }
/// ```
pub struct ScoreListWindowCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _leaderboard_id: String,
    _collection: String,
    _time_span: String,
    _return_top_if_absent: Option<bool>,
    _results_above: Option<i32>,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ScoreListWindowCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ScoreListWindowCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LeaderboardScores)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.scores.listWindow", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((10 + self._additional_params.len()));
        params.push(("leaderboardId", self._leaderboard_id.to_string()));
        params.push(("collection", self._collection.to_string()));
        params.push(("timeSpan", self._time_span.to_string()));
        if let Some(value) = self._return_top_if_absent {
            params.push(("returnTopIfAbsent", value.to_string()));
        }
        if let Some(value) = self._results_above {
            params.push(("resultsAbove", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "leaderboardId", "collection", "timeSpan", "returnTopIfAbsent", "resultsAbove", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/leaderboards/{leaderboardId}/window/{collection}".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{leaderboardId}", "leaderboardId"), ("{collection}", "collection")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["leaderboardId", "collection"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *leaderboard id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the leaderboard.    
    pub fn leaderboard_id(mut self, new_value: &str) -> ScoreListWindowCall<'a, C, NC, A> {
        self._leaderboard_id = new_value.to_string();
        self
    }
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The collection of scores you're requesting.    
    pub fn collection(mut self, new_value: &str) -> ScoreListWindowCall<'a, C, NC, A> {
        self._collection = new_value.to_string();
        self
    }
    /// Sets the *time span* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The time span for the scores and ranks you're requesting.    
    pub fn time_span(mut self, new_value: &str) -> ScoreListWindowCall<'a, C, NC, A> {
        self._time_span = new_value.to_string();
        self
    }
    /// Sets the *return top if absent* query property to the given value.
    ///
    /// 
    /// True if the top scores should be returned when the player is not in the leaderboard. Defaults to true.    
    pub fn return_top_if_absent(mut self, new_value: bool) -> ScoreListWindowCall<'a, C, NC, A> {
        self._return_top_if_absent = Some(new_value);
        self
    }
    /// Sets the *results above* query property to the given value.
    ///
    /// 
    /// The preferred number of scores to return above the player's score. More scores may be returned if the player is at the bottom of the leaderboard; fewer may be returned if the player is at the top. Must be less than or equal to maxResults.    
    pub fn results_above(mut self, new_value: i32) -> ScoreListWindowCall<'a, C, NC, A> {
        self._results_above = Some(new_value);
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> ScoreListWindowCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of leaderboard scores to return in the response. For any response, the actual number of leaderboard scores returned may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> ScoreListWindowCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> ScoreListWindowCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ScoreListWindowCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ScoreListWindowCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ScoreListWindowCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Removes a push token for the current user and application. Removing a non-existent push token will report success.
///
/// A builder for the *remove* method supported by a *pushtoken* resource.
/// It is not used directly, but through a `PushtokenMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::PushTokenId;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PushTokenId = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pushtokens().remove(&req)
///              .doit();
/// # }
/// ```
pub struct PushtokenRemoveCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: PushTokenId,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PushtokenRemoveCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PushtokenRemoveCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.pushtokens.remove", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        for &field in [].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/games/v1/pushtokens/remove".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &PushTokenId) -> PushtokenRemoveCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PushtokenRemoveCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PushtokenRemoveCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PushtokenRemoveCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Registers a push token for the current user and application.
///
/// A builder for the *update* method supported by a *pushtoken* resource.
/// It is not used directly, but through a `PushtokenMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::PushToken;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PushToken = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pushtokens().update(&req)
///              .doit();
/// # }
/// ```
pub struct PushtokenUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: PushToken,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PushtokenUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PushtokenUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.pushtokens.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        for &field in [].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/games/v1/pushtokens".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &PushToken) -> PushtokenUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PushtokenUpdateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PushtokenUpdateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PushtokenUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Checks whether the games client is out of date.
///
/// A builder for the *check* method supported by a *revision* resource.
/// It is not used directly, but through a `RevisionMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.revisions().check("clientRevision")
///              .doit();
/// # }
/// ```
pub struct RevisionCheckCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _client_revision: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for RevisionCheckCall<'a, C, NC, A> {}

impl<'a, C, NC, A> RevisionCheckCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, RevisionCheckResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.revisions.check", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("clientRevision", self._client_revision.to_string()));
        for &field in ["alt", "clientRevision"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/revisions/check".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *client revision* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The revision of the client SDK used by your application. Format:
    /// [PLATFORM_TYPE]:[VERSION_NUMBER]. Possible values of PLATFORM_TYPE are:
    ///  
    /// - "ANDROID" - Client is running the Android SDK. 
    /// - "IOS" - Client is running the iOS SDK. 
    /// - "WEB_APP" - Client is running as a Web App.
    pub fn client_revision(mut self, new_value: &str) -> RevisionCheckCall<'a, C, NC, A> {
        self._client_revision = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> RevisionCheckCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> RevisionCheckCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> RevisionCheckCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a list of the event definitions in this application.
///
/// A builder for the *listDefinitions* method supported by a *event* resource.
/// It is not used directly, but through a `EventMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.events().list_definitions()
///              .page_token("invidunt")
///              .max_results(-87)
///              .language("dolores")
///              .doit();
/// # }
/// ```
pub struct EventListDefinitionCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for EventListDefinitionCall<'a, C, NC, A> {}

impl<'a, C, NC, A> EventListDefinitionCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, EventDefinitionListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.events.listDefinitions", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/eventDefinitions".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> EventListDefinitionCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of event definitions to return in the response, used for paging. For any response, the actual number of event definitions to return may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> EventListDefinitionCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> EventListDefinitionCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> EventListDefinitionCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> EventListDefinitionCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> EventListDefinitionCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Records a batch of changes to the number of times events have occurred for the currently authenticated user of this application.
///
/// A builder for the *record* method supported by a *event* resource.
/// It is not used directly, but through a `EventMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// use games1::EventRecordRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: EventRecordRequest = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.events().record(&req)
///              .language("eos")
///              .doit();
/// # }
/// ```
pub struct EventRecordCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _request: EventRecordRequest,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for EventRecordCall<'a, C, NC, A> {}

impl<'a, C, NC, A> EventRecordCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, EventUpdateResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.events.record", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/events".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &EventRecordRequest) -> EventRecordCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> EventRecordCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> EventRecordCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> EventRecordCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> EventRecordCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a list showing the current progress on events in this application for the currently authenticated user.
///
/// A builder for the *listByPlayer* method supported by a *event* resource.
/// It is not used directly, but through a `EventMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.events().list_by_player()
///              .page_token("voluptua.")
///              .max_results(-19)
///              .language("sed")
///              .doit();
/// # }
/// ```
pub struct EventListByPlayerCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for EventListByPlayerCall<'a, C, NC, A> {}

impl<'a, C, NC, A> EventListByPlayerCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PlayerEventListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.events.listByPlayer", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/events".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> EventListByPlayerCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of events to return in the response, used for paging. For any response, the actual number of events to return may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> EventListByPlayerCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> EventListByPlayerCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> EventListByPlayerCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> EventListByPlayerCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> EventListByPlayerCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Report that a reward for the milestone corresponding to milestoneId for the quest corresponding to questId has been claimed by the currently authorized user.
///
/// A builder for the *claim* method supported by a *questMilestone* resource.
/// It is not used directly, but through a `QuestMilestoneMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.quest_milestones().claim("questId", "milestoneId", "requestId")
///              .doit();
/// # }
/// ```
pub struct QuestMilestoneClaimCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _quest_id: String,
    _milestone_id: String,
    _request_id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for QuestMilestoneClaimCall<'a, C, NC, A> {}

impl<'a, C, NC, A> QuestMilestoneClaimCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.questMilestones.claim", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("questId", self._quest_id.to_string()));
        params.push(("milestoneId", self._milestone_id.to_string()));
        params.push(("requestId", self._request_id.to_string()));
        for &field in ["questId", "milestoneId", "requestId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/games/v1/quests/{questId}/milestones/{milestoneId}/claim".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        for &(find_this, param_name) in [("{questId}", "questId"), ("{milestoneId}", "milestoneId")].iter() {
                        let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["questId", "milestoneId"].iter() {
                for (index, &(ref name, _)) in params.iter().rev().enumerate() {
                    if name == param_name {
                        indices_for_removal.push(params.len() - index - 1);
                        break;
                    }
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *quest id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the quest.    
    pub fn quest_id(mut self, new_value: &str) -> QuestMilestoneClaimCall<'a, C, NC, A> {
        self._quest_id = new_value.to_string();
        self
    }
    /// Sets the *milestone id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The ID of the milestone.    
    pub fn milestone_id(mut self, new_value: &str) -> QuestMilestoneClaimCall<'a, C, NC, A> {
        self._milestone_id = new_value.to_string();
        self
    }
    /// Sets the *request id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// A numeric ID to ensure that the request is handled correctly across retries. Your client application must generate this ID randomly.    
    pub fn request_id(mut self, new_value: &str) -> QuestMilestoneClaimCall<'a, C, NC, A> {
        self._request_id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> QuestMilestoneClaimCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> QuestMilestoneClaimCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> QuestMilestoneClaimCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Lists all the achievement definitions for your application.
///
/// A builder for the *list* method supported by a *achievementDefinition* resource.
/// It is not used directly, but through a `AchievementDefinitionMethods`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-games1" as games1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use games1::Games;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Games::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.achievement_definitions().list()
///              .page_token("et")
///              .max_results(-48)
///              .language("diam")
///              .doit();
/// # }
/// ```
pub struct AchievementDefinitionListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a Games<C, NC, A>,
    _page_token: Option<String>,
    _max_results: Option<i32>,
    _language: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for AchievementDefinitionListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> AchievementDefinitionListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AchievementDefinitionsListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "games.achievementDefinitions.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._language {
            params.push(("language", value.to_string()));
        }
        for &field in ["alt", "pageToken", "maxResults", "language"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/games/v1/achievements".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveAppdata.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Result::MissingToken
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()

            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Result::HttpError(err)
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Result::Failure(res)
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Result::JsonDecodeError(err);
                            }
                        }
                    };

                    dlg.finished(true);
                    return Result::Success(result_value)
                }
            }
        }
    }


    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The token returned by the previous request.    
    pub fn page_token(mut self, new_value: &str) -> AchievementDefinitionListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maximum number of achievement resources to return in the response, used for paging. For any response, the actual number of achievement resources returned may be less than the specified maxResults.    
    pub fn max_results(mut self, new_value: i32) -> AchievementDefinitionListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *language* query property to the given value.
    ///
    /// 
    /// The preferred language to use for strings returned by this method.    
    pub fn language(mut self, new_value: &str) -> AchievementDefinitionListCall<'a, C, NC, A> {
        self._language = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AchievementDefinitionListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> AchievementDefinitionListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> AchievementDefinitionListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


