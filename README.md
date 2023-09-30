# illumina_array_reader
A rust library to read Illumina Idat files. A port of [illuminaio](https://f1000research.com/articles/2-264/v1).

## How to implement in your project 
Add this line to you Cargo.toml within the ```[dependencies]``` section:  
```illumina_array_reader = {git = "https://github.com/a-xavier/illumina_array_reader.git"}```

## How to use
illumina_array_reader implements two structs ```MicroArray``` and ```Idat```. In practice, one would only need to use ```MicroArray``` since it's a collection of a red and a green ```Idat```.

Each ```Idat``` struct contains descriptions and most importantly, a ```data``` field containing a polars DataFrame with the data contained in the idat (ProbeID, Mean fluorescence, Fluorescence Standard Deviation and Number of beads).

To load an array, just provide a **Basename**. A **Basename** is the path to both idat files, without the ```_Grn.idat``` or ```_Grn.idat.gz```.

For example:  
```rust
use illumina_array_reader::array::MicroArray;
fn main() {
    let array = MicroArray::from_base("/path/to/files/203952880120_R01C01");

    dbg!(array);
}
```

Will print:

```rust
MicroArray::from_base(all_idats[0]) = MicroArray {
    basename: "/path/to/files/203952880120_R01C01",
    green_idat: Idat {
        pathname: "/path/to/files/203952880120_R01C01_Grn.idat",
        is_red: false,
        is_green: true,
        processed: true,
        is_valid_idat: true,
        total_number_of_beads: 1051815,
        barcode: "203952880120",
        position: "R01C01",
        data: shape: (1_051_815, 4)
        ┌──────────┬───────┬──────┬─────────┐
        │ ID       ┆ Mean  ┆ SD   ┆ N_beads │
        │ ---      ┆ ---   ┆ ---  ┆ ---     │
        │ u32      ┆ u16   ┆ u16  ┆ u8      │
        ╞══════════╪═══════╪══════╪═════════╡
        │ 1600101  ┆ 10409 ┆ 1275 ┆ 14      │
        │ 1600111  ┆ 5754  ┆ 302  ┆ 10      │
        │ 1600115  ┆ 2539  ┆ 442  ┆ 19      │
        │ 1600123  ┆ 5540  ┆ 703  ┆ 20      │
        │ …        ┆ …     ┆ …    ┆ …       │
        │ 99810970 ┆ 98    ┆ 38   ┆ 6       │
        │ 99810978 ┆ 7953  ┆ 412  ┆ 17      │
        │ 99810990 ┆ 5474  ┆ 735  ┆ 9       │
        │ 99810992 ┆ 4181  ┆ 508  ┆ 25      │
        └──────────┴───────┴──────┴─────────┘,
        chip_type: "BeadChip 8x5",
    },
    red_idat: Idat {
        pathname: "/path/to/files/203952880120_R01C01_Red.idat",
        is_red: true,
        is_green: false,
        processed: true,
        is_valid_idat: true,
        total_number_of_beads: 1051815,
        barcode: "203952880120",
        position: "R01C01",
        data: shape: (1_051_815, 4)
        ┌──────────┬───────┬──────┬─────────┐
        │ ID       ┆ Mean  ┆ SD   ┆ N_beads │
        │ ---      ┆ ---   ┆ ---  ┆ ---     │
        │ u32      ┆ u16   ┆ u16  ┆ u8      │
        ╞══════════╪═══════╪══════╪═════════╡
        │ 1600101  ┆ 4648  ┆ 923  ┆ 14      │
        │ 1600111  ┆ 4382  ┆ 1110 ┆ 10      │
        │ 1600115  ┆ 27623 ┆ 2773 ┆ 19      │
        │ 1600123  ┆ 3322  ┆ 652  ┆ 20      │
        │ …        ┆ …     ┆ …    ┆ …       │
        │ 99810970 ┆ 2627  ┆ 806  ┆ 6       │
        │ 99810978 ┆ 5296  ┆ 947  ┆ 17      │
        │ 99810990 ┆ 15208 ┆ 1883 ┆ 9       │
        │ 99810992 ┆ 728   ┆ 458  ┆ 25      │
        └──────────┴───────┴──────┴─────────┘,
        chip_type: "BeadChip 8x5",
    },
}


```