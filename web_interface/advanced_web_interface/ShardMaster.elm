-- ShardMaster.elm
module ShardMaster exposing (main)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as Decode
import Json.Encode as Encode

type alias Shard =
  { id : Int
 , name : String
 , config : Config
  }

type alias Config =
  { database : String
 , cache : String
  }

type Msg
  = GetShards
  | GetShard Int
  | ReceiveShards (Result Http.Error (List Shard))
  | ReceiveShard (Result Http.Error Shard)

init : ( Model, Cmd Msg )
init =
  ( { shards = [], selectedShard = Nothing }, getShards )

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
  case msg of
    GetShards ->
      ( model, getShards )

    GetShard id ->
     ( model, getShard id )

    ReceiveShards result ->
      case result of
        Ok shards ->
          ( { model | shards = shards }, Cmd.none )

        Err error ->
          ( model, Debug.log error (ReceiveShards result) )

    ReceiveShard result ->
      case result of
        Ok shard ->
          ( { model | selectedShard = Just shard }, Cmd.none )

        Err error ->
          ( model, Debug.log error (ReceiveShard result) )

getShards : Cmd Msg
getShards =
  Http.get
    { url = "http://localhost:8080/shards"
    , expect = Http.expectJson Decode.list Shard
    , onSuccess = ReceiveShards
    , onFailure = ReceiveShards
    }

getShard : Int -> Cmd Msg
getShard id =
  Http.get
    { url = "http://localhost:8080/shards/$id"
    , expect = Http.expectJson Decode.object Shard
    , onSuccess = ReceiveShard
    , onFailure = ReceiveShard
    }

view : Model -> Html Msg
view model =
  div []
    [ h1 [] [ text "ShardMaster Web Interface" ]
    , ul []
        ( List.map viewShard model.shards
        ++ [ li [] [ button [ onClick GetShard model.selectedShard.id ] [ text "Reload" ] ] ]
        )
    , maybe
        { init = Nothing
        , view = viewShard
        , update = updateShard
        }
        model.selectedShard
    ]

viewShard : Shard -> Html Msg
viewShard shard =
  li []
    [ button [ onClick (GetShard shard.id) ] [ text shard.name ] ]

updateShard : Shard -> Model -> ( Model, Cmd Msg )
updateShard shard model =
  ( { model | selectedShard = Just shard }, Cmd.none )

main : Program () Model Msg
main =
  Browser.sandbox
    { init = init
    , view = view
    , update = update
    }
