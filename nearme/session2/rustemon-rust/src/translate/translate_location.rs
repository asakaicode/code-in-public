pub struct Region {
    jp_name: String,
}

impl Region {
    pub fn new(location_jp: String) -> Self {
        Self { location_name }
    }

    pub async fn tlanslate_from_jp(&mut self) -> String {
        let jp_str = self.jp_name.as_str();
        match jp_str {
            "フタバタウン" => String::from("twinleaf-town"),
            "マサゴタウン" => String::from("sandgem-town"),
            _ => panic!("No assigned name."),
        }
    }
}
