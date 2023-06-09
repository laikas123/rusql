
use std::collections::HashMap;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ColType {
    RequiredStrCol,
    OptionalStrCol,
    RequiredIntCol,
    OptionalIntCol,
    Unknown,
}

//since there's a lot of checks 
//on whether Db is empty or not
//easier to add field with
//this type of enum
//for checks rather than
//tons of match statements
#[derive(PartialEq, Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DbStatus {
    Empty,
    Selected,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Db{
    pub name: String,
    pub tables: HashMap<String, DbTable>,
    pub status: DbStatus,
}


impl Db {

    pub fn table_mut(&mut self, table_name: String) -> Option<&mut DbTable> {
        self.tables.get_mut(&table_name)
    }

    pub fn pretty_print_tables(&self) {
        for (_, table) in &self.tables{
            table.pretty_print();
        }
    }

}

//although every entry in the table
//is stored as type string, ColType is
//there to keep track of whether int
//to string conversions need to happen
#[derive(Debug, Deserialize, Serialize)]
pub struct DbTable {
    pub name: String,

    //note the indices of each vector 
    //line up with each other


    //column names and types
    pub int_cols: Vec<(String, ColType)>,
    pub str_cols: Vec<(String, ColType)>,
    
    //data for each row
    pub int_rows: Vec<Vec<usize>>,
    pub str_rows: Vec<Vec<String>>,
}

static NULL_INT_COL: usize = 9999;



impl DbTable {

    //name = the name of the table
    //col_names_types = column names and types
    //import_str_rows = rows of str data to import
    //import_int_rows = rows of int data to import
    pub fn new(name: String, col_names_types: Vec<(String, ColType)>, import_str_rows: Vec<Vec<String>>, import_int_rows: Vec<Vec<usize>>) -> Option<Self> {


        //make sure that if rows are being imported that each column is covered
        //and that there's an equal amount of rows from all types
        if import_str_rows.len() != 0 || import_int_rows.len() != 0 {

            println!("Got here");

            //get length of first str row
            let len_str_row = import_str_rows[0].len();

            //make sure all of the str_rows have this same length
            let str_all_same_len = import_str_rows.iter().all(|elem| elem.len() == len_str_row);

            if !str_all_same_len {
                println!("Error, not all str_rows are same length");
                return None;
            }


            //get length of first int row
            let len_int_row = import_int_rows[0].len();

            //make sure all of the int_rows have this same length
            let int_all_same_len = import_int_rows.iter().all(|elem| elem.len() == len_int_row);

            if !int_all_same_len {
                println!("Error, not all int_rows are same length");
                return None;
            }

            //make sure the combined length matches total cols
            if (len_str_row + len_int_row) !=  col_names_types.len() {
                println!("Error, import data doesn't span all columns");
                return None;
            }

            //not equal total number of rows from each type
            if import_str_rows.len() != import_int_rows.len() {
                println!("Error, different number of rows for each type");
                return None;
            }
            
        }
       

        let mut int_cols = Vec::new();
        let mut str_cols = Vec::new();

        for (col_name, col_type) in col_names_types {
            if col_type == ColType::RequiredIntCol || col_type == ColType::OptionalIntCol {
                int_cols.push((col_name, col_type));
            }else if col_type == ColType::RequiredStrCol || col_type == ColType::OptionalStrCol{
                str_cols.push((col_name, col_type));
            }
        }


        let mut int_rows = Vec::new();
        let mut str_rows = Vec::new();

        

        for row in import_str_rows {
            let mut i = 0;
            let mut row_vec = Vec::new();
            for col_val in row {
                //If None is trying to be put somewhere it shouldn't abort all
                if col_val == "?" && str_cols[i].1 == ColType::RequiredStrCol {
                    println!("Error required column {} given None value", str_cols[i].0);
                    return None;
                }
                row_vec.push(col_val);

                i += 1;
            }

            str_rows.push(row_vec);
        }

        for row in import_int_rows {
            let mut i = 0;
            let mut row_vec = Vec::new();
            for col_val in row {
                //If None is trying to be put somewhere it shouldn't abort all
                if col_val == NULL_INT_COL && int_cols[i].1 == ColType::RequiredIntCol {
                    println!("Error required column {} given None value", int_cols[i].0);
                    return None;
                }
                row_vec.push(col_val);

                i += 1;
            }
            int_rows.push(row_vec);
        }

        Some(DbTable 
        {
            name: name,
            int_cols: int_cols,
            str_cols: str_cols,
            int_rows: int_rows,
            str_rows: str_rows,
        })


    }

