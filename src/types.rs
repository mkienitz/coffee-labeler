use serde::Deserialize;

#[derive(Deserialize)]
pub struct BeanInfo {
    pub country: String,
    pub name: String,
    pub roaster: String,
    pub varietals: String,
    pub region: String,
    pub farm: String,
    pub elevation: String,
    pub dose_weight: String,
    pub roasting_date: String,
    pub processing: String,
    pub aromatics: String,
}

impl Default for BeanInfo {
    fn default() -> Self {
        Self {
            country: "EC".to_string(),
            name: "Ecuador Segundo".to_string(),
            roaster: "A.M.O.C.".to_string(),
            varietals: "F1 Hybrid".to_string(),
            region: "San Antonio, Loja".to_string(),
            farm: "Segundo".to_string(),
            elevation: "~1425m".to_string(),
            dose_weight: "12g".to_string(),
            roasting_date: "24-02-01".to_string(),
            processing: "washed".to_string(),
            aromatics: "lemongrass, raspberry, black tea".to_string(),
        }
    }
}
