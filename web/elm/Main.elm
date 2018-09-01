module Main exposing (main)

import Html exposing (..)
import Html.Events exposing (..)
import Html.Attributes exposing (..)

import Http
--import Model exposing (..)
--import Update exposing (update)
--import View exposing (view)

type alias Game =
    {
      name: String
    }

type Msg
  = GetGame
  | FetchRandomGameCompleted (Result Http.Error String)

main : Program Never Game Msg
main =
  Html.program
    { init = init
    , update = update
    , subscriptions = \_ -> Sub.none
    , view = view
    }

init : (Maybe.Maybe Game, Cmd Msg)
init =
  ( Nothing, fetchRandomGameCmd )

view : Game -> Html Msg
view game =
  div [ class "container" ] [
    h2 [ class "text-center" ] [ text "BoardGamerCon" ]
    , p [ class "text-center" ] [
      text "Your source for custom Board Game convention agendas!",
      p [ class "text-center" ][
        button [ class "btn btn-success", onClick GetGame ] [ text "Get a game from BGG!" ]
      ]
    ]
    , div [ class "board-game-content"] [
      p [ class "board-game-title"] [
        label [] [ game.properties.name ]
        , text [ game ]
      ]
    ]
  ]

update : Msg -> Game -> (Game, Cmd Msg)

update msg game =
  case msg of
    GetGame ->
      ( game, fetchRandomGameCmd )
    FetchRandomGameCompleted result ->
      fetchRandomGameCompleted game result

randomGameUrl : String
randomGameUrl =
  "/api/"

fetchRandomGame : Http.Request String
fetchRandomGame =
  Http.getString randomGameUrl

fetchRandomGameCmd : Cmd Msg
fetchRandomGameCmd =
  Http.send FetchRandomGameCompleted fetchRandomGame

fetchRandomGameCompleted : Game -> Result Http.Error String -> ( Game, Cmd Msg )
fetchRandomGameCompleted game result =
  case result of
    Ok newGame ->
      ( { game | name = newGame }, Cmd.none )
    Err _ ->
      ( game, Cmd.none )