    pub fn get_column_index_and_type(&self, col_name: &str) -> Option<(usize, ColType)> {
        
        //first check num cols (note .iter() iterates over reference so no move
        //occurs)
        let mut pos = self.int_cols.iter().position(|elem| elem.0 == col_name);
        if pos.is_some() {
            let ret_index = pos.unwrap();
            return Some((ret_index, self.int_cols[ret_index].1));
        }

        pos = self.str_cols.iter().position(|elem| elem.0 == col_name);
        if pos.is_some() {
            let ret_index = pos.unwrap();
            return Some((ret_index, self.str_cols[ret_index].1));
        }

        return None;



    }


    fn is_required_column(col_type: &ColType) -> bool {
        if *col_type == ColType::RequiredIntCol || *col_type == ColType::RequiredStrCol {
            return true;
        }else{
            return false;
        }
    }

    //returns the new row count if successful
    //otherwise returns -1
    pub fn insert(&mut self, import_str_rows: Vec<Vec<String>>, import_int_rows: Vec<Vec<usize>>) -> i32{
        

        //make sure that if rows are being imported that each column is covered
        //and that there's an equal amount of rows from all types
        if import_str_rows.len() != 0 || import_int_rows.len() != 0 {

            println!("Got here");

            //get length of first str row
            let len_str_row = import_str_rows[0].len();

            //make sure all of the str_rows have this same length
            let str_all_same_len = import_str_rows.iter().all(|elem| elem.len() == len_str_row);

            if !str_all_same_len {
                println!("Error, not all str_rows are same length");
                return -1;
            }


            //get length of first int row
            let len_int_row = import_int_rows[0].len();

            //make sure all of the int_rows have this same length
            let int_all_same_len = import_int_rows.iter().all(|elem| elem.len() == len_int_row);

            if !int_all_same_len {
                println!("Error, not all int_rows are same length");
                return -1;
            }

            println!("import string rows {:?}", import_str_rows);
            println!("import int rows {:?}", import_int_rows);

            //make sure the combined length matches total cols
            if len_str_row  != self.str_cols.len() || len_int_row != self.int_cols.len() {
                println!("Error, import data doesn't span all columns");
                return -1;
            }

            //not equal total number of rows from each type
            if import_str_rows.len() != import_int_rows.len() {
                println!("Error, different number of rows for each type");
                return -1;
            }
            
        }

        for row in import_str_rows {
            let mut i = 0;
            let mut row_vec = Vec::new();
            for col_val in row {
                //If None is trying to be put somewhere it shouldn't abort all
                if col_val == "?" && self.str_cols[i].1 == ColType::RequiredStrCol {
                    println!("Error required column {} given None value", self.str_cols[i].0);
                    return -1;
                }
                row_vec.push(col_val);

                i += 1;
            }

            self.str_rows.push(row_vec);
        }

        for row in import_int_rows {
            let mut i = 0;
            let mut row_vec = Vec::new();
            for col_val in row {
                //If None is trying to be put somewhere it shouldn't abort all
                if col_val == NULL_INT_COL && self.int_cols[i].1 == ColType::RequiredIntCol {
                    println!("Error required column {} given None value", self.int_cols[i].0);
                    return -1;
                }
                row_vec.push(col_val);

                i += 1;
            }
            self.int_rows.push(row_vec);
        }

        //could have also been self.str_rows.len()
        //but either will do since they are same
        return (self.int_rows.len()).try_into().unwrap();

    }

  


    pub fn pretty_print(&self) {
        println!("\nPRINTING TABLE:\n");
        println!("Table: {}", &self.name);
        print!("Cols: \n{:?} {:?}\n", &self.str_cols, &self.int_cols);
        print!("Rows: \n", );
        for i in 0..self.str_rows.len() {
            print!("{:?}{:?}\n", self.str_rows[i], self.int_rows[i]);
        } 
        // &self.str_rows.iter().for_each(|elem| { println!("{:?}", elem) });
        // println!("Int Cols: \n{:?}", &self.int_cols);
        // println!("Int Rows:");
        // &self.int_rows.iter().for_each(|elem| { println!("{:?}", elem) });
        println!("\n");
    }



}




