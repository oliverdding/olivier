use crate::sea_orm_active_enums::Category;
use std::str::FromStr;

impl FromStr for Category {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ask" => Ok(Category::Ask),
            "comment" => Ok(Category::Comment),
            "story" => Ok(Category::Story),
            _ => Err("invalid category"),
        }
    }
}
