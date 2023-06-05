#[cfg(feature = "json-storage")]
mod json_storage_usage {
    use futures::executor::block_on;
    use gluesql::prelude::Payload;
    use gluesql::{json_storage::JsonStorage, prelude::Glue};
    use gluesql_core::store::StoreMut;

    pub fn run() {
        json_basic();
        json_basic_async();
    }

    fn json_basic() {
        let path = "../data/";
        let json_storage = JsonStorage::new(path).expect("Something went wrong!");
        let mut glue = Glue::new(json_storage);

        block_on(glue.storage.delete_schema("User")).expect("fail to delete User schema");

        // Create User Table
        let create_query = "
          CREATE TABLE User (
            userId INT NOT NULL,
            userName TEXT NOT NULL
          );
        ";
        match glue.execute(create_query) {
            Ok(payloads) => {
                print_payload(payloads);
            }
            Err(_) => panic!("fail to create User Table into the JsonStorage"),
        }

        // Insert Rows Into User Table
        let insert_query = "
          INSERT INTO User VALUES
            (1, 'Taegit'),
            (2, 'Alice'),
            (3, 'Bob');
        ";
        match glue.execute(insert_query) {
            Ok(payloads) => {
                print_payload(payloads);
            }
            Err(_) => panic!("fail to insert rows into User table"),
        }

        // Delete One Row From User Table
        let delete_query = "
          DELETE FROM User WHERE userId=2;
        ";
        match glue.execute(delete_query) {
            Ok(payloads) => {
                print_payload(payloads);
            }
            Err(_) => panic!("fail to delete row from User table"),
        }

        // Select Rows From User Table
        let select_query = "
            SELECT * FROM User;
        ";
        match glue.execute(select_query) {
            Ok(payloads) => {
                print_payload(payloads);
            }
            Err(_) => panic!("fail to select rows from User table"),
        }

        block_on(glue.storage.delete_schema("User")).expect("fail to delete User schema");
    }

    fn json_basic_async() {
        let path = "../data/";
        let json_storage = JsonStorage::new(path).expect("Something went wrong!");
        let mut glue = Glue::new(json_storage);

        block_on(glue.storage.delete_schema("User")).expect("fail to delete User schema");

        // Create User Table
        let create_query = "
          CREATE TABLE User (
            userId INT NOT NULL,
            userName TEXT NOT NULL
          );
        ";
        block_on(async {
            match glue.execute_async(create_query).await {
                Ok(payloads) => {
                    print_payload(payloads);
                }
                Err(_) => panic!("fail to create User Table into the JsonStorage"),
            }
        });

        // Insert Rows Into User Table
        let insert_query = "
          INSERT INTO User VALUES
            (1, 'Taegit'),
            (2, 'Alice'),
            (3, 'Bob');
        ";
        block_on(async {
            match glue.execute_async(insert_query).await {
                Ok(payloads) => {
                    print_payload(payloads);
                }
                Err(_) => panic!("fail to insert rows into User table"),
            }
        });

        // Delete One Row From User Table
        let delete_query = "
          DELETE FROM User WHERE userId=2;
        ";
        block_on(async {
            match glue.execute_async(delete_query).await {
                Ok(payloads) => {
                    print_payload(payloads);
                }
                Err(_) => panic!("fail to delete row from User table"),
            }
        });

        // Select Rows From User Table
        let select_query = "
            SELECT * FROM User;
        ";
        block_on(async {
            match glue.execute_async(select_query).await {
                Ok(payloads) => {
                    print_payload(payloads);
                }
                Err(_) => panic!("fail to select rows from User table"),
            }
        });

        block_on(glue.storage.delete_schema("User")).expect("fail to delete User schema");
    }

    fn print_payload(payloads: Vec<Payload>) {
        for payload in payloads.iter() {
            match payload {
                Payload::Select { labels, rows } => {
                    println!("[Select({})]' finished", rows.len());
                    println!("- Labels: {}", labels.join(","));
                    for (idx, row) in rows.iter().enumerate() {
                        println!("- Row {}: {:?}", idx + 1, row);
                    }
                }
                _ => {
                    println!("[{:?}] finished", payload);
                }
            }
        }
    }
}

fn main() {
    #[cfg(feature = "json-storage")]
    json_storage_usage::run();
}
