use std::collections::HashSet;

fn main() {
    // #TODO:
    // 1. Take in list of flight paths and process JSON format
    // 2. Split flight paths list into BTreeSet or two HashSets for source and dest airport codes
    // 3. Use symmetric difference to ignore shared airport codes

    //input: [['IND', 'EWR'], ['SFO', 'ATL'], ['GSO', 'IND'], ['ATL', 'GSO']]
    //source_codes: ['IND', 'SFO', 'GSO', 'ATL']
    //dest_codes: ['EWR', 'ATL', 'IND', 'GSO']
    //trip_path: ['SFO', 'EWR']
    let source_codes = HashSet::from(["IND", "SFO", "GSO", "ATL"]);
    let dest_codes = HashSet::from(["EWR", "ATL", "IND", "GSO"]);

    let trip_path = source_codes.symmetric_difference(&dest_codes);
    println!("{:?}", trip_path);
}