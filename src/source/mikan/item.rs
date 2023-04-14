// Copyright 2023 RinChanNOWWW
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use yaserde_derive::YaDeserialize;
use yaserde_derive::YaSerialize;

#[derive(Debug, YaDeserialize, Default)]
pub struct MikanRSSContent {
    pub channel: Channel,
}

#[derive(Debug, YaDeserialize, YaSerialize, Default)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    #[yaserde(rename = "item")]
    pub items: Vec<MikanRSSItem>,
}

#[derive(Debug, YaDeserialize, YaSerialize, Default)]
pub struct MikanRSSItem {
    pub guid: String,
    pub link: String,
    pub title: String,
    pub description: String,
    pub torrent: Torrent,
    pub enclosure: Enclosure,
}

#[derive(Debug, YaDeserialize, YaSerialize, Default)]
#[yaserde(namespace = "https://mikanani.me/0.1/")]
pub struct Torrent {
    pub link: String,
    #[yaserde(rename = "contentLength")]
    pub content_length: String,
    #[yaserde(rename = "pubDate")]
    pub pub_date: String,
}

#[derive(Debug, YaDeserialize, YaSerialize, Default)]
pub struct Enclosure {
    #[yaserde(attribute, rename = "type")]
    pub e_type: String,
    #[yaserde(attribute)]
    pub length: String,
    #[yaserde(attribute)]
    pub url: String,
}
