use async_trait::async_trait;
use gtk::glib;
use rand::seq::SliceRandom;

use std::time::Duration;

use super::{AudD, Data, Error, Provider, ProviderError, Response};
use crate::{core::AudioRecording, model::Song};

#[derive(Debug)]
pub struct AudDMock;

impl AudDMock {
    fn random_data(&self) -> Result<Data, Error> {
        let raw_responses = [
            r#"{"status":"success","result":null}"#,
            r#"{"status":"error","error":{"error_code":901,"error_message":"Recognition failed: authorization failed: no api_token passed and the limit was reached. Get an api_token from dashboard.audd.io."},"request_params":{},"request_api_method":"recognize","request_http_method":"POST","see api documentation":"https://docs.audd.io","contact us":"api@audd.io"}"#,
            r#"{"status":"error","error":{"error_code":900,"error_message":"Recognition failed: authorization failed: wrong api_token. Please check if your account is activated on dashboard.audd.io and has either a trial or an active subscription."},"request_params":{},"request_api_method":"recognize","request_http_method":"POST","see api documentation":"https://docs.audd.io","contact us":"api@audd.io"}"#,
            r#"{"status":"error","error":{"error_code":300,"error_message":"Recognition failed: a problem with fingerprints creating. Keep in mind that you should send only audio files or links to audio files. We support some of the Instagram, Twitter, TikTok and Facebook videos, and also parse html for OpenGraph and JSON-LD media and \\u003caudio\\u003e/\\u003cvideo\\u003e tags, but it's always better to send a 10-20 seconds-long audio file. For audio streams, see https://docs.audd.io/streams/"},"request_params":{},"request_api_method":"recognize","request_http_method":"POST","see api documentation":"https://docs.audd.io","contact us":"api@audd.io"}"#,
            r#"{"status":"success","result":{"artist":"The London Symphony Orchestra","title":"Eine Kleine Nachtmusik","album":"An Hour Of The London Symphony Orchestra","release_date":"2014-04-22","label":"Glory Days Music","timecode":"00:24","song_link":"https://lis.tn/EineKleineNachtmusik"}}"#,
            r#"{"status":"success","result":{"artist":"Public","title":"Make You Mine","album":"Let's Make It","release_date":"2014-10-07","label":"PUBLIC","timecode":"00:43","song_link":"https://lis.tn/FUYgUV"}}"#,
            r#"{"status":"success","result":{"artist":"5 Seconds Of Summer","title":"Amnesia","album":"Amnesia","release_date":"2014-06-24","label":"Universal Music","timecode":"01:02","song_link":"https://lis.tn/WSKAzD","spotify":{"album":{"name":"5 Seconds Of Summer","artists":[{"name":"5 Seconds of Summer","id":"5Rl15oVamLq7FbSb0NNBNy","uri":"spotify:artist:5Rl15oVamLq7FbSb0NNBNy","href":"https://api.spotify.com/v1/artists/5Rl15oVamLq7FbSb0NNBNy","external_urls":{"spotify":"https://open.spotify.com/artist/5Rl15oVamLq7FbSb0NNBNy"}}],"album_group":"","album_type":"album","id":"2LkWHNNHgD6BRNeZI2SL1L","uri":"spotify:album:2LkWHNNHgD6BRNeZI2SL1L","available_markets":null,"href":"https://api.spotify.com/v1/albums/2LkWHNNHgD6BRNeZI2SL1L","images":[{"height":640,"width":640,"url":"https://i.scdn.co/image/ab67616d0000b27393432e914046a003229378da"},{"height":300,"width":300,"url":"https://i.scdn.co/image/ab67616d00001e0293432e914046a003229378da"},{"height":64,"width":64,"url":"https://i.scdn.co/image/ab67616d0000485193432e914046a003229378da"}],"external_urls":{"spotify":"https://open.spotify.com/album/2LkWHNNHgD6BRNeZI2SL1L"},"release_date":"2014-06-27","release_date_precision":"day"},"external_ids":{"isrc":"GBUM71401926"},"popularity":69,"is_playable":true,"linked_from":null,"artists":[{"name":"5 Seconds of Summer","id":"5Rl15oVamLq7FbSb0NNBNy","uri":"spotify:artist:5Rl15oVamLq7FbSb0NNBNy","href":"https://api.spotify.com/v1/artists/5Rl15oVamLq7FbSb0NNBNy","external_urls":{"spotify":"https://open.spotify.com/artist/5Rl15oVamLq7FbSb0NNBNy"}}],"available_markets":null,"disc_number":1,"duration_ms":237247,"explicit":false,"external_urls":{"spotify":"https://open.spotify.com/track/1JCCdiru7fhstOIF4N7WJC"},"href":"https://api.spotify.com/v1/tracks/1JCCdiru7fhstOIF4N7WJC","id":"1JCCdiru7fhstOIF4N7WJC","name":"Amnesia","preview_url":"","track_number":12,"uri":"spotify:track:1JCCdiru7fhstOIF4N7WJC"}}}"#,
            r#"{"status":"success","result":{"artist":"Alessia Cara","title":"Scars To Your Beautiful","album":"Know-It-All","release_date":"2015-11-13","label":"EP Entertainment, LLC / Def Jam","timecode":"00:28","song_link":"https://lis.tn/ScarsToYourBeautiful","spotify":{"album":{"name":"Know-It-All (Deluxe)","artists":[{"name":"Alessia Cara","id":"2wUjUUtkb5lvLKcGKsKqsR","uri":"spotify:artist:2wUjUUtkb5lvLKcGKsKqsR","href":"https://api.spotify.com/v1/artists/2wUjUUtkb5lvLKcGKsKqsR","external_urls":{"spotify":"https://open.spotify.com/artist/2wUjUUtkb5lvLKcGKsKqsR"}}],"album_group":"","album_type":"album","id":"3rDbA12I5duZnlwakqDdZa","uri":"spotify:album:3rDbA12I5duZnlwakqDdZa","available_markets":null,"href":"https://api.spotify.com/v1/albums/3rDbA12I5duZnlwakqDdZa","images":[{"height":640,"width":640,"url":"https://i.scdn.co/image/ab67616d0000b273e3ae597159d6c2541c4ee61b"},{"height":300,"width":300,"url":"https://i.scdn.co/image/ab67616d00001e02e3ae597159d6c2541c4ee61b"},{"height":64,"width":64,"url":"https://i.scdn.co/image/ab67616d00004851e3ae597159d6c2541c4ee61b"}],"external_urls":{"spotify":"https://open.spotify.com/album/3rDbA12I5duZnlwakqDdZa"},"release_date":"2015-11-13","release_date_precision":"day"},"external_ids":{"isrc":"USUM71506811"},"popularity":75,"is_playable":true,"linked_from":null,"artists":[{"name":"Alessia Cara","id":"2wUjUUtkb5lvLKcGKsKqsR","uri":"spotify:artist:2wUjUUtkb5lvLKcGKsKqsR","href":"https://api.spotify.com/v1/artists/2wUjUUtkb5lvLKcGKsKqsR","external_urls":{"spotify":"https://open.spotify.com/artist/2wUjUUtkb5lvLKcGKsKqsR"}}],"available_markets":null,"disc_number":1,"duration_ms":230226,"explicit":false,"external_urls":{"spotify":"https://open.spotify.com/track/0prNGof3XqfTvNDxHonvdK"},"href":"https://api.spotify.com/v1/tracks/0prNGof3XqfTvNDxHonvdK","id":"0prNGof3XqfTvNDxHonvdK","name":"Scars To Your Beautiful","preview_url":"","track_number":10,"uri":"spotify:track:0prNGof3XqfTvNDxHonvdK"}}}"#,
            r#"{"status":"success","result":{"artist":"Daniel Boone","title":"Beautiful Sunday","album":"Pop Legend Vol.1","release_date":"2010-01-15","label":"Open Records","timecode":"00:33","song_link":"https://lis.tn/YTuccJ","spotify":{"album":{"name":"Cocktail Super Pop","artists":[{"name":"Various Artists","id":"0LyfQWJT6nXafLPZqxe9Of","uri":"spotify:artist:0LyfQWJT6nXafLPZqxe9Of","href":"https://api.spotify.com/v1/artists/0LyfQWJT6nXafLPZqxe9Of","external_urls":{"spotify":"https://open.spotify.com/artist/0LyfQWJT6nXafLPZqxe9Of"}}],"album_group":"","album_type":"compilation","id":"1ZsLymIsvlHEnGtQFen5xd","uri":"spotify:album:1ZsLymIsvlHEnGtQFen5xd","available_markets":null,"href":"https://api.spotify.com/v1/albums/1ZsLymIsvlHEnGtQFen5xd","images":[{"height":640,"width":640,"url":"https://i.scdn.co/image/ab67616d0000b273db8f64a52a4ec4cde9a9528a"},{"height":300,"width":300,"url":"https://i.scdn.co/image/ab67616d00001e02db8f64a52a4ec4cde9a9528a"},{"height":64,"width":64,"url":"https://i.scdn.co/image/ab67616d00004851db8f64a52a4ec4cde9a9528a"}],"external_urls":{"spotify":"https://open.spotify.com/album/1ZsLymIsvlHEnGtQFen5xd"},"release_date":"2013-01-18","release_date_precision":"day"},"external_ids":{"isrc":"ES5530914999"},"popularity":0,"is_playable":true,"linked_from":null,"artists":[{"name":"Daniel Boone","id":"3M5aUsJmembbwKbUx434lS","uri":"spotify:artist:3M5aUsJmembbwKbUx434lS","href":"https://api.spotify.com/v1/artists/3M5aUsJmembbwKbUx434lS","external_urls":{"spotify":"https://open.spotify.com/artist/3M5aUsJmembbwKbUx434lS"}}],"available_markets":null,"disc_number":1,"duration_ms":176520,"explicit":false,"external_urls":{"spotify":"https://open.spotify.com/track/6o3AMOtlfI6APSUooekMtt"},"href":"https://api.spotify.com/v1/tracks/6o3AMOtlfI6APSUooekMtt","id":"6o3AMOtlfI6APSUooekMtt","name":"Beautiful Sunday","preview_url":"https://p.scdn.co/mp3-preview/b2fa24732fe08a251b0c8d44774f37fd55378378?cid=e44e7b8278114c7db211c00ea273ac69","track_number":16,"uri":"spotify:track:6o3AMOtlfI6APSUooekMtt"}}}"#,
            r#"{"status":"success","result":{"artist":"Kitchie Nadal","title":"Huwag Na Huwag Mong Sasabihin","album":"Kitchie Nadal","release_date":"2004-01-01","label":"12 Stone Records","timecode":"00:07","song_link":"https://lis.tn/PwQvZ","apple_music":{"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview115/v4/7d/2a/e5/7d2ae52b-dbde-845c-0803-9b12762cba75/mzaf_8377462403541839064.plus.aac.p.m4a"}],"artwork":{"width":1400,"height":1400,"url":"https://is4-ssl.mzstatic.com/image/thumb/Music124/v4/4b/f2/39/4bf23908-cc93-0ca5-217e-8c7e426b1990/889211800933.jpg/{w}x{h}bb.jpg","bgColor":"281800","textColor1":"d3f1b2","textColor2":"9bda12","textColor3":"b1c58e","textColor4":"84b30f"},"artistName":"Kitchie Nadal","url":"https://music.apple.com/us/album/huwag-na-huwag-mong-sasabihin/1031215547?app=music\u0026at=1000l33QU\u0026i=1031215560\u0026mt=1","discNumber":1,"genreNames":["Rock","Music","Pop"],"durationInMillis":249080,"releaseDate":"2004-01-01","name":"Huwag Na Huwag Mong Sasabihin","isrc":"USDY41595164","albumName":"Kitchie Nadal","playParams":{"id":"1031215560","kind":"song"},"trackNumber":5,"composerName":"Anna Katrina Dumilon Nadal"},"musicbrainz":[{"id":"d245a10e-f39a-4cfe-a98c-803fcf72e924","score":100,"title":"Huwag Na Huwag Mong Sasabihin","length":249080,"disambiguation":"","video":null,"artist-credit":[{"name":"Kitchie Nadal","artist":{"id":"0a7fd2c2-d03b-45d3-bc47-47547f4a91e4","name":"Kitchie Nadal","sort-name":"Nadal, Kitchie"}}],"releases":[{"id":"8d8e11b4-6859-4e78-8025-3f6254c4e4be","count":1,"title":"Kitchie Nadal","status":"Official","date":"2004-01-01","country":"PH","release-events":[{"date":"2004-01-01","area":{"id":"786532a5-2e36-315a-bdf2-221dc1b64b72","name":"Philippines","sort-name":"Philippines","iso-3166-1-codes":["PH"]}}],"track-count":12,"media":[{"position":1,"format":"Digital Media","track":[{"id":"51359cc6-197c-439b-abd4-119da1403c7a","number":"5","title":"Huwag Na Huwag Mong Sasabihin","length":249000}],"track-count":12,"track-offset":4}],"release-group":{"id":"ba030cb3-ec17-32c1-bf7b-581b6dc91592","type-id":"f529b476-6e62-324f-b0aa-1f3e33d313fc","title":"Kitchie Nadal","primary-type":"Album","secondary-types":null}},{"id":"4962e2b0-f9ad-4921-bb39-155b01084cd9","count":1,"title":"Kitchie Nadal","status":"Official","date":"2004","country":"PH","release-events":[{"date":"2004","area":{"id":"786532a5-2e36-315a-bdf2-221dc1b64b72","name":"Philippines","sort-name":"Philippines","iso-3166-1-codes":["PH"]}}],"track-count":10,"media":[{"position":1,"format":"CD","track":[{"id":"7940efe3-ec49-37d1-adb2-d5d216c89c54","number":"4","title":"Wag Na Wag Mong Sasabihin","length":250613}],"track-count":10,"track-offset":3}],"release-group":{"id":"ba030cb3-ec17-32c1-bf7b-581b6dc91592","type-id":"f529b476-6e62-324f-b0aa-1f3e33d313fc","title":"Kitchie Nadal","primary-type":"Album","secondary-types":null}},{"id":"0e42f8d2-3688-4d21-8b1f-09ec17aed74c","count":1,"title":"Kitchie Nadal","status":"Official","disambiguation":"(Special Limited Edition)","date":"2005","country":"PH","release-events":[{"date":"2005","area":{"id":"786532a5-2e36-315a-bdf2-221dc1b64b72","name":"Philippines","sort-name":"Philippines","iso-3166-1-codes":["PH"]}}],"track-count":13,"media":[{"position":1,"format":"CD","track":[{"id":"89c04c73-113b-304f-9798-b695d118a223","number":"5","title":"Huwag Na Huwag Mong Sasabihin","length":249080}],"track-count":13,"track-offset":4}],"release-group":{"id":"ba030cb3-ec17-32c1-bf7b-581b6dc91592","type-id":"f529b476-6e62-324f-b0aa-1f3e33d313fc","title":"Kitchie Nadal","primary-type":"Album","secondary-types":null}}],"isrcs":["USDY41595164"],"tags":null}],"spotify":{"album":{"name":"Kitchie Nadal","artists":[{"name":"Kitchie Nadal","id":"4OjU5UP0GFWeniBC82sGBY","uri":"spotify:artist:4OjU5UP0GFWeniBC82sGBY","href":"https://api.spotify.com/v1/artists/4OjU5UP0GFWeniBC82sGBY","external_urls":{"spotify":"https://open.spotify.com/artist/4OjU5UP0GFWeniBC82sGBY"}}],"album_group":"","album_type":"album","id":"7beVKW0o7iRoM0eRT3kGfk","uri":"spotify:album:7beVKW0o7iRoM0eRT3kGfk","available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BD","BE","BF","BG","BH","BI","BJ","BN","BO","BR","BS","BT","BW","BY","BZ","CA","CD","CG","CH","CI","CL","CM","CO","CR","CV","CW","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GE","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HT","HU","ID","IE","IL","IN","IQ","IS","IT","JM","JO","JP","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","LY","MA","MC","MD","ME","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RS","RU","RW","SA","SB","SC","SE","SG","SI","SK","SL","SM","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","US","UY","UZ","VC","VE","VN","VU","WS","XK","ZA","ZM","ZW"],"href":"https://api.spotify.com/v1/albums/7beVKW0o7iRoM0eRT3kGfk","images":[{"height":640,"width":640,"url":"https://i.scdn.co/image/ab67616d0000b2734699830bfdf6e043cda2d294"},{"height":300,"width":300,"url":"https://i.scdn.co/image/ab67616d00001e024699830bfdf6e043cda2d294"},{"height":64,"width":64,"url":"https://i.scdn.co/image/ab67616d000048514699830bfdf6e043cda2d294"}],"external_urls":{"spotify":"https://open.spotify.com/album/7beVKW0o7iRoM0eRT3kGfk"},"release_date":"2004-01-01","release_date_precision":"day"},"external_ids":{"isrc":"usdy41595164"},"popularity":55,"is_playable":null,"linked_from":null,"artists":[{"name":"Kitchie Nadal","id":"4OjU5UP0GFWeniBC82sGBY","uri":"spotify:artist:4OjU5UP0GFWeniBC82sGBY","href":"https://api.spotify.com/v1/artists/4OjU5UP0GFWeniBC82sGBY","external_urls":{"spotify":"https://open.spotify.com/artist/4OjU5UP0GFWeniBC82sGBY"}}],"available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BD","BE","BF","BG","BH","BI","BJ","BN","BO","BR","BS","BT","BW","BY","BZ","CA","CD","CG","CH","CI","CL","CM","CO","CR","CV","CW","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GE","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HT","HU","ID","IE","IL","IN","IQ","IS","IT","JM","JO","JP","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","LY","MA","MC","MD","ME","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RS","RU","RW","SA","SB","SC","SE","SG","SI","SK","SL","SM","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","US","UY","UZ","VC","VE","VN","VU","WS","XK","ZA","ZM","ZW"],"disc_number":1,"duration_ms":249080,"explicit":false,"external_urls":{"spotify":"https://open.spotify.com/track/4mOjjXKDOR1sel6APULUaN"},"href":"https://api.spotify.com/v1/tracks/4mOjjXKDOR1sel6APULUaN","id":"4mOjjXKDOR1sel6APULUaN","name":"Huwag Na Huwag Mong Sasabihin","preview_url":"https://p.scdn.co/mp3-preview/4f3b0bbb8d5ed81d41a3e0d01ebbff0397840448?cid=e44e7b8278114c7db211c00ea273ac69","track_number":5,"uri":"spotify:track:4mOjjXKDOR1sel6APULUaN"}}}"#,
            r#"{"status":"success","result":{"artist":"Letter Day Story","title":"Kung Maibabalik","album":"Sama-sama","release_date":"2010-07-30","label":"SME - Musiko","timecode":"01:06","song_link":"https://lis.tn/KungMaibabalik","apple_music":{"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview115/v4/02/3d/6d/023d6d90-06f5-5665-0e09-dd95eda5a63c/mzaf_17349228777324501725.plus.aac.p.m4a"}],"artwork":{"width":600,"height":600,"url":"https://is2-ssl.mzstatic.com/image/thumb/Music/d7/86/e3/mzi.cpsqshnm.jpg/{w}x{h}bb.jpg","bgColor":"181d46","textColor1":"d5ecfc","textColor2":"d4c6de","textColor3":"afc3d8","textColor4":"afa4c0"},"artistName":"Letter Day Story","url":"https://music.apple.com/us/album/kung-maibabalik-duet-with-yeng-constantino/387195884?app=music\u0026at=1000l33QU\u0026i=387196208\u0026mt=1","discNumber":1,"genreNames":["Pop","Music"],"durationInMillis":292680,"releaseDate":"2010-07-30","name":"Kung Maibabalik (Duet with Yeng Constantino)","isrc":"PHS111000115","albumName":"Sama-Sama","playParams":{"id":"387196208","kind":"song"},"trackNumber":4,"composerName":"Letter Day Story"},"spotify":{"album":{"name":"Sama-sama","artists":[{"name":"Letter Day Story","id":"2DQsTcDI6YQaIKgY1FjH1M","uri":"spotify:artist:2DQsTcDI6YQaIKgY1FjH1M","href":"https://api.spotify.com/v1/artists/2DQsTcDI6YQaIKgY1FjH1M","external_urls":{"spotify":"https://open.spotify.com/artist/2DQsTcDI6YQaIKgY1FjH1M"}}],"album_group":"","album_type":"album","id":"1gTZ1F8DK1vofKEgTrQ9Ep","uri":"spotify:album:1gTZ1F8DK1vofKEgTrQ9Ep","available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BD","BE","BF","BG","BH","BI","BJ","BN","BO","BR","BS","BT","BW","BY","BZ","CD","CG","CH","CI","CL","CM","CO","CR","CV","CW","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GE","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HT","HU","ID","IE","IL","IN","IQ","IS","IT","JM","JO","JP","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","LY","MA","MC","MD","ME","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RS","RU","RW","SA","SB","SC","SE","SG","SI","SK","SL","SM","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","US","UY","UZ","VC","VE","VN","VU","WS","XK","ZA","ZM","ZW"],"href":"https://api.spotify.com/v1/albums/1gTZ1F8DK1vofKEgTrQ9Ep","images":[{"height":640,"width":640,"url":"https://i.scdn.co/image/ab67616d0000b273fc812dcdc6faf4dbec926dee"},{"height":300,"width":300,"url":"https://i.scdn.co/image/ab67616d00001e02fc812dcdc6faf4dbec926dee"},{"height":64,"width":64,"url":"https://i.scdn.co/image/ab67616d00004851fc812dcdc6faf4dbec926dee"}],"external_urls":{"spotify":"https://open.spotify.com/album/1gTZ1F8DK1vofKEgTrQ9Ep"},"release_date":"2010-07-30","release_date_precision":"day"},"external_ids":{"isrc":"PHS111000115"},"popularity":41,"is_playable":null,"linked_from":null,"artists":[{"name":"Letter Day Story","id":"2DQsTcDI6YQaIKgY1FjH1M","uri":"spotify:artist:2DQsTcDI6YQaIKgY1FjH1M","href":"https://api.spotify.com/v1/artists/2DQsTcDI6YQaIKgY1FjH1M","external_urls":{"spotify":"https://open.spotify.com/artist/2DQsTcDI6YQaIKgY1FjH1M"}},{"name":"Constantino Yeng","id":"3nOJ0MZlqRyJRocFn7n7Cr","uri":"spotify:artist:3nOJ0MZlqRyJRocFn7n7Cr","href":"https://api.spotify.com/v1/artists/3nOJ0MZlqRyJRocFn7n7Cr","external_urls":{"spotify":"https://open.spotify.com/artist/3nOJ0MZlqRyJRocFn7n7Cr"}}],"available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BD","BE","BF","BG","BH","BI","BJ","BN","BO","BR","BS","BT","BW","BY","BZ","CD","CG","CH","CI","CL","CM","CO","CR","CV","CW","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GE","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HT","HU","ID","IE","IL","IN","IQ","IS","IT","JM","JO","JP","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","LY","MA","MC","MD","ME","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RS","RU","RW","SA","SB","SC","SE","SG","SI","SK","SL","SM","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","US","UY","UZ","VC","VE","VN","VU","WS","XK","ZA","ZM","ZW"],"disc_number":1,"duration_ms":292680,"explicit":false,"external_urls":{"spotify":"https://open.spotify.com/track/79tpIg39GSFRQcmrvyYedW"},"href":"https://api.spotify.com/v1/tracks/79tpIg39GSFRQcmrvyYedW","id":"79tpIg39GSFRQcmrvyYedW","name":"Kung Maibabalik (with Constantino Yeng)","preview_url":"https://p.scdn.co/mp3-preview/9112305c59befc6f1d1ba3dafc44bab7e1ce92ed?cid=e44e7b8278114c7db211c00ea273ac69","track_number":4,"uri":"spotify:track:79tpIg39GSFRQcmrvyYedW"}}}"#,
            r#"{"status":"success","result":{"artist":"Lord Huron","title":"The Night We Met","album":"13 Reasons Why (A Netflix Original Series Soundtrack)","release_date":"2015-02-09","label":"IAMSOUND","timecode":"00:06","song_link":"https://lis.tn/TheNightWeMet","spotify":{"album":{"name":"Strange Trails","artists":[{"name":"Lord Huron","id":"6ltzsmQQbmdoHHbLZ4ZN25","uri":"spotify:artist:6ltzsmQQbmdoHHbLZ4ZN25","href":"https://api.spotify.com/v1/artists/6ltzsmQQbmdoHHbLZ4ZN25","external_urls":{"spotify":"https://open.spotify.com/artist/6ltzsmQQbmdoHHbLZ4ZN25"}}],"album_group":"","album_type":"album","id":"4sD1qg4jwTZR4mvR4Iflk5","uri":"spotify:album:4sD1qg4jwTZR4mvR4Iflk5","available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BD","BE","BF","BG","BH","BI","BJ","BN","BO","BR","BS","BT","BW","BY","BZ","CD","CG","CH","CI","CL","CM","CO","CR","CV","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GE","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HT","HU","ID","IE","IL","IN","IQ","IS","IT","JM","JO","JP","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","LY","MA","MC","MD","ME","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RS","RU","RW","SA","SB","SC","SE","SG","SI","SK","SL","SM","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","UY","UZ","VC","VE","VN","VU","WS","XK","ZA","ZM","ZW"],"href":"https://api.spotify.com/v1/albums/4sD1qg4jwTZR4mvR4Iflk5","images":[{"height":640,"width":640,"url":"https://i.scdn.co/image/ab67616d0000b2731fa318e90c9d4ddc6b480853"},{"height":300,"width":300,"url":"https://i.scdn.co/image/ab67616d00001e021fa318e90c9d4ddc6b480853"},{"height":64,"width":64,"url":"https://i.scdn.co/image/ab67616d000048511fa318e90c9d4ddc6b480853"}],"external_urls":{"spotify":"https://open.spotify.com/album/4sD1qg4jwTZR4mvR4Iflk5"},"release_date":"2015-04-06","release_date_precision":"day"},"external_ids":{"isrc":"US53Q1200103"},"popularity":86,"is_playable":null,"linked_from":null,"artists":[{"name":"Lord Huron","id":"6ltzsmQQbmdoHHbLZ4ZN25","uri":"spotify:artist:6ltzsmQQbmdoHHbLZ4ZN25","href":"https://api.spotify.com/v1/artists/6ltzsmQQbmdoHHbLZ4ZN25","external_urls":{"spotify":"https://open.spotify.com/artist/6ltzsmQQbmdoHHbLZ4ZN25"}}],"available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BD","BE","BF","BG","BH","BI","BJ","BN","BO","BR","BS","BT","BW","BY","BZ","CD","CG","CH","CI","CL","CM","CO","CR","CV","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GE","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HT","HU","ID","IE","IL","IN","IQ","IS","IT","JM","JO","JP","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","LY","MA","MC","MD","ME","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RS","RU","RW","SA","SB","SC","SE","SG","SI","SK","SL","SM","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","UY","UZ","VC","VE","VN","VU","WS","XK","ZA","ZM","ZW"],"disc_number":1,"duration_ms":208211,"explicit":false,"external_urls":{"spotify":"https://open.spotify.com/track/3hRV0jL3vUpRrcy398teAU"},"href":"https://api.spotify.com/v1/tracks/3hRV0jL3vUpRrcy398teAU","id":"3hRV0jL3vUpRrcy398teAU","name":"The Night We Met","preview_url":"https://p.scdn.co/mp3-preview/a16d7e2d837ed06f2a6a852ad694d39833deeb3e?cid=e44e7b8278114c7db211c00ea273ac69","track_number":14,"uri":"spotify:track:3hRV0jL3vUpRrcy398teAU"}}}"#,
            r#"{"status":"success","result":{"artist":"A S T R O","title":"Ten Days","album":"Summer Memories","release_date":"2021-07-30","label":"Retro Jungle","timecode":"00:46","song_link":"https://lis.tn/RlNRoN","spotify":{"album":{"name":"Ten Days","artists":[{"name":"A S T R O","id":"35iqidtNbi2CzifmDD4URB","uri":"spotify:artist:35iqidtNbi2CzifmDD4URB","href":"https://api.spotify.com/v1/artists/35iqidtNbi2CzifmDD4URB","external_urls":{"spotify":"https://open.spotify.com/artist/35iqidtNbi2CzifmDD4URB"}},{"name":"WYS","id":"2CiO7xWdwPMDlVwlt9qa1f","uri":"spotify:artist:2CiO7xWdwPMDlVwlt9qa1f","href":"https://api.spotify.com/v1/artists/2CiO7xWdwPMDlVwlt9qa1f","external_urls":{"spotify":"https://open.spotify.com/artist/2CiO7xWdwPMDlVwlt9qa1f"}}],"album_group":"","album_type":"single","id":"5SHjnvutpL73uNTFpF6kgn","uri":"spotify:album:5SHjnvutpL73uNTFpF6kgn","available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BD","BE","BF","BG","BH","BI","BJ","BN","BO","BR","BS","BT","BW","BY","BZ","CA","CD","CG","CH","CI","CL","CM","CO","CR","CV","CW","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GE","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HT","HU","ID","IE","IL","IN","IQ","IS","IT","JM","JO","JP","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","LY","MA","MC","MD","ME","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RS","RU","RW","SA","SB","SC","SE","SG","SI","SK","SL","SM","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","US","UY","UZ","VC","VE","VN","VU","WS","XK","ZA","ZM","ZW"],"href":"https://api.spotify.com/v1/albums/5SHjnvutpL73uNTFpF6kgn","images":[{"height":640,"width":640,"url":"https://i.scdn.co/image/ab67616d0000b273e952f7d612f2b2b6486a6922"},{"height":300,"width":300,"url":"https://i.scdn.co/image/ab67616d00001e02e952f7d612f2b2b6486a6922"},{"height":64,"width":64,"url":"https://i.scdn.co/image/ab67616d00004851e952f7d612f2b2b6486a6922"}],"external_urls":{"spotify":"https://open.spotify.com/album/5SHjnvutpL73uNTFpF6kgn"},"release_date":"2021-07-30","release_date_precision":"day"},"external_ids":{"isrc":"QZN882134982"},"popularity":37,"is_playable":null,"linked_from":null,"artists":[{"name":"A S T R O","id":"35iqidtNbi2CzifmDD4URB","uri":"spotify:artist:35iqidtNbi2CzifmDD4URB","href":"https://api.spotify.com/v1/artists/35iqidtNbi2CzifmDD4URB","external_urls":{"spotify":"https://open.spotify.com/artist/35iqidtNbi2CzifmDD4URB"}},{"name":"WYS","id":"2CiO7xWdwPMDlVwlt9qa1f","uri":"spotify:artist:2CiO7xWdwPMDlVwlt9qa1f","href":"https://api.spotify.com/v1/artists/2CiO7xWdwPMDlVwlt9qa1f","external_urls":{"spotify":"https://open.spotify.com/artist/2CiO7xWdwPMDlVwlt9qa1f"}}],"available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BD","BE","BF","BG","BH","BI","BJ","BN","BO","BR","BS","BT","BW","BY","BZ","CA","CD","CG","CH","CI","CL","CM","CO","CR","CV","CW","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GE","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HT","HU","ID","IE","IL","IN","IQ","IS","IT","JM","JO","JP","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","LY","MA","MC","MD","ME","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RS","RU","RW","SA","SB","SC","SE","SG","SI","SK","SL","SM","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","US","UY","UZ","VC","VE","VN","VU","WS","XK","ZA","ZM","ZW"],"disc_number":1,"duration_ms":157307,"explicit":false,"external_urls":{"spotify":"https://open.spotify.com/track/6MGkXJw5R37YzKdosDHFJX"},"href":"https://api.spotify.com/v1/tracks/6MGkXJw5R37YzKdosDHFJX","id":"6MGkXJw5R37YzKdosDHFJX","name":"Ten Days","preview_url":"https://p.scdn.co/mp3-preview/8801fc62c6130531e928994022e72cfd8690f1ac?cid=e44e7b8278114c7db211c00ea273ac69","track_number":1,"uri":"spotify:track:6MGkXJw5R37YzKdosDHFJX"}}}"#,
            r#"{"status":"success","result":{"artist":"Stephen Speaks","title":"Passenger Seat","album":"Passenger Seat","release_date":"2011-12-13","label":"BELIEVE - Rippley Records","timecode":"01:14","song_link":"https://lis.tn/yihsxI","apple_music":{"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview126/v4/fd/54/6e/fd546e0f-99c5-dda5-ce47-64290b328847/mzaf_2247212798847815130.plus.aac.p.m4a"}],"artwork":{"width":1440,"height":1440,"url":"https://is3-ssl.mzstatic.com/image/thumb/Music126/v4/56/8b/77/568b775c-ce50-10f8-5463-32a53d81b32b/cover.jpg/{w}x{h}bb.jpg","bgColor":"050304","textColor1":"eed0a1","textColor2":"e3bb91","textColor3":"bfa782","textColor4":"b69675"},"artistName":"Stephen Speaks","url":"https://music.apple.com/us/album/passenger-seat/1604869472?app=music\u0026at=1000l33QU\u0026i=1604869987\u0026mt=1","discNumber":1,"genreNames":["Pop","Music"],"durationInMillis":274245,"releaseDate":"2011-12-13","name":"Passenger Seat","isrc":"TCABC1173267","albumName":"Passenger Seat - Single","playParams":{"id":"1604869987","kind":"song"},"trackNumber":1,"composerName":"Rockwell Ripperger \u0026 TJ McCloud"},"spotify":{"album":{"name":"Passenger Seat","artists":[{"name":"Stephen Speaks","id":"0AvtzKTYleNqdJXoZasQWG","uri":"spotify:artist:0AvtzKTYleNqdJXoZasQWG","href":"https://api.spotify.com/v1/artists/0AvtzKTYleNqdJXoZasQWG","external_urls":{"spotify":"https://open.spotify.com/artist/0AvtzKTYleNqdJXoZasQWG"}}],"album_group":"","album_type":"single","id":"6kfmY3SwNpSu8XrNeOVLd5","uri":"spotify:album:6kfmY3SwNpSu8XrNeOVLd5","available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BD","BE","BF","BG","BH","BJ","BN","BO","BR","BS","BT","BW","BZ","CA","CG","CH","CI","CL","CM","CO","CR","CV","CW","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GE","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HT","HU","ID","IE","IL","IN","IS","IT","JM","JO","JP","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","MA","MC","MD","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RU","RW","SA","SB","SC","SE","SG","SI","SK","SL","SM","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","US","UY","UZ","VC","VE","VN","VU","WS","ZA","ZM"],"href":"https://api.spotify.com/v1/albums/6kfmY3SwNpSu8XrNeOVLd5","images":[{"height":640,"width":640,"url":"https://i.scdn.co/image/ab67616d0000b2731635b19999f7ad0d6eddc3c1"},{"height":300,"width":300,"url":"https://i.scdn.co/image/ab67616d00001e021635b19999f7ad0d6eddc3c1"},{"height":64,"width":64,"url":"https://i.scdn.co/image/ab67616d000048511635b19999f7ad0d6eddc3c1"}],"external_urls":{"spotify":"https://open.spotify.com/album/6kfmY3SwNpSu8XrNeOVLd5"},"release_date":"2011-12-13","release_date_precision":"day"},"external_ids":{"isrc":"TCABC1173267"},"popularity":68,"is_playable":null,"linked_from":null,"artists":[{"name":"Stephen Speaks","id":"0AvtzKTYleNqdJXoZasQWG","uri":"spotify:artist:0AvtzKTYleNqdJXoZasQWG","href":"https://api.spotify.com/v1/artists/0AvtzKTYleNqdJXoZasQWG","external_urls":{"spotify":"https://open.spotify.com/artist/0AvtzKTYleNqdJXoZasQWG"}}],"available_markets":["AD","AE","AG","AL","AM","AO","AR","AT","AU","AZ","BA","BB","BD","BE","BF","BG","BH","BJ","BN","BO","BR","BS","BT","BW","BZ","CA","CG","CH","CI","CL","CM","CO","CR","CV","CW","CY","CZ","DE","DJ","DK","DM","DO","DZ","EC","EE","EG","ES","FI","FJ","FM","FR","GA","GB","GD","GE","GH","GM","GN","GQ","GR","GT","GW","GY","HK","HN","HR","HT","HU","ID","IE","IL","IN","IS","IT","JM","JO","JP","KE","KG","KH","KI","KM","KN","KR","KW","KZ","LA","LB","LC","LI","LK","LR","LS","LT","LU","LV","MA","MC","MD","MG","MH","MK","ML","MN","MO","MR","MT","MU","MV","MW","MX","MY","MZ","NA","NE","NG","NI","NL","NO","NP","NR","NZ","OM","PA","PE","PG","PH","PK","PL","PS","PT","PW","PY","QA","RO","RU","RW","SA","SB","SC","SE","SG","SI","SK","SL","SM","SN","SR","ST","SV","SZ","TD","TG","TH","TJ","TL","TN","TO","TR","TT","TV","TW","TZ","UA","UG","US","UY","UZ","VC","VE","VN","VU","WS","ZA","ZM"],"disc_number":1,"duration_ms":274245,"explicit":false,"external_urls":{"spotify":"https://open.spotify.com/track/5ZpSUdy5wL5Hvwsqz6JCAt"},"href":"https://api.spotify.com/v1/tracks/5ZpSUdy5wL5Hvwsqz6JCAt","id":"5ZpSUdy5wL5Hvwsqz6JCAt","name":"Passenger Seat","preview_url":"https://p.scdn.co/mp3-preview/172a1eabdf82bb5c932a3f59154975cdce850765?cid=e44e7b8278114c7db211c00ea273ac69","track_number":1,"uri":"spotify:track:5ZpSUdy5wL5Hvwsqz6JCAt"}}}"#,
        ];

        let random_response = raw_responses
            .choose(&mut rand::thread_rng())
            .expect("Failed to get choose random from raw responses");

        log::debug!("random_response: {}", random_response);

        Ok(Response::parse(random_response.as_bytes())?.data()?)
    }
}

#[async_trait(?Send)]
impl Provider for AudDMock {
    async fn recognize(&self, _: &AudioRecording) -> Result<Song, ProviderError> {
        glib::timeout_future(Duration::from_secs(1)).await;
        Ok(AudD::handle_data(
            self.random_data().map_err(ProviderError::AudD)?,
        ))
    }

    fn listen_duration(&self) -> Duration {
        Duration::from_secs(1)
    }
}
