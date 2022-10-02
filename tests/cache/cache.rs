use super::Cache;
use test_case::test_case;

#[test_case(2.320041, 48.8588897, ["Île-de-France	4", "France métropolitaine	3", "France	2"].to_vec(); "with Paris coordinates")]
#[test_case(-74.0060152, 40.7127281, ["City of New York	5", "New York	4", "United States	2"].to_vec(); "with New York coordinates")]
#[test_case(18.9558186, 69.651648, ["Tromsø	7", "Troms og Finnmark	4", "Norge	2"].to_vec(); "with Tromso coordinates")]
fn test_lookup_with_coordinates(longitude: f32, latitude: f32, expected_result: Vec<&str>) {
    let file_path: String = "data/planet-220926.osm.0_01.bin".to_string();
    let mut cache = Cache::parse_buffer(&file_path).unwrap();
    let result = cache.lookup(longitude, latitude);
    assert_eq!(result, expected_result);
}

#[test]
fn test_lookup_using_cache_with_coordinates() {
    let file_path: String = "data/planet-220926.osm.0_01.bin".to_string();
    let mut cache = Cache::parse_buffer(&file_path).unwrap();
    let longitude = 2.320041;
    let latitude = 48.8588897;
    let result = cache.lookup(longitude, latitude);
    let expected_result = ["Île-de-France	4", "France métropolitaine	3", "France	2"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    assert_eq!(result, expected_result);
    for (_, geocoding) in cache.cache {
        assert_eq!(geocoding, expected_result);
    }
}
