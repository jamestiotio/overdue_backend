use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    errors::CustomError,
    models::{MaterialEntry, MaterialValueEntry},
};

pub async fn get_values_of_materials(
    client: &Client,
) -> Result<Vec<MaterialValueEntry>, CustomError> {
    let statement = client
        .prepare("SELECT name, value FROM material ORDER BY id ASC")
        .await
        .map_err(|_err| CustomError::DbError)?;

    let result = client
        .query(&statement, &[])
        .await
        .map_err(|_err| CustomError::DbError)?
        .iter()
        .map(|row| MaterialValueEntry::from_row_ref(row).expect("error mapping material values"))
        .collect::<Vec<MaterialValueEntry>>();

    Ok(result)
}

pub async fn check_if_materials_total_value_sum_up_to_score(
    score: i32,
    materials: Vec<MaterialEntry>,
    bonus: i32,
    values: Vec<MaterialValueEntry>,
) -> Result<bool, CustomError> {
    let mut sum: i32 = 0;

    // Ensure that materials array is not strangely constructed/parsed.
    if (materials.is_empty() && materials.len() > 0) || (materials.capacity() < materials.len()) {
        return Ok(false);
    }

    // Validate that length of materials vector does not exceed the values vector.
    if materials.len() > values.len() {
        return Ok(false);
    }

    // Check that materials vector does not contain any materials not specified in
    // the currently-existing material names in the database.
    for material in materials.iter() {
        if !(&values).iter().any(|value| value.name == material.name) {
            return Ok(false);
        }
    }

    // Check that materials vector does not contain any duplicate items (i.e.,
    // multiple material entries with the same name). This is an attempt to avoid
    // potential/possible race conditions.
    let mut already_seen = vec![];
    for material in materials.iter() {
        match already_seen.contains(&material.name) {
            true => return Ok(false),
            _ => already_seen.push(material.name.clone()),
        }
    }

    // Prevent submission of scores with non-zero bonuses but with no materials.
    if materials.is_empty() && bonus > 0 {
        return Ok(false);
    }

    sum += bonus;

    for material in materials.iter() {
        // Nested inner loop is (generally) better/less expensive in terms of
        // performance compared to keep communicating with and hitting the database
        // (especially for our case). Exceptions and special cases exist, of
        // course, but for our case, we will follow the general consensus of best
        // practices.
        for value in values.iter() {
            if material.name == value.name {
                sum += value.value * material.quantity;
                break;
            }
        }
    }

    Ok(score == sum)
}

// This implementation of dynamically constructing the SQL query on the Rust
// side works more performantly compared to a fixed for loop of SQL statements
// (execution time is much faster for a single bigger nested query compared to
// multiple smaller queries).
pub async fn add_materials_to_aggregate(
    client: &Client,
    materials: Vec<MaterialEntry>,
) -> Result<bool, CustomError> {
    // Initialize mutable SQL statement to be used for database update (variable name courtesy of Filbert - https://github.com/FolkLoreee).
    let mut nomnom: String =
        "UPDATE material AS m SET quantity = c.quantity FROM (VALUES".to_string();

    for i in (1..materials.len() * 2 + 1).step_by(2) {
        nomnom.push_str(
            &format!(
                " (${}, (SELECT quantity FROM material WHERE name = ${}) + ${}),",
                i,
                i,
                i + 1
            )
            .to_string(),
        );
    }

    // Remove the last final comma character.
    nomnom.pop();

    nomnom.push_str(") AS c(name, quantity) WHERE c.name = m.name");

    // Pre-allocate the capacity limit.
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
        Vec::with_capacity(materials.len() * 2);

    for material in materials.iter() {
        params.push(&material.name);
        params.push(&material.quantity);
    }

    // Prepare actual SQL statement.
    let statement = client
        .prepare(&nomnom)
        .await
        .map_err(|_err| CustomError::DbError)?;

    // Setting the inputs as parameters for the query statement this way (SQL query
    // parameterization) prevents SQL injection.
    let result = client
        .query(&statement, &params[..])
        .await
        .map_err(|_err| CustomError::DbError)
        .is_ok();

    Ok(result)
}

