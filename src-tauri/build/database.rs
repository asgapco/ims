//! A module which provides functions to handle the generation of sql scripts for each table in the
//! database.

use std::{cell::OnceCell, io::Write};

/// A constant holding the code column code for the tables.
const CODE_COLUMN: &str = "CODE INT PRIMARY KEY AUTO_INCREMENT";
/// A constant holding the name of the folder which will hold the sql scripts.
const SCRIPTS_FOLDER: &str = "sql-scripts";
/// A constant for the environment variable name.
const PACKAGE_ENVIRONMENT_VARIABLE: &str = "PKG_ENV";
/// A constant for the `prod` value of the `pkg_env` environment variable.
const PRODUCTION_PKG_ENV_VARIABLE_VALUE: &str = "prod";
/// A constant for the `dev` value of the `pkg_env` environment variable.
const DEVELOPEMENT_PKG_ENV_VARIABLE_VALUE: &str = "dev";
/// A constant holding the list of all the table names.
const TABLES_NAME: [&str; 7] = [
    "ITEM_BRANDS",
    "ITEM_GROUPS",
    "ITEM_SUBGROUPS",
    "ITEM_SUBGROUPS1",
    "ITEMS",
    "UNITS",
    "YEAR_SOURCE",
];
/// A constant holding all the table sql queries.
const SQL_TABLE_QUERIES: OnceCell<[String; 7]> = OnceCell::new();

/// A function which generates all the table sql script files.
///
/// # Error
///
/// It returns an IO error if any file operations fail.
pub fn generate_database_tables() -> Result<(), Box<dyn std::error::Error>> {
    let pkg_env_var: String = std::env::var(PACKAGE_ENVIRONMENT_VARIABLE)
        .unwrap_or(String::from(DEVELOPEMENT_PKG_ENV_VARIABLE_VALUE));
    match pkg_env_var.to_lowercase().as_str() {
        PRODUCTION_PKG_ENV_VARIABLE_VALUE => {
            todo!();
        }
        _ => {
            println!("***Generating sql scripts in dev mode***");

            if let Ok(_) = std::fs::create_dir(SCRIPTS_FOLDER) {
                println!("-> Making directory sql-scripts");
            };

            println!("-> Changing directory to sql-scripts");
            std::env::set_current_dir(SCRIPTS_FOLDER)?;

            println!("-> Generating sql script for dropping all the tables that exists");
            if let Ok(mut drop_tables_file) = std::fs::File::create_new("drop-tables.sql") {
                for table_name in TABLES_NAME {
                    drop_tables_file
                        .write(format!("DROP TABLE IF EXISTS {}; ", table_name).as_bytes())?;
                }
            }

            // Initialize the constant with all the sql table queries.
            let sql_table_queries = SQL_TABLE_QUERIES.get_or_init(||
                    [
                         format!("CREATE TABLE {}({CODE_COLUMN}, ANAME VARCHAR(60), ENAME VARCHAR(60) NOT NULL)", TABLES_NAME[0]),
                         format!("CREATE TABLE {}({CODE_COLUMN}, ANAME VARCHAR(60), ENAME VARCHAR(60) NOT NULL)", TABLES_NAME[1]),
                         format!("CREATE TABLE {}({CODE_COLUMN}, ANAME VARCHAR(60) NOT NULL, ENAME VARCHAR(60), GROUP_CODE INT NOT NULL, FOREIGN KEY(GROUP_CODE) REFERENCES ITEM_GROUPS(CODE)", TABLES_NAME[2]),
                         format!("CREATE TABLE {}({CODE_COLUMN}, ANAME VARCHAR(60), ENAME VARCHAR(60) NOT NULL, SUBGROUP_CODE INT NOT NULL, FOREIGN KEY(SUBGROUP_CODE) REFERENCES ITEM_SUBGROUPS(CODE))", TABLES_NAME[3]),
                         format!("CREATE TABLE {}({CODE_COLUMN}, ANAME VARCHAR(60), ENAME VARCHAR(60) NOT NULL, GROUP_CODE INT NOT NULL, SUBGROUP_CODE INT, SUBGROUP1_CODE INT, UNIT_CODE INT NOT NULL, UNIT_CODE_2 INT, UNIT_CODE_3 INT, BRAND_CODE INT, ITEM_DESC VARCHAR(100), AVAILABLE_QTY INT DEFAULT 0, FOREIGN KEY(UNIT_CODE) REFERENCES UNITS(CODE), FOREIGN KEY(UNIT_CODE_2) REFERENCES UNITS(CODE), FOREIGN KEY(UNIT_CODE_3) REFERENCES UNITS(CODE), FOREIGN KEY(GROUP_CODE) REFERENCES ITEM_GROUPS(CODE), FOREIGN KEY(SUBGROUP_CODE) REFERENCES ITEM_SUBGROUPS(CODE), FOREIGN KEY(SUBGROUP1_CODE) REFERENCES ITEM_SUBGROUPS1(CODE), FOREIGN KEY(BRAND_CODE) REFERENCES ITEM_BRANDS(CODE))", TABLES_NAME[4]),
                         format!("CREATE TABLE {}({CODE_COLUMN}, ANAME VARCHAR(18), ENAME VARCHAR(18) NOT NULL, UNIT_QUANTITY INT NOT NULL)", TABLES_NAME[5]),
                         format!("CREATE TABLE {}(CODE VARCHAR(30) PRIMARY KEY, ANAME VARCHAR(30), ENAME VARCHAR(30) NOT NULL, FROM_DATE DATE NOT NULL, TO_DATE DATE NOT NULL);", TABLES_NAME[6]),
                    ]
                ).clone();

            for (sql_table_query, table_name) in sql_table_queries.iter().zip(TABLES_NAME) {
                generate_table_script(&sql_table_query, table_name)?;
            }

            // Go outside of the `sql-scripts` directory. This is important because otherwise
            // the other build code expects the current working directory to be `src-tauri`
            // instead of `src-tauri/sql-scripts`.
            std::env::set_current_dir("..")?;
        }
    }

    Ok(())
}

/// A helper function which helps generate the file with their respective sql query.
///
/// # Arguments
///
/// * `sql_table_query` - It takes sql query as string.
/// * `table_name` - It takes the table name as string.
///
/// # Error
///
/// It returns IO error if any file operations fail.
fn generate_table_script(
    sql_table_query: &str,
    table_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "-> Generating {} table sql script",
        table_name.to_lowercase()
    );
    if let Ok(mut table_file) = std::fs::File::create_new(format!(
        "{}.sql",
        table_name.to_lowercase().replace("_", "-")
    )) {
        table_file.write_all(sql_table_query.as_bytes())?;
    }

    Ok(())
}