#[test]
fn test_insert() {



    let name = "Shoes".to_string();
    let col_names_types = vec![("Name".to_string(), ColType::RequiredStrCol), ("Price".to_string(), ColType::RequiredIntCol), ("Stock".to_string(), ColType::RequiredIntCol), ("Discount".to_string(), ColType::OptionalIntCol)];
    let import_str_rows = vec![vec![Some("Slippers".to_string())], 
                               vec![Some("Boots".to_string())]];
    let import_int_rows = vec![vec![Some(12), Some(33), None], vec![Some(33), Some(5), Some(12)]];

    
    let mut shoes_table = DbTable::new(name, col_names_types, import_str_rows, import_int_rows).expect("good");

    shoes_table.pretty_print();


    let import_str_rows = vec![vec![Some("Heels".to_string())], 
                               vec![Some("Flats".to_string())]];
    let import_int_rows = vec![vec![Some(40), Some(2), Some(5)], vec![Some(33), Some(70), None]];

    let new_len = shoes_table.insert(import_str_rows, import_int_rows);
    println!("new len {}", new_len);

    assert_eq!(new_len, 4);


    



}

pub fn string_to_coltype(col_type_string: &str) -> ColType{
    match col_type_string {
        "RS" => ColType::RequiredStrCol,
        "RI" => ColType::RequiredIntCol,
        "OS" => ColType::OptionalStrCol,
        "OI" => ColType::OptionalIntCol,
        _ => ColType::Unknown,
    }
}

#[test]
fn test_new(){

    let name = "Shoes".to_string();
    let col_names_types = vec![("Name".to_string(), ColType::RequiredStrCol), ("Price".to_string(), ColType::RequiredIntCol), ("Stock".to_string(), ColType::RequiredIntCol), ("Discount".to_string(), ColType::OptionalIntCol)];
    let import_str_rows = vec![vec![None], 
                               vec![Some("Boots".to_string())]];
    let import_int_rows = vec![vec![Some(12), Some(33), None], 
                               vec![Some(33), Some(5), Some(12)]];

    
    let mut shoes_table = DbTable::new(name, col_names_types, import_str_rows, import_int_rows);

    //should fail because missing required name 
    assert_eq!(shoes_table.is_none(), true);


    let name = "Shoes".to_string();
    let col_names_types = vec![("Name".to_string(), ColType::RequiredStrCol), ("Price".to_string(), ColType::RequiredIntCol), ("Stock".to_string(), ColType::RequiredIntCol), ("Discount".to_string(), ColType::OptionalIntCol)];
    let import_str_rows = vec![vec![Some("Boots".to_string())]];
    let import_int_rows = vec![vec![Some(12), Some(33), None], 
                               vec![Some(33), Some(5), Some(12)]];

    
    let mut shoes_table = DbTable::new(name, col_names_types, import_str_rows, import_int_rows);

    //should fail because length issue 
    assert_eq!(shoes_table.is_none(), true);


    let name = "Shoes".to_string();
    let col_names_types = vec![("Name".to_string(), ColType::RequiredStrCol), ("Price".to_string(), ColType::RequiredIntCol), ("Stock".to_string(), ColType::RequiredIntCol), ("Discount".to_string(), ColType::OptionalIntCol)];
    let import_str_rows = vec![vec![Some("Slippers".to_string())], 
                               vec![Some("Boots".to_string())]];
    let import_int_rows = vec![vec![Some(12), None, None], 
                               vec![Some(33), Some(5), Some(12)]];

    
    let mut shoes_table = DbTable::new(name, col_names_types, import_str_rows, import_int_rows);

    //should fail because missing required stock
    assert_eq!(shoes_table.is_none(), true);


    let name = "Shoes".to_string();
    let col_names_types = vec![("Name".to_string(), ColType::RequiredStrCol), ("Price".to_string(), ColType::RequiredIntCol), ("Stock".to_string(), ColType::RequiredIntCol), ("Discount".to_string(), ColType::OptionalIntCol)];
    let import_str_rows = vec![vec![Some("Slippers".to_string())], 
                               vec![Some("Boots".to_string())]];
    let import_int_rows = vec![vec![Some(12), Some(33), None], vec![Some(33), Some(5), Some(12)]];

    
    let mut shoes_table = DbTable::new(name, col_names_types, import_str_rows, import_int_rows).expect("good");

    //should pass since passed in clean data
    // assert_eq!(shoes_table.is_none(), false);

    shoes_table.pretty_print();

   


}