// Define unit tests for the payload validation logic.
#[cfg(test)]
mod tests {
    use super::check_if_materials_total_value_sum_up_to_score;
    use crate::models::{MaterialEntry, MaterialValueEntry};

    // Define macro to await async function to return result.
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_validate_zero_score_with_no_materials() {
        let score: i32 = 0;
        let materials: Vec<MaterialEntry> = vec![];
        let bonus: i32 = 0;
        let values: Vec<MaterialValueEntry> = vec![];

        let result: bool = aw!(check_if_materials_total_value_sum_up_to_score(
            score, materials, bonus, values
        ))
        .expect("error running score validator function");

        assert_eq!(result, true);
    }

    #[test]
    fn test_validate_some_score_with_no_materials() {
        let score: i32 = 20;
        let materials: Vec<MaterialEntry> = vec![];
        let bonus: i32 = 0;
        let values: Vec<MaterialValueEntry> = vec![];

        let result: bool = aw!(check_if_materials_total_value_sum_up_to_score(
            score, materials, bonus, values
        ))
        .expect("error running score validator function");

        assert_eq!(result, false);
    }

    #[test]
    fn test_validate_some_bonus_with_no_materials() {
        let score: i32 = 20;
        let materials: Vec<MaterialEntry> = vec![];
        let bonus: i32 = 20;
        let values: Vec<MaterialValueEntry> = vec![];

        let result: bool = aw!(check_if_materials_total_value_sum_up_to_score(
            score, materials, bonus, values
        ))
        .expect("error running score validator function");

        assert_eq!(result, false);
    }

    #[test]
    fn test_validate_zero_score_with_some_materials() {
        let score: i32 = 0;
        let materials: Vec<MaterialEntry> = vec![MaterialEntry {
            name: "portalGun".to_string(),
            quantity: 2,
        }];
        let bonus: i32 = 0;
        let values: Vec<MaterialValueEntry> = vec![MaterialValueEntry {
            name: "portalGun".to_string(),
            value: 10,
        }];

        let result: bool = aw!(check_if_materials_total_value_sum_up_to_score(
            score, materials, bonus, values
        ))
        .expect("error running score validator function");

        assert_eq!(result, false);
    }

    #[test]
    fn test_validate_unmatching_materials_with_values() {
        let score: i32 = 20;
        let materials: Vec<MaterialEntry> = vec![
            MaterialEntry {
                name: "portalGun".to_string(),
                quantity: 2,
            },
            MaterialEntry {
                name: "batarang".to_string(),
                quantity: 4,
            },
        ];
        let bonus: i32 = 0;
        let values: Vec<MaterialValueEntry> = vec![
            MaterialValueEntry {
                name: "lovePotion".to_string(),
                value: 20,
            },
            MaterialValueEntry {
                name: "portalGun".to_string(),
                value: 10,
            },
        ];

        let result: bool = aw!(check_if_materials_total_value_sum_up_to_score(
            score, materials, bonus, values
        ))
        .expect("error running score validator function");

        assert_eq!(result, false);
    }

    #[test]
    fn test_validate_more_materials_than_values() {
        let score: i32 = 60;
        let materials: Vec<MaterialEntry> = vec![
            MaterialEntry {
                name: "lovePotion".to_string(),
                quantity: 3,
            },
            MaterialEntry {
                name: "portalGun".to_string(),
                quantity: 2,
            },
        ];
        let bonus: i32 = 0;
        let values: Vec<MaterialValueEntry> = vec![MaterialValueEntry {
            name: "lovePotion".to_string(),
            value: 20,
        }];

        let result: bool = aw!(check_if_materials_total_value_sum_up_to_score(
            score, materials, bonus, values
        ))
        .expect("error running score validator function");

        assert_eq!(result, false);
    }

