use ical::parser::ical::component::IcalEvent;
use icalendar::Component;

pub fn from_property(property: ical::property::Property) -> Option<icalendar::Property> {
    let mut prop = if let Some(value) = property.value {
        icalendar::Property::new_pre_alloc(property.name, value)
    } else {
        return None;
    };

    if let Some(parameters) = property.params {
        for (name, value) in parameters {
            prop.append_parameter(icalendar::Parameter::new(
                name.as_str(),
                value.join(",").as_str(),
            ));
        }
    }

    Some(prop)
}

pub fn from_event(event: IcalEvent) -> icalendar::Event {
    let mut result = icalendar::Event::new();
    for property in event.properties {
        if let Some(property) = from_property(property) {
            result.append_property(property);
        }
    }

    result
}

data_macro::building_id_matcher!(
    pub fn match_building_id("./data/buildings.json")
);

data_macro::course_name_replacer!(
    pub fn replace_course_name("./data/courses.json")
);
