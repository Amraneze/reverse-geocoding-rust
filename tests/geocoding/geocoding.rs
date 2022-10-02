use super::Geocoding;
use test_case::test_case;

#[test_case(2.320041, 48.8588897, ["Île-de-France	4", "France métropolitaine	3", "France	2"].to_vec(); "with Paris coordinates")]
#[test_case(-74.0060152, 40.7127281, ["City of New York	5", "New York	4", "United States	2"].to_vec(); "with New York coordinates")]
#[test_case(18.9558186, 69.651648, ["Tromsø	7", "Troms og Finnmark	4", "Norge	2"].to_vec(); "with Tromso coordinates")]
fn test_lookup_with_coordinates(longitude: f32, latitude: f32, expected_result: Vec<&str>) {
    let args: Vec<String> = vec!["main", "data/planet-220926.osm.0_01.bin"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mut geocoding = Geocoding::new(&args);
    let result = geocoding.lookup(longitude, latitude);
    assert_eq!(result, expected_result);
}

#[test]
#[ignore]
fn test_lookup_from_cached_with_paris_coordinates() {
    let args: Vec<String> = vec!["main", "data/planet-220926.osm.0_01.bin"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mut geocoding = Geocoding::new(&args);
    let longitude = 2.320041 as f32;
    let latitude = 48.8588897 as f32;
    let _result = geocoding.lookup(longitude, latitude);
}

#[test]
#[should_panic(
    expected = "File path is not provided as argument. Use the format cargo ... -- file_path"
)]
fn test_parsing_empty_arguments() {
    Geocoding::new(&vec![]);
}

#[test]
#[should_panic(expected = "Index file does not have the correct type or version.")]
fn test_giving_a_non_valid_file() {
    let args: Vec<String> = vec!["main", "data/planet-220926.osm.1.png"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    Geocoding::new(&args);
}
