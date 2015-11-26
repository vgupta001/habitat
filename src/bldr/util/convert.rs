//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::collections::{BTreeMap, HashMap};

use toml;
use mustache;

// Translates a toml table to a mustache datastructure.
pub fn toml_table_to_mustache(toml: BTreeMap<String, toml::Value>) -> mustache::Data {
    let mut hashmap = HashMap::new();
    for (key, value) in toml.iter() {
        hashmap.insert(format!("{}", key), toml_to_mustache(value.clone()));
    }
    mustache::Data::Map(hashmap)
}

// Translates a given toml value to its mustache equivalent.
pub fn toml_to_mustache(value: toml::Value) -> mustache::Data {
    match value {
        toml::Value::String(s) => mustache::Data::StrVal(format!("{}", s)),
        toml::Value::Integer(i) => mustache::Data::StrVal(format!("{}", i)),
        toml::Value::Float(i) => mustache::Data::StrVal(format!("{}", i)),
        toml::Value::Boolean(b) => mustache::Data::Bool(b),
        toml::Value::Datetime(s) => mustache::Data::StrVal(format!("{}", s)),
        toml::Value::Array(a) => toml_vec_to_mustache(a),
        toml::Value::Table(t) => toml_table_to_mustache(t),
    }
}

// Translates toml vectors to mustache vectors.
pub fn toml_vec_to_mustache(toml: Vec<toml::Value>) -> mustache::Data {
    let mut mvec = vec![];
    for x in toml.iter() {
        mvec.push(toml_to_mustache(x.clone()))
    }
    mustache::Data::VecVal(mvec)
}

#[cfg(test)]
mod tests {
    use super::toml_table_to_mustache;
    use toml;
    use mustache;

    #[test]
    fn toml_data_is_rendered_to_mustache() {
        let toml = r#"
                daemonize = "no"
                slaveof = "127.0.0.1 6380"

                [winks]
                left = "yes"
                right = "no"
                wiggle = [ "snooze", "looze" ]
            "#;
        let toml_value = toml::Parser::new(toml).parse().unwrap();
        let template = mustache::compile_str("hello {{daemonize}} for {{slaveof}} \
                                              {{winks.right}} {{winks.left}} {{# winks.wiggle}} \
                                              {{.}} {{/winks.wiggle}}");
        let mut bytes = vec![];
        let data = toml_table_to_mustache(toml_value);
        template.render_data(&mut bytes, &data);
        assert_eq!(String::from_utf8(bytes).unwrap(),
                   "hello no for 127.0.0.1 6380 no yes  snooze  looze ".to_string());
    }
}