    #[test]
    fn test_validate_materials_do_not_add_up_to_score() {
        let score: i32 = 999999;
        let materials: Vec<MaterialEntry> = vec![
            MaterialEntry {
                name: "portalGun".to_string(),
                quantity: 5,
            },
            MaterialEntry {
                name: "lovePotion".to_string(),
                quantity: 5,
            },
        ];
        let bonus: i32 = 0;
        let values: Vec<MaterialValueEntry> = vec![
            MaterialValueEntry {
                name: "lovePotion".to_string(),
                value: 20,
            },
            MaterialValueEntry {
                name: "portalGun".to_string(),
                value: 10,
            },
            MaterialValueEntry {
                name: "batarang".to_string(),
                value: 30,
            },
        ];

        let result: bool = aw!(check_if_materials_total_value_sum_up_to_score(
            score, materials, bonus, values
        ))
        .expect("error running score validator function");

        assert_eq!(result, false);
    }

    #[test]
    fn test_validate_duplicate_material_names() {
        let score: i32 = 110;
        let materials: Vec<MaterialEntry> = vec![
            MaterialEntry {
                name: "portalGun".to_string(),
                quantity: 4,
            },
            MaterialEntry {
                name: "portalGun".to_string(),
                quantity: 7,
            },
        ];
        let bonus: i32 = 0;
        let values: Vec<MaterialValueEntry> = vec![
            MaterialValueEntry {
                name: "lovePotion".to_string(),
                value: 20,
            },
            MaterialValueEntry {
                name: "portalGun".to_string(),
                value: 10,
            },
            MaterialValueEntry {
                name: "batarang".to_string(),
                value: 30,
            },
        ];

        let result: bool = aw!(check_if_materials_total_value_sum_up_to_score(
            score, materials, bonus, values
        ))
        .expect("error running score validator function");

        assert_eq!(result, false);
    }

    #[test]
    fn test_validate_valid_materials_with_no_bonus() {
        let score: i32 = 180;
        let materials: Vec<MaterialEntry> = vec![
            MaterialEntry {
                name: "portalGun".to_string(),
                quantity: 4,
            },
            MaterialEntry {
                name: "lovePotion".to_string(),
                quantity: 7,
            },
        ];
        let bonus: i32 = 0;
        let values: Vec<MaterialValueEntry> = vec![
            MaterialValueEntry {
                name: "lovePotion".to_string(),
                value: 20,
            },
            MaterialValueEntry {
                name: "portalGun".to_string(),
                value: 10,
            },
            MaterialValueEntry {
                name: "batarang".to_string(),
                value: 30,
            },
        ];

        let result: bool = aw!(check_if_materials_total_value_sum_up_to_score(
            score, materials, bonus, values
        ))
        .expect("error running score validator function");

        assert_eq!(result, true);
    }

    #[test]
    fn test_validate_valid_materials_with_some_bonus() {
        let score: i32 = 183;
        let materials: Vec<MaterialEntry> = vec![
            MaterialEntry {
                name: "portalGun".to_string(),
                quantity: 4,
            },
            MaterialEntry {
                name: "lovePotion".to_string(),
                quantity: 7,
            },
        ];
        let bonus: i32 = 3;
        let values: Vec<MaterialValueEntry> = vec![
            MaterialValueEntry {
                name: "lovePotion".to_string(),
                value: 20,
            },
            MaterialValueEntry {
                name: "portalGun".to_string(),
                value: 10,
            },
            MaterialValueEntry {
                name: "batarang".to_string(),
                value: 30,
            },
        ];

        let result: bool = aw!(check_if_materials_total_value_sum_up_to_score(
            score, materials, bonus, values
        ))
        .expect("error running score validator function");

        assert_eq!(result, true);
    }
}
