use std::fmt::Debug;
use crate::reading_function::{*};
use polars::prelude::{DataFrame, Series, NamedFrom};


#[derive(Debug, Clone)]
pub struct Idat {
    pub pathname: String, // The full path of file: WITH _Gnr.idat or _Red.idat
    pub is_red: bool, // Is it the red Idat
    pub is_green: bool, // is it the green idat
    pub processed: bool, // Has it been processed (added the actual data)
    pub is_valid_idat: bool, // Check if it is a valid idat we can read
    pub total_number_of_beads: u32, // The total number of probes to read data
    pub barcode: String, //  Barcode of chip
    pub position: String, //  Position of array on chip RXXCXX
    pub data: DataFrame, // Actually hold the data as a dataframe
    pub chip_type: String // the type of chip -> 8 x 1
}

// TODO: Maybe wrap idats into enum instead of a red / green tag
pub enum _IdatKind {
    Red(Idat),
    Green(Idat)
}

/// Idat building  functions 
impl Idat {
    /// Build Red Idat
    pub fn red_from_base<T: ToString>(base: T) -> Idat {
        let basename = base.to_string();
        let path = get_file_name(basename, "Red");
        // Initialise empty unprocessed Idat
        let mut idat = Idat { pathname: path, is_red: true, is_green: false, processed: false,
            is_valid_idat: false, 
            total_number_of_beads: 0,
            barcode: String::new(),
            position: String::new(),
            data: DataFrame::default(),
            chip_type: String::new()};
        // Pre-process idat
        idat = idat.process();
        return idat
    }
    /// Build Green Idat
    pub fn green_from_base<T: ToString>(base: T) -> Idat {
        let basename = base.to_string();
        let path = get_file_name(basename, "Grn");
        // Initialise empty unprocessed Idat
        let mut idat = Idat { pathname: path, is_red: false, is_green: true, processed: false,
            is_valid_idat: false, 
            total_number_of_beads: 0,
            barcode: String::new(),
            position: String::new(),
            data: DataFrame::default(), 
            chip_type: String::new()};
        // Pre-process idat
        idat = idat.process();
        return idat
    }
}

/// IDAT processing functions
impl Idat {
    pub fn process(mut self) -> Self {

        // INITIALISE READER HEAD AT 0
        let mut reader_head: usize = 0; 

        let full_binary = get_binary_from_file(&self.pathname);

        // READ MAGIC 
        let magic = read_magic(&full_binary, & mut reader_head);

        if magic != "IDAT" {panic!("The current idat: {} is not a valid IDAT file", self.pathname)};

        // READ VERSION
        let version = read_version(&full_binary, & mut reader_head);
        if version < 3 {panic!("The current idat: {} is a version that cannot be read", self.pathname)};

        // SET VALID FLAG
        self.is_valid_idat = true;

        // GET ALL FIELDS AVAILABLE TO READ 
        let n_fields  = read_int(&full_binary, & mut reader_head);
        self.total_number_of_beads = n_fields;

        //Initialise Vectors for data storage
        // DO NOT USE FIELDS USE 3 different Vectors
        let mut code_vector : Vec<u16> = Vec::new();
        let mut code_str_vector : Vec<String> = Vec::new();
        let mut offset_vector : Vec<i64> = Vec::new();

        // Get Fields and related offsets
        // GET FIELDS
        for _ in 0..n_fields{
            let code = read_short(&full_binary, &mut reader_head);
            let offset = read_long(&full_binary, &mut reader_head);
            let code_str = known_codes_reverse().get(&code).unwrap().to_owned();
            code_vector.push(code);
            code_str_vector.push(code_str);
            offset_vector.push(offset)
        };

        //Get Number of SNP read
        let mut offset_offset  = get_offset_for_field(&code_str_vector, &offset_vector, "nSNPsRead");
        let n_snps_read = read_int(&full_binary, &mut offset_offset);
        self.total_number_of_beads = n_snps_read;
        
            // 1 - Illumina ID 
        offset_offset  = get_offset_for_field(&code_str_vector, &offset_vector, "IlluminaID");
        let illumina_id = read_int_vec(&full_binary, &mut offset_offset, n_snps_read);

        // 2 - Mean
        offset_offset  = get_offset_for_field(&code_str_vector, &offset_vector, "Mean");
        let mean = read_short_vec(&full_binary, &mut offset_offset, n_snps_read);

        // 3 - SD
        offset_offset  = get_offset_for_field(&code_str_vector, &offset_vector, "SD");
        let sd = read_short_vec(&full_binary, &mut offset_offset, n_snps_read);

        // 4 - NBeads
        offset_offset  = get_offset_for_field(&code_str_vector, &offset_vector, "NBeads");
        let n_beads = read_byte_vec(&full_binary, &mut offset_offset, n_snps_read);

        // CREATE THE DATA DATAFRAME
        let data = DataFrame::new(vec![
            Series::new("ID", illumina_id),
            Series::new("Mean", mean),
            Series::new("SD", sd),
            Series::new("N_beads", n_beads)
        ]);

        self.data = match data {
            Ok(df) => df,
            Err(error) => panic!("Problem creating data for {:?} | error {}", self.pathname, error),
        };

        // 5 - Midblock TODO - It's the same as illumina ID?

        // 6 - Barcode 
        offset_offset  = get_offset_for_field(&code_str_vector, &offset_vector, "Barcode");
        let barcode = read_string(&full_binary, &mut offset_offset);
        self.barcode = barcode;

        // 7 -  Chip Type
        offset_offset  = get_offset_for_field(&code_str_vector, &offset_vector, "ChipType");
        let chip_type = read_string(&full_binary, &mut offset_offset);
        self.chip_type = chip_type;

        // 8 - Runinfoblock TODO

        // 9 - MostlyA - Which is R01C01 - Position
        offset_offset  = get_offset_for_field(&code_str_vector, &offset_vector, "MostlyA");
        let position = read_string(&full_binary, &mut offset_offset);
        self.position = position;

        // 10 - RED GREEN ? Always zero?
        offset_offset  = get_offset_for_field(&code_str_vector, &offset_vector, "RedGreen");
        let _red_green = read_int(&full_binary, &mut offset_offset); //  Dot report ?
        return self
    }
}
