use reqwest::Client;
use reqwest::Url;
use serde_xml_rs::deserialize;

pub const BGG_NUM_GAMES: i32 = 100352;
pub const BGG_ENDPOINT: &'static str = "https://www.boardgamegeek.com/xmlapi/";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BoardGames {
    #[serde(rename = "$value")]
    items: Vec<BoardGame>,
    termsofuse: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Poll {
    name: String,
    title: String,
    totalvotes: Option<String>,
    #[serde(rename = "$value")]
    items: Option<Vec<PollItems>>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PollItems {
    numplayers: Option<String>,
    #[serde(rename = "$value")]
    results: Option<Vec<PollResult>>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PollResult {
    level: Option<String>,
    value: Option<String>,
    numvotes: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Name {
    primary: Option<bool>,
    sortindex: Option<String>,
    #[serde(rename = "$value")]
    value: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BoardGamePropertyIndexed {
    #[serde(rename = "$value")]
    value: String,
    objectid: String
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum BoardGameProperty {
    yearpublished(Option<String>),
    minplayers(Option<String>),
    maxplayers(Option<String>),
    playingtime(Option<String>),
    minplaytime(Option<String>),
    maxplaytime(Option<String>),
    age(Option<String>),
    name(Name),
    description(String),
    thumbnail(String),
    image(String),
    poll(Poll),
    boardgamefamily(BoardGamePropertyIndexed),
    boardgamecategory(BoardGamePropertyIndexed),
    boardgamedesigner(BoardGamePropertyIndexed),
    boardgamepublisher(BoardGamePropertyIndexed),
    boardgameversion(BoardGamePropertyIndexed),
    boardgameimplementation(BoardGamePropertyIndexed),
    boardgameartist(BoardGamePropertyIndexed),
    boardgamesubdomain(BoardGamePropertyIndexed),
    boardgamemechanic(BoardGamePropertyIndexed),
    boardgamepodcastepisode(BoardGamePropertyIndexed),
    boardgameexpansion(BoardGamePropertyIndexed),
    boardgamehonor(BoardGamePropertyIndexed),
    boardgame(BoardGamePropertyIndexed),
    videogamegenre(BoardGamePropertyIndexed),
    videogameversion(BoardGamePropertyIndexed),
    videogametheme(BoardGamePropertyIndexed),
    videogamedeveloper(BoardGamePropertyIndexed),
    videogamepublisher(BoardGamePropertyIndexed),
    videogamemode(BoardGamePropertyIndexed),
    videogameseries(BoardGamePropertyIndexed),
    videogamehonor(BoardGamePropertyIndexed),
    videogamecompilation(BoardGamePropertyIndexed),
    videogameplatform(BoardGamePropertyIndexed),
    videogameexpansion(BoardGamePropertyIndexed),
    videogamefranchise(BoardGamePropertyIndexed),
    videogame(BoardGamePropertyIndexed),
    rpgdesigner(BoardGamePropertyIndexed),
    rpgitemversion(BoardGamePropertyIndexed),
    rpgcategory(BoardGamePropertyIndexed),
    rpgissuearticle(BoardGamePropertyIndexed),
    rpgpublisher(BoardGamePropertyIndexed),
    rpgproducer(BoardGamePropertyIndexed),
    rpgartist(BoardGamePropertyIndexed),
    rpggenre(BoardGamePropertyIndexed),
    rpgsetting(BoardGamePropertyIndexed),
    rpgseries(BoardGamePropertyIndexed),
    rpgissueversion(BoardGamePropertyIndexed),
    rpgissue(BoardGamePropertyIndexed),
    rpghonor(BoardGamePropertyIndexed),
    rpg(BoardGamePropertyIndexed),
    error(String)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BoardGame {
    objectid: Option<String>,
    subtypemismatch: Option<bool>,
    #[serde(rename = "$value")]
    properties: Vec<BoardGameProperty>
}

pub fn get_game(id: i32) -> BoardGame {
    // Build URL
    let path = format!("/boardgame/{}", id);
    let uri_string = format!("{}{}", BGG_ENDPOINT, path);
    let uri = Url::parse(&uri_string).expect("Error parsing URL.");

    // Send GET request
    let client = Client::new();
    let response_body = client.get(uri).send().expect("Error sending GET.").text().expect("Error getting response text.");

    println!("{}", response_body);

    let mut games: BoardGames = deserialize(response_body.as_bytes()).unwrap();

    // Return found game
    games.items.pop().unwrap()
}
