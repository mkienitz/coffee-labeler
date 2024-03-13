use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
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

impl BeanInfo {
    fn sanitise(&self, pattern: &str) -> Self {
        BeanInfo {
            country: self.country.replace(pattern, ""),
            name: self.name.replace(pattern, ""),
            roaster: self.roaster.replace(pattern, ""),
            varietals: self.varietals.replace(pattern, ""),
            region: self.region.replace(pattern, ""),
            farm: self.farm.replace(pattern, ""),
            elevation: self.elevation.replace(pattern, ""),
            dose_weight: self.dose_weight.replace(pattern, ""),
            roasting_date: self.roasting_date.replace(pattern, ""),
            processing: self.processing.replace(pattern, ""),
            aromatics: self.aromatics.replace(pattern, ""),
        }
    }

    pub fn sanitised(&self) -> Self {
        println!("Sanitising...\n{:?}", self);
        self.sanitise("&nbsp;")
            .sanitise("<br>")
            .sanitise("<div>")
            .sanitise("</div>")
    }
}
