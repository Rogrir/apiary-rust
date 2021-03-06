use anyhow::Result;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

/*
CREATE TABLE APIARY (
  ID             SERIAL PRIMARY KEY,
  LOCALIZATION   varchar(255),
  INFORMATION    varchar(255));
*/

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
pub struct Apiary {
  pub id: i32,
  localization: Option<String>,
  information: Option<String>,
}

impl Apiary {
  pub async fn create(
    pool: &PgPool,
    localization: Option<&String>,
    information: Option<&String>,
  ) -> Result<Apiary> {
    let row = sqlx::query!(
      "INSERT INTO apiary(localization, information) VALUES ($1, $2) RETURNING id",
      localization,
      information
    )
    .fetch_one(pool)
    .await?;

    Ok(Apiary {
      id: row.id,
      localization: localization.cloned(),
      information: information.cloned(),
    })
  }

  pub async fn read_one(pool: &PgPool, id: &str) -> Result<Apiary> {
    let row: Apiary = sqlx::query_as!(
      Apiary,
      "SELECT * FROM apiary WHERE id = $1",
      id.parse::<i32>().unwrap()
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
  }

  pub async fn read_all(pool: &PgPool) -> Result<Vec<Apiary>> {
    let rows = sqlx::query_as!(Apiary, "SELECT * FROM apiary")
      .fetch_all(pool)
      .await?;

    Ok(rows)
  }

  pub async fn update(
    pool: &PgPool,
    id: &str,
    localization: Option<&String>,
    information: Option<&String>,
  ) -> Result<Apiary> {
    let temp = Apiary::read_one(pool, id).await?;
    let result_localization = match localization {
      Some(x) => Some(x),
      None => temp.localization.as_ref(),
    };
    let result_information = match information {
      Some(x) => Some(x),
      None => temp.information.as_ref(),
    };
    sqlx::query!(
      "UPDATE apiary SET localization=$1, information=$2 WHERE id = $3",
      result_localization,
      result_information,
      id.parse::<i32>().unwrap()
    )
    .execute(pool)
    .await?;

    Ok(Apiary::read_one(pool, id).await?)
  }

  pub async fn delete(pool: &PgPool, id: &str) -> Result<()> {
    sqlx::query!(
      "DELETE FROM apiary WHERE id = $1",
      id.parse::<i32>().unwrap()
    )
    .execute(pool)
    .await?;

    Ok(())
  }
}
