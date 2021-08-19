use esp32::partition_table::PartitionTable;

fn main() {
    let partition_table = PartitionTable::from_file("partition_table.bin");
    println!("{:?}", partition_table);
}
