// infer_schema!("dotenv:DATABASE_URL");
table! {
    users {
        id -> Integer,
        name -> VarChar,
        wins -> Integer,
        losses -> Integer,
        elo -> Integer,
        draws -> Integer,
    }
}
table! {
    games {
        id -> Integer,
        winner_id -> Integer,
        loser_id -> Integer,
        winner_elo -> Integer,
        loser_elo -> Integer,
        winner_score -> Integer,
        loser_score -> Integer,
        date -> BigInt,
        draw -> Bool,
    }
}
