pub trait JsonHandler {
    fn parse_json(&self) -> String;
    fn read_json(&self) -> String;
}
